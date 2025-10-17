# pyrxing

**pyrxing** is a fast, dependency-free Python barcode/QR code reader built using [zxing-cpp](https://github.com/zxing-cpp/zxing-cpp) Rust bindings via [PyO3](https://github.com/PyO3/pyo3).

This library offers efficient barcode scanning in pure Python environments, with pre-built native wheels — including full support for **Alpine Linux** and `musl`-based systems where the official [zxing-cpp Python package](https://pypi.org/project/zxing-cpp/) requires additional build steps.

---

## 🚀 Features

* ⚡ **High performance**: Powered by zxing-cpp C++ library through optimized Rust bindings with excellent multiple barcode detection performance
* 🐍 **Python-native API**: Simple interface with just two functions: `read_barcode` and `read_barcodes`
* 📦 **No system dependencies**: No need for zbar, JRE, or any external libraries
* 🏗 **Alpine Linux compatible**: Pre-built `musllinux` wheels available (no build required)
* 🧠 **Type hinting & autocompletion**: Includes `.pyi` stub files
* 🔒 **Safe and minimal**: No unnecessary features — just barcode reading
* ⚡ **Competitive performance**: Outperforms zxing-cpp on 65.0% of formats in single detection and 80.0% in multiple detection

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

pyrxing delivers excellent performance across a comprehensive range of barcode formats, with particularly strong advantages in multiple barcode detection scenarios:

**Performance Summary:**
- **Single detection**: pyrxing outperforms zxing-cpp on **13 out of 20 formats** (65.0%)
- **Multiple detection**: pyrxing outperforms zxing-cpp on **16 out of 20 formats** (80.0%)

### Benchmark Results (μs per decode)

#### `read_barcode()` - Single Barcode Detection

| Format      | pyrxing | zxing-cpp | Improvement |
|-------------|---------|-----------|-------------|
| UPC-E       | 555.9   | 710.8     | **28% faster** |
| Data Matrix | 260.0   | 330.9     | **27% faster** |
| EAN-8       | 881.8   | 1048.2    | **19% faster** |
| UPC-A       | 1159.5  | 1358.5    | **17% faster** |
| EAN-13      | 1161.7  | 1328.4    | **14% faster** |
| Code 39     | 1063.5  | 1212.8    | **14% faster** |
| ITF         | 336.2   | 382.3     | **14% faster** |
| Code 128    | 2272.2  | 2541.0    | **12% faster** |
| Codabar     | 415.1   | 468.1     | **13% faster** |
| MaxiCode    | 947.5   | 1026.0    | **8% faster** |
| PDF417      | 369.6   | 400.1     | **8% faster** |
| Aztec       | 85.9    | 92.8      | **7% faster** |
| DX Film Edge| 302.1   | 314.1     | **4% faster** |
| QR Code     | 294.5   | 292.3     | 1% slower |
| Code 93     | 281.3   | 276.3     | 2% slower |
| Micro QR    | 75.4    | 73.1      | 3% slower |
| rMQR        | 84.6    | 84.3      | 0% (tied) |
| DataBar     | 198.5   | 196.9     | 1% slower |
| DataBar Exp.| 358.3   | 356.1     | 1% slower |
| DataBar Ltd.| 196.6   | 186.6     | 5% slower |

#### `read_barcodes()` - Multiple Barcode Detection

| Format      | pyrxing | zxing-cpp | Improvement |
|-------------|---------|-----------|-------------|
| UPC-A       | 4500.0  | 4817.0    | **7% faster** |
| Codabar     | 1248.8  | 1341.4    | **7% faster** |
| EAN-8       | 2683.3  | 2854.9    | **6% faster** |
| UPC-E       | 1962.7  | 2062.0    | **5% faster** |
| ITF         | 1039.5  | 1085.9    | **4% faster** |
| Code 39     | 3078.0  | 3206.5    | **4% faster** |
| PDF417      | 851.1   | 883.7     | **4% faster** |
| EAN-13      | 4566.9  | 4739.6    | **4% faster** |
| Code 128    | 7769.5  | 8014.2    | **3% faster** |
| Aztec       | 558.3   | 575.1     | **3% faster** |
| DX Film Edge| 1572.9  | 1609.0    | **2% faster** |
| MaxiCode    | 2509.3  | 2561.3    | **2% faster** |
| Data Matrix | 1002.5  | 1082.3    | **8% faster** |
| rMQR        | 466.5   | 471.9     | **1% faster** |
| QR Code     | 2203.3  | 2205.6    | **0% faster** |
| Micro QR    | 407.0   | 407.8     | **0% faster** |
| Code 93     | 950.5   | 937.8     | 1% slower |
| DataBar     | 670.4   | 666.9     | 1% slower |
| DataBar Exp.| 1300.4  | 1279.2    | 2% slower |
| DataBar Ltd.| 629.3   | 620.7     | 1% slower |

### Benchmark Environment

- **OS**: macOS 15.5 (Apple Silicon)
- **CPU**: Apple M1 Max (10-core)
- **Python**: 3.13.3
- **Libraries**: pyrxing (current), zxing-cpp 2.3.0, Pillow 11.3.0
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
```

---

## License

Apache License 2.0
