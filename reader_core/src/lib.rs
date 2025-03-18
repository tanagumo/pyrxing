use std::collections::HashSet;

use image::GrayImage;
use rxing::common::HybridBinarizer;
use rxing::multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader};
use rxing::{
    BinaryBitmap, DecodeHintType, DecodeHintValue, DecodeHints, DecodingHintDictionary, Exceptions,
    Luma8LuminanceSource, MultiUseMultiFormatReader, Point, RXingResult, Reader,
};

pub use rxing::BarcodeFormat;

static DEFAULT_FORMATS: &'static [BarcodeFormat] = &[
    BarcodeFormat::AZTEC,
    BarcodeFormat::CODABAR,
    BarcodeFormat::CODE_39,
    BarcodeFormat::CODE_93,
    BarcodeFormat::CODE_128,
    BarcodeFormat::DATA_MATRIX,
    BarcodeFormat::EAN_8,
    BarcodeFormat::EAN_13,
    BarcodeFormat::ITF,
    BarcodeFormat::MAXICODE,
    BarcodeFormat::PDF_417,
    BarcodeFormat::QR_CODE,
    BarcodeFormat::MICRO_QR_CODE,
    BarcodeFormat::RECTANGULAR_MICRO_QR_CODE,
    BarcodeFormat::RSS_14,
    BarcodeFormat::RSS_EXPANDED,
    BarcodeFormat::TELEPEN,
    BarcodeFormat::UPC_A,
    BarcodeFormat::UPC_E,
    BarcodeFormat::UPC_EAN_EXTENSION,
    BarcodeFormat::DXFilmEdge,
];

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

    pub fn format(&self) -> &BarcodeFormat {
        self.inner.getBarcodeFormat()
    }
}

enum Decoded {
    Single(Option<DecodeResult>),
    Multi(Vec<DecodeResult>),
}

fn decode(image: GrayImage, multi: bool, formats: &[BarcodeFormat]) -> anyhow::Result<Decoded> {
    let width = image.width();
    let height = image.height();
    let mut data = BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
        image.to_vec(),
        width,
        height,
    )));

    let formats = if !formats.is_empty() {
        formats
    } else {
        DEFAULT_FORMATS
    };

    let hints = DecodingHintDictionary::from([(
        DecodeHintType::POSSIBLE_FORMATS,
        DecodeHintValue::PossibleFormats(formats.iter().map(|v| *v).collect::<HashSet<_>>()),
    )]);
    let hints = DecodeHints::from(hints);

    if multi {
        let mut reader = GenericMultipleBarcodeReader::new(MultiUseMultiFormatReader::default());

        match reader.decode_multiple_with_hints(&mut data, &hints) {
            Ok(results) => Ok(Decoded::Multi(
                results
                    .into_iter()
                    .map(DecodeResult::new)
                    .collect::<Vec<_>>(),
            )),
            Err(e) => match e {
                Exceptions::NotFoundException(_) => Ok(Decoded::Multi(vec![])),
                _ => Err(e.into()),
            },
        }
    } else {
        let mut reader = MultiUseMultiFormatReader::default();
        match reader.decode_with_hints(&mut data, &hints) {
            Ok(r) => Ok(Decoded::Single(Some(DecodeResult::new(r)))),
            Err(e) => match e {
                Exceptions::NotFoundException(_) => Ok(Decoded::Single(None)),
                _ => Err(e.into()),
            },
        }
    }
}

pub fn decode_single(
    image: GrayImage,
    formats: &[BarcodeFormat],
) -> anyhow::Result<Option<DecodeResult>> {
    decode(image, false, formats).map(|decoded| match decoded {
        Decoded::Single(r) => r,
        _ => unreachable!(),
    })
}

pub fn decode_multiple(
    image: GrayImage,
    formats: &[BarcodeFormat],
) -> anyhow::Result<Vec<DecodeResult>> {
    decode(image, true, formats).map(|decoded| match decoded {
        Decoded::Multi(results) => results,
        _ => unreachable!(),
    })
}
