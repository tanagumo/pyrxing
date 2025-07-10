import statistics
import time
from contextlib import contextmanager
from dataclasses import dataclass
from typing import Callable, Generator, Self

import PIL.Image
from PIL.Image import Image as PILImage


class Measure:
    def __init__(self) -> None:
        self._start = None
        self._end = None

    def __enter__(self) -> Self:
        self._start = time.perf_counter_ns()
        return self

    def __exit__(self, *a, **k):
        self._end = time.perf_counter_ns()

    def get_elapsed_ns(self) -> float:
        assert self._end and self._start
        return self._end - self._start


@contextmanager
def measure() -> Generator[Measure, None, None]:
    m = Measure()
    with m:
        yield m


@dataclass
class Image:
    image: PILImage
    text: str


def load_images() -> list[Image]:
    image_to_text = {
        "test_aztec.gif": "This is an Aztec Code by TEC-IT",
        "test_codabar.png": "A974917948$D",
        "test_code_39.gif": "ABC-1234-39",
        "test_code_93.gif": "ABC-1234-/+",
        "test_code_128.gif": "ABC-xyz-1234",
        "test_data_matrix.gif": "This is a Data Matrix by TEC-IT",
        "test_ean_8.gif": "9031101",
        "test_ean_13.gif": "9780201379624",
        "test_itf.png": "6928974920423",
        "test_maxicode.gif": "This is a MaxiCode by TEC-IT",
        "test_pdf417.gif": "This is a PDF417 by TEC-IT",
        "test_qr_code.gif": "This is a QR Code by TEC-IT",
        "test_micro_qr_code.gif": "ABC-MICRO-123",
        "test_rMQR.png": "this is rMQR code",
        "test_rss_14.gif": "0123456789012",
        "test_rss_expanded.gif": "011234567890123-ABCabc",
        "test_telepen.gif": "ABC-abc-1234",
        "test_upc_a.gif": "72527273070",
        "test_upc_e.gif": "0123456",
        "test_upc_ean_extension.png": "48902",
    }

    images = []

    for name in image_to_text.keys():
        image = PIL.Image.open(f"assets/benchmark/{name}")
        text = image_to_text[name]
        images.append(Image(image=image, text=text))

    return images


type ReadBarcode = Callable[[PILImage], str | None]


@dataclass
class BenchResult:
    library_name: str
    avg_speed_micro: float
    accuracy_percent: float


def benchmark_single(
    library_name: str, read_barcode: ReadBarcode, images: list[Image]
) -> BenchResult:
    successful_reads = 0
    total_elapsed = 0
    for image in images:
        with measure() as m:
            result = read_barcode(image.image)
        total_elapsed += m.get_elapsed_ns()

        if result is not None and result == image.text:
            successful_reads += 1

    return BenchResult(
        library_name=library_name,
        avg_speed_micro=(total_elapsed / 1000) / len(images),
        accuracy_percent=(successful_reads / len(images)) * 100,
    )


def _benchmark(
    library_name: str,
    read_barcode: ReadBarcode,
    images: list[Image],
    run_count: int,
) -> BenchResult:
    results: list[BenchResult] = []
    for _ in range(run_count):
        results.append(benchmark_single(library_name, read_barcode, images))

    return BenchResult(
        library_name=library_name,
        avg_speed_micro=statistics.median([b.avg_speed_micro for b in results]),
        accuracy_percent=statistics.median([b.accuracy_percent for b in results]),
    )


def benchmark_pyrxing(images: list[Image], run_count: int = 10) -> BenchResult:
    import pyrxing

    def read_with_pyrxing(image: PILImage) -> str | None:
        ret = pyrxing.read_barcode(image)
        return ret and ret.text

    return _benchmark("pyrxing", read_with_pyrxing, images, run_count=run_count)


def benchmark_zxingcpp(images: list[Image], run_count: int = 10) -> BenchResult:
    import zxingcpp

    def read_with_zxingcpp(image: PILImage) -> str | None:
        ret = zxingcpp.read_barcode(image)
        return ret and ret.text

    return _benchmark("zxingcpp", read_with_zxingcpp, images, run_count=run_count)


def main(): ...


if __name__ == "__main__":
    main()
