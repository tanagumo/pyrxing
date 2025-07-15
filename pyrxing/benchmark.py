import json
import statistics
import time
from glob import glob
from contextlib import contextmanager
from typing import Literal

from PIL import Image

ASSETS = sorted(glob("assets/test_*.png"))


@contextmanager
def measure(stack: list[float]):
    b = time.perf_counter_ns()
    yield
    stack.append(time.perf_counter_ns() - b)


def bench(
    module, filename: str, count: int = 1
) -> dict[Literal["read_barcode", "read_barcodes"], float]:

    ret = {}
    for funcname in ["read_barcode", "read_barcodes"]:
        func = getattr(module, funcname)

        elapsed: list[float] = []
        for _ in range(count):
            i = Image.open(filename)
            with measure(elapsed):
                res = func(i)
            assert res is not None

        ret[funcname] = statistics.median(elapsed) / 1000
    return ret


def main():
    import pyrxing
    import zxingcpp

    # warmup
    {filename: bench(zxingcpp, filename, count=20) for filename in ASSETS}
    {filename: bench(pyrxing, filename, count=20) for filename in ASSETS}

    pyrxing_result = {
        filename: bench(pyrxing, filename, count=100) for filename in ASSETS
    }
    zxingcpp_result = {
        filename: bench(zxingcpp, filename, count=100) for filename in ASSETS
    }

    ret = {
        "pyrxing": pyrxing_result,
        "zxingcpp": zxingcpp_result,
    }

    print(json.dumps(ret, indent=4))


if __name__ == "__main__":
    main()
