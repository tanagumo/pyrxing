use image::GrayImage;
use rxing::{helpers as rxing_helpers, Exceptions};
use rxing::{BarcodeFormat, Point, RXingResult};

#[derive(Debug)]
pub struct DecodeResult {
    inner: RXingResult,
}

impl DecodeResult {
    fn new(inner: RXingResult) -> Self {
        Self { inner }
    }

    pub fn text(&self) -> &str {
        self.inner.getText()
    }

    pub fn points(&self) -> &[Point] {
        self.inner.getPoints()
    }

    pub fn format(&self) -> &'static str {
        match self.inner.getBarcodeFormat() {
            BarcodeFormat::AZTEC => "aztec",
            BarcodeFormat::CODABAR => "codabar",
            BarcodeFormat::CODE_39 => "code 39",
            BarcodeFormat::CODE_93 => "code 93",
            BarcodeFormat::CODE_128 => "code 128",
            BarcodeFormat::DATA_MATRIX => "datamatrix",
            BarcodeFormat::EAN_8 => "ean 8",
            BarcodeFormat::EAN_13 => "ean 13",
            BarcodeFormat::ITF => "itf",
            BarcodeFormat::MAXICODE => "maxicode",
            BarcodeFormat::PDF_417 => "pdf 417",
            BarcodeFormat::QR_CODE => "qrcode",
            BarcodeFormat::MICRO_QR_CODE => "mqr",
            BarcodeFormat::RECTANGULAR_MICRO_QR_CODE => "rmqr",
            BarcodeFormat::RSS_14 => "rss 14",
            BarcodeFormat::RSS_EXPANDED => "rss expanded",
            BarcodeFormat::TELEPEN => "telepen",
            BarcodeFormat::UPC_A => "upc a",
            BarcodeFormat::UPC_E => "upc e",
            BarcodeFormat::UPC_EAN_EXTENSION => "upc/ean extension",
            BarcodeFormat::DXFilmEdge => "DXFilmEdge",
            _ => "unknown",
        }
    }
}

fn decode(image: GrayImage, multi: bool) -> anyhow::Result<Vec<DecodeResult>> {
    let width = image.width();
    let height = image.height();

    if multi {
        match rxing_helpers::detect_multiple_in_luma(image.into_vec(), width, height) {
            Ok(results) => Ok(results
                .into_iter()
                .map(DecodeResult::new)
                .collect::<Vec<_>>()),
            Err(e) => match e {
                Exceptions::NotFoundException(_) => Ok(vec![]),
                _ => Err(e.into()),
            },
        }
    } else {
        match rxing_helpers::detect_in_luma(image.into_vec(), width, height, None) {
            Ok(r) => Ok(vec![DecodeResult::new(r)]),
            Err(e) => match e {
                Exceptions::NotFoundException(_) => Ok(vec![]),
                _ => Err(e.into()),
            },
        }
    }
}

pub fn decode_single(image: GrayImage) -> anyhow::Result<Vec<DecodeResult>> {
    decode(image, false)
}

pub fn decode_multiple(image: GrayImage) -> anyhow::Result<Vec<DecodeResult>> {
    decode(image, true)
}
