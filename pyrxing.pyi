import os
from typing import Any, Literal, Protocol


BarcodeFormat = Literal[
    "Aztec",
    "AztecCode",
    "AztecRune",
    "Codabar",
    "Code39",
    "Code93",
    "Code128",
    "CompactPDF417",
    "DataBar",
    "DataBarExpanded",
    "DataBarExpandedStacked",
    "DataBarLimited",
    "DataBarOmni",
    "DataBarStacked",
    "DataBarStackedOmni",
    "DataMatrix",
    "DXFilmEdge",
    "EAN2",
    "EAN5",
    "EAN8",
    "EAN13",
    "EANUPC",
    "ISBN",
    "ITF",
    "MaxiCode",
    "MicroPDF417",
    "MicroQRCode",
    "PDF417",
    "PZN",
    "QRCode",
    "QRCodeModel1",
    "QRCodeModel2",
    "RMQRCode",
    "UPCA",
    "UPCE",
]


class ImageProtocol(Protocol):
    @property
    def mode(self) -> str: ...

    @property
    def width(self) -> int: ...

    @property
    def height(self) -> int: ...

    def tobytes(self) -> bytes:
        """return pixel data as byte array"""

    def convert(self, mode: str) -> Any: ...

    def load(self): ...


class ArrayProtocol(Protocol):
    @property
    def __array_interface__(self) -> dict[str, Any]: ...

    def tobytes(self) -> bytes: ...


class Error(Exception):
    """Base class of every exception the module raises."""


class BarcodeDecodeError(Error):
    """The reader backend failed to decode the image."""


class ImageError(Error):
    """The image cannot be used: unreadable, of an unsupported mode, or malformed."""


class UnsupportedFormatError(Error):
    """A requested barcode format name is not supported."""

    @property
    def format(self) -> str:
        """The rejected barcode format name."""


class Point:
    @property
    def x(self) -> int: ...

    @property
    def y(self) -> int: ...

class DecodeResult:
    @property
    def text(self) -> str: ...

    @property
    def points(self) -> list[Point]: ...

    @property
    def format(self) -> str: ...


def read_barcode(image: str | os.PathLike[str] | ImageProtocol | ArrayProtocol, *, formats: list[BarcodeFormat] | None = None) -> DecodeResult | None: ...
def read_barcodes(image: str | os.PathLike[str] | ImageProtocol | ArrayProtocol, *, formats: list[BarcodeFormat] | None = None) -> list[DecodeResult]: ...
