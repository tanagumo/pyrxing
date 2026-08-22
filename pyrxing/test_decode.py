import ctypes
from typing import Self

import pytest
from PIL import Image

import pyrxing


ASSETS = {
    "assets/test_aztec.png": {"format": "MicroQRCode", "value": "Code Code Data"},
    "assets/test_codabar.png": {"format": "Codabar", "value": "A-12656-D"},
    "assets/test_code128.png": {
        "format": "Code128",
        "value": "Python Barcode Random World",
    },
    "assets/test_code39.png": {"format": "Code39", "value": "QD9MZ5O"},
    "assets/test_code93.png": {"format": "Code93", "value": "ABC-1234-/+"},
    "assets/test_data_bar.png": {
        "format": "DataBarOmni",
        "value": "(01)01234567890128",
    },
    "assets/test_data_bar_expanded.png": {
        "format": "DataBarExpanded",
        "value": "011234567890123-ABCabc",
    },
    "assets/test_data_bar_limited.png": {
        "format": "DataBarLimited",
        "value": "(01)01234567890128",
    },
    "assets/test_data_matrix.png": {
        "format": "DataMatrix",
        "value": "DataMatrix_31802",
    },
    "assets/test_dx_film_edge.png": {
        "format": "Code128",
        "value": "DXFilmEdge_6086",
    },
    "assets/test_ean13.png": {"format": "EAN13", "value": "2708639496369"},
    "assets/test_ean8.png": {"format": "EAN8", "value": "07727931"},
    "assets/test_itf.png": {"format": "ITF", "value": "4577279804"},
    "assets/test_maxi_code.png": {
        "format": "MaxiCode",
        "value": "MaxiCode_46753",
    },
    "assets/test_micro_qr.png": {
        "format": "MicroQRCode",
        "value": "0ZFXY9",
    },
    "assets/test_pdf417.png": {"format": "PDF417", "value": "PDF417_12417"},
    "assets/test_qr_code.png": {
        "format": "QRCode",
        "value": "https://demo.net/demo/7809",
    },
    "assets/test_rmqr.png": {"format": "MicroQRCode", "value": "V4GQ68D2"},
    "assets/test_upc_a.png": {"format": "EAN13", "value": "0041935974561"},
    "assets/test_upc_e.png": {"format": "UPCE", "value": "0182749000074"},
}

def test_read():
    for path, v in ASSETS.items():
        i = Image.open(path)
        res = pyrxing.read_barcode(i)
        assert res is not None
        assert res.text == v['value']
        assert res.format == v['format']

        res = pyrxing.read_barcodes(i)
        assert len(res) == 1
        assert res[0].text == v['value']
        assert res[0].format == v['format']


QR_CODE: str = "assets/test_qr_code.png"
QR_CODE_VALUE: str = "https://demo.net/demo/7809"


class SizedImage:
    """An ImageProtocol implementation reporting a size of its own choosing."""

    mode: str = "L"

    def __init__(self, width: int, height: int, data: bytes) -> None:
        self.width = width
        self.height = height
        self._data = data

    def tobytes(self) -> bytes:
        return self._data

    def convert(self, mode: str) -> Self:
        return self

    def load(self) -> None:
        pass


def test_read_barcode_accepts_more_formats_than_the_zxing_format_count() -> None:
    image = Image.open(QR_CODE)

    assert pyrxing.read_barcode(image, formats=["QRCode"] * 33).text == QR_CODE_VALUE


def test_read_barcode_rejects_a_size_disagreeing_with_the_pixel_data() -> None:
    image = Image.open(QR_CODE).convert("L")
    truncated = SizedImage(image.width, image.height, image.tobytes()[:-1])

    with pytest.raises(ValueError, match="expected 122500, got 122499"):
        pyrxing.read_barcode(truncated)


class PointerInterfaceImage:
    """An ImageProtocol implementation exposing the NumPy form of __array_interface__.

    NumPy reports `data` as a (address, read-only) pair rather than a bytes object.
    """

    mode: str = "L"

    def __init__(self, pil: Image.Image) -> None:
        self._data = pil.convert("L").tobytes()
        self.width = pil.width
        self.height = pil.height

    def tobytes(self) -> bytes | bytearray:
        return self._data

    def convert(self, mode: str) -> Self:
        return self

    def load(self) -> None:
        pass

    @property
    def __array_interface__(self) -> dict[str, object]:
        address = ctypes.cast(ctypes.c_char_p(self._data), ctypes.c_void_p).value
        return {
            "version": 3,
            "typestr": "|u1",
            "shape": (self.height, self.width),
            "data": (address, True),
        }


class BytearrayImage(PointerInterfaceImage):
    """An ImageProtocol implementation returning a buffer other than bytes."""

    def tobytes(self) -> bytearray:
        return bytearray(self._data)


def test_read_barcode_accepts_a_pointer_shaped_array_interface() -> None:
    image = PointerInterfaceImage(Image.open(QR_CODE))

    assert pyrxing.read_barcode(image).text == QR_CODE_VALUE


def test_read_barcode_accepts_a_buffer_other_than_bytes() -> None:
    image = BytearrayImage(Image.open(QR_CODE))

    assert pyrxing.read_barcode(image).text == QR_CODE_VALUE


def test_read_barcode_rejects_a_pixel_count_beyond_u32() -> None:
    # 0x80000000 * 2 is 2**32, so a wrapping multiplication would report zero pixels
    # and let an empty buffer through.
    with pytest.raises(ValueError, match="expected 4294967296, got 0"):
        pyrxing.read_barcode(SizedImage(0x80000000, 2, b""))
