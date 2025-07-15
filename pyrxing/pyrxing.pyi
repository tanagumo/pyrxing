from typing import Any, Literal, Protocol


BarcodeFormat = Literal[
    "Aztec",
    "Codabar",
    "Code39",
    "Code93",
    "Code128",
    "DataBar",
    "DataBarExpanded",
    "DataBarLimited",
    "DataMatrix",
    "EAN8",
    "EAN13",
    "ITF",
    "MaxiCode",
    "PDF417",
    "QRCode",
    "UPCA",
    "UPCE",
    "MicroQRCode",
    "RMQRCode",
    "DXFilmEdge",
]


class ImageProtocol(Protocol):
    @property
    def width(self) -> int: ...

    @property
    def height(self) -> int: ...

    def tobytes(self) -> bytes:
        """return pixel data as byte array"""

    def convert(self, mode: str) -> Any: ...

    def load(self): ...


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


def read_barcode(image: str | ImageProtocol, *, formats: list[BarcodeFormat] | None = None) -> DecodeResult | None: ...
def read_barcodes(image: str | ImageProtocol, *, formats: list[BarcodeFormat] | None = None) -> list[DecodeResult]: ...
