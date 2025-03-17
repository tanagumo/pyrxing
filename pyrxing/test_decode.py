import pytest

from PIL import Image
import pyrxing


def test_read_barcodes():
    qr_code = Image.open('assets/test_qr1_rgb.png')
    res = pyrxing.read_barcodes(qr_code)
    assert len(res) == 1
    assert res[0].text == 'test_qr1'
    assert res[0].format == 'qrcode'

    # read_barcodes accept path for image file
    res = pyrxing.read_barcodes('assets/test_qr1_rgb.png')
    assert len(res) == 1
    assert res[0].text == 'test_qr1'
    assert res[0].format == 'qrcode'

    qr_code_palette = Image.open('assets/test_qr1_palette.png')
    assert qr_code_palette.mode == 'P'
    res = pyrxing.read_barcodes(qr_code_palette)
    assert len(res) == 1
    assert res[0].text == 'test_qr1'
    assert res[0].format == 'qrcode'

    itf_code = Image.open('assets/itf_code.png')
    res = pyrxing.read_barcodes(itf_code)
    assert len(res) == 1
    assert res[0].text == '12121212121217'
    assert res[0].format == 'itf'

    jan_code = Image.open('assets/jan_13.png')
    res = pyrxing.read_barcodes(jan_code)
    assert len(res) == 1
    assert res[0].text == '1212121212128'
    assert res[0].format == 'ean 13'

    # image merged from test_qr1_rgb.png and itf_code.png
    multiple = Image.open('assets/multiple_codes.png')
    res = pyrxing.read_barcodes(multiple)
    assert len(res) == 2
    assert {r.text for r in res} == {'12121212121217', 'test_qr1'}
    assert {r.format for r in res} == {'qrcode', 'itf'}

    # when the formats parameter is specified, only codes matching the specified formats are returned.
    res = pyrxing.read_barcodes(multiple, formats=['QR_CODE'])
    assert len(res) == 1
    assert {r.text for r in res} == {'test_qr1'}
    assert {r.format for r in res} == {'qrcode'}

    res = pyrxing.read_barcodes(multiple, formats=['ITF'])
    assert len(res) == 1
    assert {r.text for r in res} == {'12121212121217'}
    assert {r.format for r in res} == {'itf'}

    res = pyrxing.read_barcodes(multiple, formats=['ITF', 'QR_CODE'])
    assert len(res) == 2
    assert {r.text for r in res} == {'12121212121217', 'test_qr1'}
    assert {r.format for r in res} == {'qrcode', 'itf'}

    res = pyrxing.read_barcodes(multiple, formats=['AZTEC'])
    assert res == []

    # Raise FileNotFoundError if the file does not exist.
    with pytest.raises(FileNotFoundError):
        pyrxing.read_barcodes('does_not_exist')

    # Raise a ValueError if the value passed to function read_barcodes is not of type str .
    with pytest.raises(FileNotFoundError):
        pyrxing.read_barcodes('does_not_exist')

    # Raise a ImageError if the mode of the image is not supported.
    cmyk = Image.open('assets/test_qr_cmyk.jpeg')
    assert cmyk.mode == 'CMYK'
    with pytest.raises(pyrxing.ImageError):
        pyrxing.read_barcodes(cmyk)

    # When an image does not have any codes, read_barcodes returns an empty list.
    res = pyrxing.read_barcodes('assets/no_code.png')
    assert res == []

    # When an image does not have any codes, read_barcodes returns an empty list.
    res = pyrxing.read_barcode('assets/no_code.png')
    assert res is None
