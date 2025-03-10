import pytest

from PIL import Image
import pyrxing


def test_decode():
    qr_code = Image.open('assets/test_qr1_rgb.png')
    res = pyrxing.decode(qr_code)
    assert len(res) == 1
    assert res[0].text == 'test_qr1'
    assert res[0].format == 'qrcode'

    # decode accept path for image file
    res = pyrxing.decode('assets/test_qr1_rgb.png')
    assert len(res) == 1
    assert res[0].text == 'test_qr1'
    assert res[0].format == 'qrcode'

    qr_code_palette = Image.open('assets/test_qr1_palette.png')
    assert qr_code_palette.mode == 'P'
    res = pyrxing.decode(qr_code_palette)
    assert len(res) == 1
    assert res[0].text == 'test_qr1'
    assert res[0].format == 'qrcode'

    itf_code = Image.open('assets/itf_code.png')
    res = pyrxing.decode(itf_code)
    assert len(res) == 1
    assert res[0].text == '12121212121217'
    assert res[0].format == 'itf'

    jan_code = Image.open('assets/jan_13.png')
    res = pyrxing.decode(jan_code)
    assert len(res) == 1
    assert res[0].text == '1212121212128'
    assert res[0].format == 'ean 13'

    # image merged from test_qr1_rgb.png and itf_code.png
    multiple = Image.open('assets/multiple_codes.png')
    res = pyrxing.decode(multiple)
    assert len(res) == 2
    assert {r.text for r in res} == {'12121212121217', 'test_qr1'}
    assert {r.format for r in res} == {'qrcode', 'itf'}

    # Raise FileNotFoundError if the file does not exist.
    with pytest.raises(FileNotFoundError):
        pyrxing.decode('does_not_exist')

    # Raise a ValueError if the value passed to function decode is not of type str .
    with pytest.raises(FileNotFoundError):
        pyrxing.decode('does_not_exist')

    # Raise a ImageError if the mode of the image is not supported.
    cmyk = Image.open('assets/test_qr_cmyk.jpeg')
    assert cmyk.mode == 'CMYK'
    with pytest.raises(pyrxing.ImageError):
        pyrxing.decode(cmyk)

    # When an image does not have any codes, decode returns an empty list.
    res = pyrxing.decode('assets/no_code.png')
    assert res == []
