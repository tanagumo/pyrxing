use std::{cell::OnceCell, fmt::Display};

use flagset::FlagSet;
use image::GrayImage;
use zxingcpp::{Barcode, BarcodeFormat as ZxBarcodeFormat, PointI};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl From<PointI> for Point {
    fn from(value: PointI) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum BarcodeFormat {
    Aztec,
    Codabar,
    Code39,
    Code93,
    Code128,
    DataBar,
    DataBarExpanded,
    DataBarLimited,
    DataMatrix,
    EAN8,
    EAN13,
    ITF,
    MaxiCode,
    PDF417,
    QRCode,
    UPCA,
    UPCE,
    MicroQRCode,
    RMQRCode,
    DXFilmEdge,
    LinearCodes,
    MatrixCodes,
}

impl Display for BarcodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BarcodeFormat::*;
        let v = match self {
            Aztec => "Aztec",
            Codabar => "Codabar",
            Code39 => "Code39",
            Code93 => "Code93",
            Code128 => "Code128",
            DataBar => "DataBar",
            DataBarExpanded => "DataBarExpanded",
            DataBarLimited => "DataBarLimited",
            DataMatrix => "DataMatrix",
            EAN8 => "EAN8",
            EAN13 => "EAN13",
            ITF => "ITF",
            MaxiCode => "MaxiCode",
            PDF417 => "PDF417",
            QRCode => "QRCode",
            UPCA => "UPCA",
            UPCE => "UPCE",
            MicroQRCode => "MicroQRCode",
            RMQRCode => "RMQRCode",
            DXFilmEdge => "DXFilmEdge",
            LinearCodes => "LinearCodes",
            MatrixCodes => "MatrixCodes",
        };
        write!(f, "{}", v)
    }
}

impl From<BarcodeFormat> for ZxBarcodeFormat {
    fn from(value: BarcodeFormat) -> Self {
        use BarcodeFormat::*;
        match value {
            Aztec => ZxBarcodeFormat::Aztec,
            Codabar => ZxBarcodeFormat::Codabar,
            Code39 => ZxBarcodeFormat::Code39,
            Code93 => ZxBarcodeFormat::Code93,
            Code128 => ZxBarcodeFormat::Code128,
            DataBar => ZxBarcodeFormat::DataBar,
            DataBarExpanded => ZxBarcodeFormat::DataBarExpanded,
            DataBarLimited => ZxBarcodeFormat::DataBarLimited,
            DataMatrix => ZxBarcodeFormat::DataMatrix,
            EAN8 => ZxBarcodeFormat::EAN8,
            EAN13 => ZxBarcodeFormat::EAN13,
            ITF => ZxBarcodeFormat::ITF,
            MaxiCode => ZxBarcodeFormat::MaxiCode,
            PDF417 => ZxBarcodeFormat::PDF417,
            QRCode => ZxBarcodeFormat::QRCode,
            UPCA => ZxBarcodeFormat::UPCA,
            UPCE => ZxBarcodeFormat::UPCE,
            MicroQRCode => ZxBarcodeFormat::MicroQRCode,
            RMQRCode => ZxBarcodeFormat::RMQRCode,
            DXFilmEdge => ZxBarcodeFormat::DXFilmEdge,
            LinearCodes => ZxBarcodeFormat::LinearCodes,
            MatrixCodes => ZxBarcodeFormat::MatrixCodes,
        }
    }
}

impl From<ZxBarcodeFormat> for BarcodeFormat {
    fn from(value: ZxBarcodeFormat) -> Self {
        use ZxBarcodeFormat::*;
        match value {
            Aztec => BarcodeFormat::Aztec,
            Codabar => BarcodeFormat::Codabar,
            Code39 => BarcodeFormat::Code39,
            Code93 => BarcodeFormat::Code93,
            Code128 => BarcodeFormat::Code128,
            DataBar => BarcodeFormat::DataBar,
            DataBarExpanded => BarcodeFormat::DataBarExpanded,
            DataBarLimited => BarcodeFormat::DataBarLimited,
            DataMatrix => BarcodeFormat::DataMatrix,
            EAN8 => BarcodeFormat::EAN8,
            EAN13 => BarcodeFormat::EAN13,
            ITF => BarcodeFormat::ITF,
            MaxiCode => BarcodeFormat::MaxiCode,
            PDF417 => BarcodeFormat::PDF417,
            QRCode => BarcodeFormat::QRCode,
            UPCA => BarcodeFormat::UPCA,
            UPCE => BarcodeFormat::UPCE,
            MicroQRCode => BarcodeFormat::MicroQRCode,
            RMQRCode => BarcodeFormat::RMQRCode,
            DXFilmEdge => BarcodeFormat::DXFilmEdge,
            LinearCodes => BarcodeFormat::LinearCodes,
            MatrixCodes => BarcodeFormat::MatrixCodes,
            other => panic!("unexpected code: {}", other),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    top_left: Point,
    bottom_right: Point,
}

impl Position {
    pub fn top_left(&self) -> Point {
        self.top_left
    }

    pub fn bottom_right(&self) -> Point {
        self.bottom_right
    }
}

pub struct DecodeResult {
    inner: Barcode,
    cached_text: OnceCell<String>,
    cached_position: OnceCell<Position>,
    cached_format: OnceCell<BarcodeFormat>,
}

impl DecodeResult {
    fn new(inner: Barcode) -> Self {
        Self {
            inner,
            cached_text: OnceCell::new(),
            cached_position: OnceCell::new(),
            cached_format: OnceCell::new(),
        }
    }

    pub fn text(&self) -> &str {
        self.cached_text.get_or_init(|| self.inner.text())
    }

    pub fn position(&self) -> Position {
        *self.cached_position.get_or_init(|| {
            let position = self.inner.position();
            Position {
                top_left: position.top_left.into(),
                bottom_right: position.bottom_right.into(),
            }
        })
    }

    pub fn format(&self) -> BarcodeFormat {
        *self
            .cached_format
            .get_or_init(|| self.inner.format().into())
    }

    pub fn points(&self) -> [Point; 4] {
        let position = self.position();
        let top_left = position.top_left();
        let bottom_right = position.bottom_right();

        [
            top_left,
            Point::new(bottom_right.x, top_left.y),
            bottom_right,
            Point::new(top_left.x, bottom_right.y),
        ]
    }
}

pub fn decode_multiple(
    image: GrayImage,
    formats: &[BarcodeFormat],
) -> anyhow::Result<Vec<DecodeResult>> {
    let format: FlagSet<ZxBarcodeFormat> = if formats.is_empty() {
        Into::<FlagSet<ZxBarcodeFormat>>::into(ZxBarcodeFormat::Any)
    } else {
        let mut flag = Into::<FlagSet<ZxBarcodeFormat>>::into(ZxBarcodeFormat::None);
        for f in formats {
            flag |= Into::<ZxBarcodeFormat>::into(*f);
        }
        flag
    };

    let read_barcodes = zxingcpp::read()
        .formats(format)
        .try_harder(false)
        .try_invert(false)
        .try_downscale(false)
        .is_pure(false);
    let barcodes = read_barcodes.from(&image)?;
    Ok(barcodes
        .into_iter()
        .map(DecodeResult::new)
        .collect::<Vec<_>>())
}

pub fn decode_single(
    image: GrayImage,
    formats: &[BarcodeFormat],
) -> anyhow::Result<Option<DecodeResult>> {
    match decode_multiple(image, formats) {
        Ok(mut results) => {
            if results.is_empty() {
                Ok(None)
            } else {
                Ok(Some(results.remove(0)))
            }
        }
        Err(e) => Err(e),
    }
}
