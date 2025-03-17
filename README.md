# pyrxing

**pyrxing** is a Python barcode reader library built using [rxing](https://github.com/rxing-core/rxing) (a Rust port of ZXing) and [PyO3](https://github.com/PyO3/pyo3). This library provides a simple and efficient barcode reading solution without requiring additional system dependencies.

## Motivation

While working, I used `pyzbar`, but it required installing the `zbar` library on the host system, and there were issues with reading certain QR codes. Searching for alternatives, I discovered `rxing`, a Rust port of ZXing, and decided to create a Python extension module using `PyO3`.

## Features

- **Easy Installation**: No special setup is needed on the host system; simply install using `pip`.
- **Minimal API**: Provides only two public functions, `read_barcode` and `read_barcodes`, for simplicity.
- **Type Hinting Support**: Includes `.pyi` files for type checking and IDE autocompletion.

## Installation

Install the library using `pip`:

```bash
pip install pyrxing
```

*Note: Wheel support for `musl` is under development.*

## Usage

Here are examples of how to use the library to read barcodes:

```python
from pyrxing import read_barcode, read_barcodes

# Read a single barcode
barcode = read_barcode(image_path)

# Read multiple barcodes
barcodes = read_barcodes(image_path)

# Read multiple barcodes that matches specified formats
barcodes = read_barcodes(image_path, formats=['QR_CODE'])

```

## API Reference

Below is the content of the `pyrxing.pyi` file, providing type hints and definitions:

```python
from typing import Literal, Protocol

# see https://docs.rs/rxing/0.7.1/rxing/enum.BarcodeFormat.html
type BarcodeFormat = Literal[
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
        """Return pixel data as a byte array."""

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

def read_barcode(image: str | ImageProtocol, *, formats: list[BarcodeFormat] | None = None) -> DecodeResult | None: ...

def read_barcodes(image: str | ImageProtocol, *, formats: list[BarcodeFormat] | None = None) -> list[DecodeResult]: ...
```

## Planned Features
- **Expanded Platform Support**: This project currently provides only ARM-based macOS wheels and plans to create wheels compliant with manylinux and musllinux standards in the future..

## Features Not Planned
- **Barcode Generation**: The library will not include barcode generation functionality, as it focuses solely on barcode decoding.
- **Windows Support**: Currently, there are no plans to support Windows operating systems.

## rxing Track
Currently tracking rxing 0.7.1

## Copyright notes
The original license / copyright remains with the rxing developers.
