import pytest
from PIL import Image

import pyrxing


ASSETS = {
    "assets/test_aztec.png": {"format": "MicroQRCode", "value": "Code Code Data"},
    "assets/test_codabar.png": {"format": "Codabar", "value": "A-12656-D"},
    "assets/test_code128.png": {
        "format": "Code128",
        "value": "Python Barcode Random World",
    },
    "assets/test_code39.png": {"format": "Code39", "value": "QD9MZ5O"},
    "assets/test_code93.png": {"format": "Code93", "value": "ABC-1234-/+"},
    "assets/test_data_bar.png": {
        "format": "DataBar",
        "value": "01234567890128",
    },
    "assets/test_data_bar_expanded.png": {
        "format": "DataBarExpanded",
        "value": "011234567890123-ABCabc",
    },
    "assets/test_data_bar_limited.png": {
        "format": "DataBarLimited",
        "value": "0101234567890128",
    },
    "assets/test_data_matrix.png": {
        "format": "DataMatrix",
        "value": "DataMatrix_31802",
    },
    "assets/test_dx_film_edge.png": {
        "format": "Code128",
        "value": "DXFilmEdge_6086",
    },
    "assets/test_ean13.png": {"format": "EAN13", "value": "2708639496369"},
    "assets/test_ean8.png": {"format": "EAN8", "value": "07727931"},
    "assets/test_itf.png": {"format": "ITF", "value": "4577279804"},
    "assets/test_maxi_code.png": {
        "format": "MaxiCode",
        "value": "MaxiCode_46753",
    },
    "assets/test_micro_qr.png": {
        "format": "MicroQRCode",
        "value": "0ZFXY9",
    },
    "assets/test_pdf417.png": {"format": "PDF417", "value": "PDF417_12417"},
    "assets/test_qr_code.png": {
        "format": "QRCode",
        "value": "https://demo.net/demo/7809",
    },
    "assets/test_rmqr.png": {"format": "MicroQRCode", "value": "V4GQ68D2"},
    "assets/test_upc_a.png": {"format": "UPCA", "value": "041935974561"},
    "assets/test_upc_e.png": {"format": "UPCE", "value": "18274974"},
}

def test_read():
    for path, v in ASSETS.items():
        i = Image.open(path)
        res = pyrxing.read_barcode(i)
        assert res is not None
        assert res.text == v['value']
        assert res.format == v['format']

        res = pyrxing.read_barcodes(i)
        assert len(res) == 1
        assert res[0].text == v['value']
        assert res[0].format == v['format']
