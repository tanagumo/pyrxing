# pyrxing

**pyrxing** is a fast, dependency-free Python barcode/QR code reader built using [rxing](https://github.com/rxing-core/rxing) — a high-performance Rust port of ZXing — via [PyO3](https://github.com/PyO3/pyo3).

This library offers efficient barcode scanning in pure Python environments, with pre-built native wheels — including full support for **Alpine Linux** and `musl`-based systems.

---

## 🚀 Features

* ⚡ **Fast and lightweight**: Powered by Rust for high-speed barcode decoding
* 🐍 **Python-native API**: Simple interface with just two functions: `read_barcode` and `read_barcodes`
* 📦 **No system dependencies**: No need for zbar, JRE, or any external libraries
* 🏗 **Alpine Linux compatible**: Pre-built `musllinux` wheels available
* 🧠 **Type hinting & autocompletion**: Includes `.pyi` stub files
* 🔒 **Safe and minimal**: No unnecessary features — just barcode reading

---

## 📦 Installation

Install with pip:

```
pip install pyrxing
```

---

## 🧪 Usage

```python
from pyrxing import read_barcode, read_barcodes

# Read a single barcode from an image path
barcode = read_barcode("example.png")

# Read multiple barcodes from an image
barcodes = read_barcodes("example.png")

# Optionally filter by barcode format
barcodes = read_barcodes("example.png", formats=['QR_CODE'])
```

You can also pass a compatible image object instead of a path.

---

## ✅ Supported Environments

### Platforms

* **Linux** (manylinux & musllinux wheels)
  * Architectures: `x86_64`, `aarch64`, `armv7`
* **macOS**
  * Universal binaries for both Intel and Apple Silicon (arm64)
* **Windows**
  * Architectures: `x64`, `x86`

### Python Versions

Python 3.11+

---

## 🛠 Planned Features

* [ ] More platform wheels (expanding support for other OS/Python combinations)
* [ ] Additional barcode format configuration options

---

## 🚫 Not Planned

* ❌ Barcode generation

---

## 📚 API Reference

For full API and type hints, see `pyrxing.pyi` or use your IDE's autocomplete.

```python
from typing import Literal, Protocol, Union

# see https://docs.rs/rxing/0.7.1/rxing/enum.BarcodeFormat.html
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


def read_barcode(image: Union[str, ImageProtocol], *, formats: Union[list[BarcodeFormat], None] = None) -> Union[DecodeResult, None]: ...
def read_barcodes(image: Union[str, ImageProtocol], *, formats: Union[list[BarcodeFormat], None] = None) -> list[DecodeResult]: ...
```

---

## 📌 Notes

* Tracking `rxing` version: **0.7.1**
* Copyright for decoding logic belongs to the original [rxing](https://github.com/rxing-core/rxing) authors.
