use std::collections::HashSet;

use image::GrayImage;
use rxing::common::HybridBinarizer;
use rxing::multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader};
use rxing::{
    Binarizer, BinaryBitmap, DecodeHints, Exceptions, Luma8LuminanceSource,
    MultiUseMultiFormatReader, Point, RXingResult, Reader,
};

pub use rxing::BarcodeFormat;

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

trait Decoder {
    fn decode<B>(
        &self,
        binarizer: B,
        hints: &mut DecodeHints,
    ) -> rxing::common::Result<RXingResult>
    where
        B: Binarizer,
        B::Source: Clone;

    fn decode_multiple<B>(
        &self,
        binarizer: B,
        hints: &mut DecodeHints,
    ) -> rxing::common::Result<Vec<RXingResult>>
    where
        B: Binarizer,
        B::Source: Clone;
}

struct AdaptiveDecoder;

impl AdaptiveDecoder {
    fn decode<F, B, T>(
        decode_func: F,
        binarizer: B,
        hints: &DecodeHints,
    ) -> rxing::common::Result<T>
    where
        F: Fn(BinaryBitmap<B>, &DecodeHints) -> rxing::common::Result<T>,
        B: Binarizer,
        B::Source: Clone,
    {
        let bitmap = BinaryBitmap::new(binarizer);
        decode_func(bitmap, hints)
    }
}

impl Decoder for AdaptiveDecoder {
    fn decode<B>(&self, binarizer: B, hints: &mut DecodeHints) -> rxing::common::Result<RXingResult>
    where
        B: Binarizer,
        B::Source: Clone,
    {
        let cloned_source = binarizer.get_luminance_source().clone();
        let cloned_binarizer = binarizer.create_binarizer(cloned_source);

        let decode_func = |mut data, hints: &DecodeHints| {
            let mut reader = MultiUseMultiFormatReader::default();
            reader.decode_with_hints(&mut data, &hints)
        };

        let original_try_harder = hints.TryHarder;
        hints.TryHarder = None;
        match AdaptiveDecoder::decode(decode_func, binarizer, hints) {
            ok @ Ok(_) => ok,
            _ => {
                hints.TryHarder = Some(true);
                match AdaptiveDecoder::decode(decode_func, cloned_binarizer, hints) {
                    ok @ Ok(_) => ok,
                    err @ Err(_) => {
                        hints.TryHarder = original_try_harder;
                        err
                    }
                }
            }
        }
    }

    fn decode_multiple<B>(
        &self,
        binarizer: B,
        hints: &mut DecodeHints,
    ) -> rxing::common::Result<Vec<RXingResult>>
    where
        B: Binarizer,
        B::Source: Clone,
    {
        let cloned_source = binarizer.get_luminance_source().clone();
        let cloned_binarizer = binarizer.create_binarizer(cloned_source);

        let decode_func = |mut data, hints: &DecodeHints| {
            let mut reader =
                GenericMultipleBarcodeReader::new(MultiUseMultiFormatReader::default());
            reader.decode_multiple_with_hints(&mut data, &hints)
        };

        let original_try_harder = hints.TryHarder;
        hints.TryHarder = None;
        match AdaptiveDecoder::decode(decode_func, binarizer, hints) {
            ok @ Ok(_) => ok,
            _ => {
                hints.TryHarder = Some(true);
                match AdaptiveDecoder::decode(decode_func, cloned_binarizer, hints) {
                    ok @ Ok(_) => ok,
                    err @ Err(_) => {
                        hints.TryHarder = original_try_harder;
                        err
                    }
                }
            }
        }
    }
}

fn decode(image: GrayImage, multi: bool, formats: &[BarcodeFormat]) -> anyhow::Result<Decoded> {
    let width = image.width();
    let height = image.height();
    let binarizer = HybridBinarizer::new(Luma8LuminanceSource::new(image.to_vec(), width, height));

    let mut hints = DecodeHints::default();
    if !formats.is_empty() {
        hints.PossibleFormats = Some(formats.iter().map(|v| *v).collect::<HashSet<_>>());
    }

    let decoder = AdaptiveDecoder;

    if multi {
        match decoder.decode_multiple(binarizer, &mut hints) {
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
        match decoder.decode(binarizer, &mut hints) {
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
