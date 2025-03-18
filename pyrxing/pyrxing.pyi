from typing import Literal, Protocol, Union


BarcodeFormat = Literal[
    'AZTEC',
    'CODABAR',
    'CODE_39',
    'CODE_93',
    'CODE_128',
    'DATA_MATRIX',
    'EAN_8',
    'EAN_13',
    'ITF',
    'MAXICODE',
    'PDF_417',
    'QR_CODE',
    'MICRO_QR_CODE',
    'RECTANGULAR_MICRO_QR_CODE',
    'RSS_14',
    'RSS_EXPANDED',
    'TELEPEN',
    'UPC_A',
    'UPC_E',
    'UPC_EAN_EXTENSION',
    'DX_FILM_EDGE',
]


class ImageProtocol(Protocol):
    @property
    def width(self) -> int: ...

    @property
    def height(self) -> int: ...

    def tobytes(self) -> bytes:
        """return pixel data as byte array"""

    def convert(self, mode: str): ...


class BarcodeDecodeError(Exception): ...

class ImageError(Exception): ...

class Point:
    @property
    def x(self) -> float: ...

    @property
    def y(self) -> float: ...

class DecodeResult:
    @property
    def text(self) -> str: ...

    @property
    def points(self) -> list[Point]: ...

    @property
    def format(self) -> str: ...


def read_barcode(image: Union[str, ImageProtocol], *, formats: list[BarcodeFormat] | None = None) -> DecodeResult | None: ...
def read_barcodes(image: Union[str, ImageProtocol], *, formats: list[BarcodeFormat] | None = None) -> list[DecodeResult]: ...
