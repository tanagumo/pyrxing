import PIL.Image


class DecodingError(Exception): ...

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


def decode(value: str | PIL.Image.Image) -> list[DecodeResult]: ...

