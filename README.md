# pyrxing

**pyrxing** is a fast, dependency-free Python barcode/QR code reader for image paths, Pillow images, and NumPy/OpenCV arrays. It is built using [zxing-cpp](https://github.com/zxing-cpp/zxing-cpp) Rust bindings via [PyO3](https://github.com/PyO3/pyo3).

This library offers efficient barcode scanning in pure Python environments, with pre-built native wheels — including full support for **Alpine Linux** and `musl`-based systems where the official [zxing-cpp Python package](https://pypi.org/project/zxing-cpp/) requires additional build steps.

---

## 🚀 Features

* ⚡ **High performance**: Powered by zxing-cpp 0.5.3 C++ library through optimized Rust bindings with excellent barcode detection performance
* 🐍 **Python-native API**: Simple interface with just two functions: `read_barcode` and `read_barcodes`
* 🔢 **NumPy and OpenCV support**: Decode `uint8` image arrays directly, including non-contiguous views
* 📦 **No system dependencies**: No need for zbar, JRE, or any external libraries
* 🏗 **Alpine Linux compatible**: Pre-built `musllinux` wheels available (no build required)
* 🧠 **Type hinting & autocompletion**: Includes `.pyi` stub files with 35 barcode format variants
* 🔒 **Safe and minimal**: No unnecessary features — just barcode reading
* ⚡ **Competitive performance**: Matches or exceeds official zxing-cpp Python bindings across all formats

---

## ✅ Supported Environments

### Platforms

* **Linux** (manylinux & musllinux wheels)
  * Architectures: `x86_64`, `aarch64`
* **macOS**
  * Universal binaries for both Intel and Apple Silicon (arm64)
* **Windows**
  * Architectures: `x64`

### Python Versions

Python 3.11 - 3.14

---

## 📦 Installation

Install with pip:

```bash
pip install pyrxing
```

**Recommended for Alpine Linux/musl environments**: While the official zxing-cpp Python package requires building from source on musl-based systems, pyrxing provides pre-built wheels for immediate installation.

---

## 📊 Performance

pyrxing delivers competitive performance across a comprehensive range of barcode formats, matching or exceeding the official zxing-cpp Python bindings:

**Performance Summary:**
- **Single detection**: Comparable performance with zxing-cpp (between 2% slower and 28% faster)
- **Multiple detection**: Faster than zxing-cpp on every format (up to 11%)
- **Overall**: pyrxing leads on 39 of the 40 measurements

### Benchmark Results (μs per decode)

#### `read_barcode()` - Single Barcode Detection

| Format      | pyrxing | zxing-cpp | Difference |
|-------------|---------|-----------|------------|
| Micro QR    | 47.5    | 50.1      | **5% faster** |
| rMQR        | 50.7    | 54.3      | **7% faster** |
| Aztec       | 55.9    | 60.7      | **8% faster** |
| DataBar Ltd.| 190.9   | 194.3     | **2% faster** |
| QR Code     | 192.2   | 202.6     | **5% faster** |
| DataBar     | 197.9   | 200.8     | **1% faster** |
| Data Matrix | 198.0   | 273.1     | **28% faster** |
| Code 93     | 277.4   | 284.8     | **3% faster** |
| DX Film Edge| 305.0   | 337.3     | **10% faster** |
| PDF417      | 337.5   | 370.6     | **9% faster** |
| ITF         | 342.7   | 403.9     | **15% faster** |
| DataBar Exp.| 387.7   | 379.7     | 2% slower |
| Codabar     | 418.8   | 478.1     | **12% faster** |
| UPC-E       | 542.9   | 660.2     | **18% faster** |
| EAN-8       | 851.3   | 1033.1    | **18% faster** |
| MaxiCode    | 984.0   | 1109.4    | **11% faster** |
| Code 39     | 1052.8  | 1242.5    | **15% faster** |
| EAN-13      | 1108.1  | 1338.5    | **17% faster** |
| UPC-A       | 1118.4  | 1350.5    | **17% faster** |
| Code 128    | 2203.8  | 2591.3    | **15% faster** |

#### `read_barcodes()` - Multiple Barcode Detection

| Format      | pyrxing | zxing-cpp | Difference |
|-------------|---------|-----------|------------|
| Micro QR    | 298.1   | 299.6     | **1% faster** |
| rMQR        | 331.5   | 332.4     | **0% faster** |
| Aztec       | 390.7   | 402.4     | **3% faster** |
| DataBar Ltd.| 532.1   | 537.5     | **1% faster** |
| DataBar     | 544.4   | 548.9     | **1% faster** |
| Data Matrix | 666.0   | 746.5     | **11% faster** |
| PDF417      | 713.5   | 745.7     | **4% faster** |
| Code 93     | 767.4   | 777.0     | **1% faster** |
| ITF         | 826.2   | 892.8     | **7% faster** |
| Codabar     | 1017.2  | 1081.4    | **6% faster** |
| DataBar Exp.| 1137.0  | 1149.2    | **1% faster** |
| DX Film Edge| 1260.6  | 1321.0    | **5% faster** |
| QR Code     | 1298.9  | 1308.1    | **1% faster** |
| UPC-E       | 1364.5  | 1490.9    | **8% faster** |
| EAN-8       | 1789.6  | 1968.2    | **9% faster** |
| MaxiCode    | 1946.3  | 2068.9    | **6% faster** |
| Code 39     | 2291.9  | 2476.1    | **7% faster** |
| UPC-A       | 3055.7  | 3299.7    | **7% faster** |
| EAN-13      | 3147.5  | 3389.3    | **7% faster** |
| Code 128    | 5925.8  | 6291.1    | **6% faster** |

### Benchmark Environment

- **OS**: macOS Tahoe 26.5.2 (Apple Silicon)
- **CPU**: Apple M1 Max
- **Python**: 3.13.10
- **Libraries**: pyrxing 0.8.0 (zxing-cpp 0.5.3), zxing-cpp 3.1.1, Pillow 12.3.0
- **Method**: Median of 100 runs with 20-run warm-up, faster of two runs

**Test Images**: Located in [`assets/`](assets/) directory:
- 1D Barcodes: `test_codabar.png`, `test_code39.png`, `test_code93.png`, `test_code128.png`, `test_ean8.png`, `test_ean13.png`, `test_itf.png`, `test_upc_a.png`, `test_upc_e.png`, `test_data_bar.png`, `test_data_bar_expanded.png`, `test_data_bar_limited.png`
- 2D Barcodes: `test_qr_code.png`, `test_micro_qr.png`, `test_rmqr.png`, `test_aztec.png`, `test_data_matrix.png`, `test_pdf417.png`, `test_maxi_code.png`
- Specialty: `test_dx_film_edge.png`

**Reproduce benchmarks**: See [`benchmark.py`](benchmark.py) for the complete benchmark script.

---

## 🧪 Usage

```python
from pyrxing import read_barcode, read_barcodes

# Read a single barcode from an image path
barcode = read_barcode("example.png")

# Read multiple barcodes from an image
barcodes = read_barcodes("example.png")

# Optionally filter by barcode format
barcodes = read_barcodes("example.png", formats=['QRCode'])
```

You can also pass an object that conforms to the `ImageProtocol` instead of a path.
```python
from pyrxing import read_barcode
from PIL import Image

# Read a single barcode from PIL.Image.Image object
barcode = read_barcode(Image.open("example.png"))
```

NumPy-compatible `uint8` arrays are accepted in grayscale (`H, W`) and single-channel (`H, W, 1`) shapes. Sliced and other non-contiguous arrays are supported.

This example generates a QR code with optional OpenCV, then passes the resulting NumPy array directly to the built pyrxing library:

```python
import cv2

from pyrxing import read_barcode

payload = "Hello from OpenCV"
qr_code = cv2.QRCodeEncoder_create().encode(payload)
qr_code = cv2.copyMakeBorder(
  qr_code, 4, 4, 4, 4, cv2.BORDER_CONSTANT, value=255
)
qr_code = cv2.resize(
  qr_code, None, fx=8, fy=8, interpolation=cv2.INTER_NEAREST
)
qr_code = cv2.cvtColor(qr_code, cv2.COLOR_BGR2GRAY)

barcode = read_barcode(qr_code, formats=["QRCode"])
assert barcode is not None
assert barcode.text == payload
```

NumPy and OpenCV are integrations, not runtime dependencies of pyrxing. Install them separately when needed.

---

## 🚫 Not Planned

* ❌ Barcode generation

---

## 📚 API Reference

For full API and type hints, see `pyrxing.pyi` or use your IDE's autocomplete.

```python
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
```

---

## License

Apache License 2.0
