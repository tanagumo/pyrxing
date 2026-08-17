import ctypes
import pathlib
import re
from typing import Self

import cv2
import numpy as np
import pytest
from PIL import Image

import pyrxing


ASSETS: dict[str, dict[str, str]] = {
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

def test_read() -> None:
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


@pytest.mark.parametrize(("path", "expected"), ASSETS.items())
@pytest.mark.parametrize(
    "read_mode",
    [
        pytest.param(cv2.IMREAD_GRAYSCALE, id="grayscale"),
        pytest.param(cv2.IMREAD_COLOR, id="bgr"),
        pytest.param(cv2.IMREAD_UNCHANGED, id="unchanged"),
    ],
)
def test_read_opencv_numpy_arrays(
    path: str, expected: dict[str, str], read_mode: int
) -> None:
    image = cv2.imread(path, read_mode)
    assert image is not None
    assert image.dtype == np.uint8
    assert image.ndim == 2 or image.shape[2] in (3, 4)

    result = pyrxing.read_barcode(image)
    assert result is not None
    assert result.text == expected["value"]
    assert result.format == expected["format"]

    results = pyrxing.read_barcodes(image)
    assert len(results) == 1
    assert results[0].text == expected["value"]
    assert results[0].format == expected["format"]


@pytest.mark.parametrize(
    ("image", "message"),
    [
        (np.zeros((2, 2), dtype=np.float32), "array dtype must be uint8"),
        (np.zeros((2, 2, 2), dtype=np.uint8), "array channel count"),
        (np.zeros((2,), dtype=np.uint8), "array shape must be"),
        (np.zeros((1, 1, 1, 1), dtype=np.uint8), "array shape must be"),
        (np.zeros((0, 2), dtype=np.uint8), "greater than zero"),
    ],
)
def test_rejects_unsupported_numpy_arrays(image: np.ndarray, message: str) -> None:
    with pytest.raises(ValueError, match=message):
        pyrxing.read_barcode(image)


class InvalidArrayData:
    __array_interface__ = {
        "shape": (2, 2),
        "typestr": "|u1",
        "version": 3,
    }

    def tobytes(self) -> bytes:
        return b"\x00" * 3


def test_rejects_mismatched_array_data() -> None:
    with pytest.raises(ValueError, match="expected 4, got 3"):
        pyrxing.read_barcode(InvalidArrayData())


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


# Subformat names of pyrxing.pyi none of which a QR code carries.
NAMES_MISSING_FROM_A_QR_CODE: list[str] = [
    "AztecCode",
    "AztecRune",
    "CompactPDF417",
    "EAN2",
    "EAN5",
    "EANUPC",
    "ISBN",
    "MicroPDF417",
    "PZN",
    "QRCodeModel1",
]


def test_read_barcode_filters_by_a_subformat_name() -> None:
    # A name the extension drops leaves `formats` empty, and an empty list means
    # every format, so the QR code below would decode instead of being filtered out.
    image = Image.open(QR_CODE)

    matched = [
        name
        for name in NAMES_MISSING_FROM_A_QR_CODE
        if pyrxing.read_barcode(image, formats=[name]) is not None
    ]

    assert matched == []
    assert pyrxing.read_barcode(image, formats=["QRCodeModel2"]).text == QR_CODE_VALUE


def stub_image_protocol_members() -> set[str]:
    """Returns the member names the ImageProtocol of pyrxing.pyi declares."""
    stub = pathlib.Path(__file__).with_name("pyrxing.pyi").read_text()
    body = stub.split("class ImageProtocol(Protocol):", 1)[1].split("\nclass ", 1)[0]
    return set(re.findall(r"def ([a-z_]+)\(", body))


class StubConformantImage:
    """An ImageProtocol implementation with exactly the members pyrxing.pyi declares."""

    def __init__(self, pil: Image.Image) -> None:
        self._pil = pil.convert("L")

    @property
    def mode(self) -> str:
        return self._pil.mode

    @property
    def width(self) -> int:
        return self._pil.width

    @property
    def height(self) -> int:
        return self._pil.height

    def tobytes(self) -> bytes:
        return self._pil.tobytes()

    def convert(self, mode: str) -> Image.Image:
        return self._pil.convert(mode)

    def load(self) -> None:
        self._pil.load()


def test_stub_declares_the_members_the_extension_requires() -> None:
    assert stub_image_protocol_members() == {
        "mode",
        "width",
        "height",
        "tobytes",
        "convert",
        "load",
    }

    image = StubConformantImage(Image.open(QR_CODE))

    assert pyrxing.read_barcode(image).text == QR_CODE_VALUE


def stub_barcode_format_names() -> list[str]:
    """Returns the names the BarcodeFormat Literal of pyrxing.pyi declares."""
    stub = pathlib.Path(__file__).with_name("pyrxing.pyi").read_text()
    literal = stub.split("BarcodeFormat = Literal[", 1)[1].split("]", 1)[0]
    return re.findall(r'"([A-Za-z0-9]+)"', literal)


def test_stub_declares_only_names_the_extension_accepts() -> None:
    image = Image.open(QR_CODE)
    names = stub_barcode_format_names()
    assert len(names) == 35

    for name in names:
        pyrxing.read_barcode(image, formats=[name])
        pyrxing.read_barcodes(image, formats=[name])


@pytest.mark.parametrize(
    ("formats", "reported"),
    [
        (["Bogus"], "Bogus"),
        (["QRCode", "Bogus"], "Bogus"),
        (["Bogus", "AlsoBogus"], "Bogus"),
        (["qrcode"], "qrcode"),
    ],
)
def test_read_barcode_rejects_an_unknown_format_name(
    formats: list[str], reported: str
) -> None:
    image = Image.open(QR_CODE)

    with pytest.raises(pyrxing.UnsupportedFormatError) as caught:
        pyrxing.read_barcode(image, formats=formats)

    assert caught.value.format == reported
    assert reported in str(caught.value)


def test_unsupported_format_error_derives_from_the_module_base() -> None:
    assert issubclass(pyrxing.UnsupportedFormatError, pyrxing.Error)
    assert not issubclass(pyrxing.UnsupportedFormatError, pyrxing.ImageError)


@pytest.mark.parametrize("formats", [None, []])
def test_read_barcode_searches_every_format_without_a_filter(
    formats: list[str] | None,
) -> None:
    image = Image.open(QR_CODE)

    assert pyrxing.read_barcode(image, formats=formats).text == QR_CODE_VALUE


def test_read_barcode_accepts_more_formats_than_the_zxing_format_count() -> None:
    image = Image.open(QR_CODE)

    assert pyrxing.read_barcode(image, formats=["QRCode"] * 33).text == QR_CODE_VALUE


def test_read_barcode_rejects_a_size_disagreeing_with_the_pixel_data() -> None:
    image = Image.open(QR_CODE).convert("L")
    truncated = SizedImage(image.width, image.height, image.tobytes()[:-1])

    with pytest.raises(pyrxing.ImageError, match="expected 122500, got 122499"):
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
    with pytest.raises(pyrxing.ImageError, match="expected 4294967296, got 0"):
        pyrxing.read_barcode(SizedImage(0x80000000, 2, b""))


def test_module_exceptions_derive_from_the_module_base() -> None:
    assert issubclass(pyrxing.ImageError, pyrxing.Error)
    assert issubclass(pyrxing.BarcodeDecodeError, pyrxing.Error)
    assert issubclass(pyrxing.Error, Exception)


def test_image_error_carries_its_message() -> None:
    image = Image.open(QR_CODE).convert("CMYK")

    with pytest.raises(pyrxing.Error) as caught:
        pyrxing.read_barcode(image)

    assert isinstance(caught.value, pyrxing.ImageError)
    assert "unsupported mode(CMYK)" in str(caught.value)
    assert not hasattr(caught.value, "message")


@pytest.mark.parametrize("factory", [str, pathlib.Path])
def test_read_barcode_accepts_a_path_like_argument(factory) -> None:
    assert pyrxing.read_barcode(factory(QR_CODE)).text == QR_CODE_VALUE


def test_read_barcode_accepts_an_fspath_implementation() -> None:
    class PathHolder:
        def __fspath__(self) -> str:
            return QR_CODE

    assert pyrxing.read_barcode(PathHolder()).text == QR_CODE_VALUE


@pytest.mark.parametrize(
    "image", [123, None, b"assets/test_qr_code.png", object(), ["assets/test_qr_code.png"]]
)
def test_read_barcode_rejects_an_argument_of_another_type(image: object) -> None:
    with pytest.raises(TypeError, match="str, os.PathLike or ImageProtocol"):
        pyrxing.read_barcode(image)


def test_read_barcode_rejects_an_incomplete_image_protocol() -> None:
    class WithoutMode:
        width = 4
        height = 3

        def tobytes(self) -> bytes:
            return bytes(12)

        def convert(self, mode: str) -> Self:
            return self

        def load(self) -> None:
            pass

    with pytest.raises(TypeError):
        pyrxing.read_barcode(WithoutMode())


def test_read_barcode_rejects_a_dimension_beyond_the_reader_limit() -> None:
    image = SizedImage(0x10000, 1, bytes(0x10000))

    with pytest.raises(pyrxing.ImageError, match="65535"):
        pyrxing.read_barcode(image)


def stub_exception_bases() -> dict[str, str]:
    """Returns the base class each exception of pyrxing.pyi declares."""
    stub = pathlib.Path(__file__).with_name("pyrxing.pyi").read_text()
    return dict(re.findall(r"^class (Error|\w+Error)\((\w+)\):", stub, re.MULTILINE))


def test_stub_declares_the_exception_hierarchy() -> None:
    declared = stub_exception_bases()
    assert declared == {
        "Error": "Exception",
        "BarcodeDecodeError": "Error",
        "ImageError": "Error",
        "UnsupportedFormatError": "Error",
    }

    for name, base in declared.items():
        actual = getattr(pyrxing, name)
        expected = Exception if base == "Exception" else getattr(pyrxing, base)
        assert actual.__bases__ == (expected,), name


def test_readme_repeats_the_type_stub() -> None:
    def api_block(path: pathlib.Path) -> str:
        text = path.read_text()
        return text.split("class ImageProtocol(Protocol):", 1)[1].split("def read_barcode", 1)[0]

    here = pathlib.Path(__file__).parent

    assert api_block(here / "README.md") == api_block(here / "pyrxing.pyi")
