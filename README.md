# pyrxing

**pyrxing** is a fast, dependency-free Python barcode/QR code reader built using [zxing-cpp](https://github.com/zxing-cpp/zxing-cpp) Rust bindings via [PyO3](https://github.com/PyO3/pyo3).

This library offers efficient barcode scanning in pure Python environments, with pre-built native wheels — including full support for **Alpine Linux** and `musl`-based systems where the official [zxing-cpp Python package](https://pypi.org/project/zxing-cpp/) requires additional build steps.

---

## 🚀 Features

* ⚡ **High performance**: Powered by zxing-cpp 0.5.0 C++ library through optimized Rust bindings with excellent barcode detection performance
* 🐍 **Python-native API**: Simple interface with just two functions: `read_barcode` and `read_barcodes`
* 📦 **No system dependencies**: No need for zbar, JRE, or any external libraries
* 🏗 **Alpine Linux compatible**: Pre-built `musllinux` wheels available (no build required)
* 🧠 **Type hinting & autocompletion**: Includes `.pyi` stub files with 32 barcode format variants
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
- **Single detection**: Comparable performance with zxing-cpp (within ±26% across all formats)
- **Multiple detection**: Comparable performance with zxing-cpp (within ±18% across all formats)
- **Overall**: Nearly identical median performance, with pyrxing being faster on most formats

### Benchmark Results (μs per decode)

#### `read_barcode()` - Single Barcode Detection

| Format      | pyrxing | zxing-cpp | Difference |
|-------------|---------|-----------|------------|
| Micro QR    | 65.2    | 68.4      | **5% faster** |
| rMQR        | 73.9    | 74.7      | **1% faster** |
| Aztec       | 84.0    | 85.7      | **2% faster** |
| DataBar     | 192.4   | 193.6     | **1% faster** |
| DataBar Ltd.| 190.1   | 192.6     | **1% faster** |
| Data Matrix | 246.4   | 334.2     | **26% faster** |
| Code 93     | 272.6   | 274.8     | **1% faster** |
| DX Film Edge| 293.3   | 336.3     | **13% faster** |
| QR Code     | 299.6   | 304.5     | **2% faster** |
| ITF         | 340.1   | 412.2     | **17% faster** |
| DataBar Exp.| 360.3   | 357.6     | 1% slower |
| PDF417      | 368.1   | 406.7     | **9% faster** |
| Codabar     | 412.6   | 473.7     | **13% faster** |
| UPC-E       | 548.9   | 687.9     | **20% faster** |
| EAN-8       | 863.0   | 1084.3    | **20% faster** |
| MaxiCode    | 923.8   | 1052.4    | **12% faster** |
| Code 39     | 1042.9  | 1249.8    | **17% faster** |
| EAN-13      | 1111.1  | 1403.8    | **21% faster** |
| UPC-A       | 1123.4  | 1412.0    | **20% faster** |
| Code 128    | 2171.9  | 2638.8    | **18% faster** |

#### `read_barcodes()` - Multiple Barcode Detection

| Format      | pyrxing | zxing-cpp | Difference |
|-------------|---------|-----------|------------|
| Micro QR    | 470.8   | 471.9     | **0% faster** |
| rMQR        | 539.0   | 542.1     | **1% faster** |
| Aztec       | 649.1   | 652.6     | **1% faster** |
| DataBar Ltd.| 741.3   | 741.8     | **0% faster** |
| DataBar     | 781.7   | 782.1     | **0% faster** |
| PDF417      | 990.1   | 1024.5    | **3% faster** |
| Code 93     | 1091.8  | 1095.1    | **0% faster** |
| Data Matrix | 1142.0  | 1223.8    | **7% faster** |
| ITF         | 1208.8  | 1277.6    | **5% faster** |
| Codabar     | 1437.5  | 1490.8    | **4% faster** |
| DataBar Exp.| 1508.9  | 1508.9    | **0% faster** |
| DX Film Edge| 1509.2  | 1835.6    | **18% faster** |
| UPC-E       | 2159.3  | 2405.4    | **10% faster** |
| QR Code     | 2477.5  | 2484.5    | **0% faster** |
| MaxiCode    | 2821.3  | 2910.4    | **3% faster** |
| EAN-8       | 2964.1  | 3211.3    | **8% faster** |
| Code 39     | 3423.3  | 3621.0    | **5% faster** |
| UPC-A       | 4796.1  | 5359.9    | **11% faster** |
| EAN-13      | 4888.4  | 5377.9    | **9% faster** |
| Code 128    | 8671.9  | 9107.1    | **5% faster** |

### Benchmark Environment

- **OS**: macOS Sequoia 15.2 (Apple Silicon)
- **CPU**: Apple M1 Max
- **Python**: 3.13.10
- **Libraries**: pyrxing 0.4.4 (zxing-cpp 0.5.0), zxing-cpp 3.0.0, Pillow 12.1.1
- **Method**: Median of 100 runs with 20-run warm-up

**Test Images**: Located in [`pyrxing/assets/`](pyrxing/assets/) directory:
- 1D Barcodes: `test_codabar.png`, `test_code39.png`, `test_code93.png`, `test_code128.png`, `test_ean8.png`, `test_ean13.png`, `test_itf.png`, `test_upc_a.png`, `test_upc_e.png`, `test_data_bar.png`, `test_data_bar_expanded.png`, `test_data_bar_limited.png`
- 2D Barcodes: `test_qr_code.png`, `test_micro_qr.png`, `test_rmqr.png`, `test_aztec.png`, `test_data_matrix.png`, `test_pdf417.png`, `test_maxi_code.png`
- Specialty: `test_dx_film_edge.png`

**Reproduce benchmarks**: See [`pyrxing/benchmark.py`](pyrxing/benchmark.py) for the complete benchmark script.

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


def read_barcode(image: str | ImageProtocol, *, formats: list[BarcodeFormat] | None = None) -> DecodeResult | None: ...
def read_barcodes(image: str | ImageProtocol, *, formats: list[BarcodeFormat] | None = None) -> list[DecodeResult]: ...
```

---

## License

Apache License 2.0
