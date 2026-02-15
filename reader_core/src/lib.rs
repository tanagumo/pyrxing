use std::{borrow::Cow, cell::OnceCell, fmt::Display};

use thiserror::Error;
use zxingcpp::{Barcode, BarcodeFormat as ZxBarcodeFormat, ImageFormat, ImageView, PointI};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unsupported barcode format: {0}")]
    UnsupportedFormat(String),

    #[error("Failed to decode barcode: {0}")]
    DecodeError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl From<PointI> for Point {
    fn from(value: PointI) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum BarcodeFormat {
    Aztec,
    AztecCode,
    AztecRune,
    Codabar,
    Code128,
    Code39,
    Code93,
    CompactPDF417,
    DXFilmEdge,
    DataBar,
    DataBarExp,
    DataBarExpStk,
    DataBarLtd,
    DataBarOmni,
    DataBarStk,
    DataBarStkOmni,
    DataMatrix,
    EAN13,
    EAN2,
    EAN5,
    EAN8,
    EANUPC,
    ISBN,
    ITF,
    MaxiCode,
    MicroPDF417,
    MicroQRCode,
    PDF417,
    PZN,
    QRCode,
    QRCodeModel1,
    QRCodeModel2,
    RMQRCode,
    UPCA,
    UPCE,
}

impl Display for BarcodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BarcodeFormat::*;
        let v = match self {
            Aztec => "Aztec",
            AztecCode => "AztecCode",
            AztecRune => "AztecRune",
            Codabar => "Codabar",
            Code39 => "Code39",
            Code93 => "Code93",
            Code128 => "Code128",
            CompactPDF417 => "CompactPDF417",
            DataBar => "DataBar",
            DataBarExp => "DataBarExpanded",
            DataBarExpStk => "DataBarExpandedStacked",
            DataBarLtd => "DataBarLimited",
            DataBarOmni => "DataBarOmni",
            DataBarStk => "DataBarStacked",
            DataBarStkOmni => "DataBarStackedOmni",
            DataMatrix => "DataMatrix",
            DXFilmEdge => "DXFilmEdge",
            EAN2 => "EAN2",
            EAN5 => "EAN5",
            EAN8 => "EAN8",
            EAN13 => "EAN13",
            EANUPC => "EANUPC",
            ISBN => "ISBN",
            ITF => "ITF",
            MaxiCode => "MaxiCode",
            MicroPDF417 => "MicroPDF417",
            MicroQRCode => "MicroQRCode",
            PDF417 => "PDF417",
            PZN => "PZN",
            QRCode => "QRCode",
            QRCodeModel1 => "QRCodeModel1",
            QRCodeModel2 => "QRCodeModel2",
            RMQRCode => "RMQRCode",
            UPCA => "UPCA",
            UPCE => "UPCE",
        };
        write!(f, "{}", v)
    }
}

impl From<BarcodeFormat> for ZxBarcodeFormat {
    fn from(value: BarcodeFormat) -> Self {
        use BarcodeFormat::*;
        match value {
            Aztec => ZxBarcodeFormat::Aztec,
            AztecCode => ZxBarcodeFormat::AztecCode,
            AztecRune => ZxBarcodeFormat::AztecRune,
            Codabar => ZxBarcodeFormat::Codabar,
            Code39 => ZxBarcodeFormat::Code39,
            Code93 => ZxBarcodeFormat::Code93,
            Code128 => ZxBarcodeFormat::Code128,
            CompactPDF417 => ZxBarcodeFormat::CompactPDF417,
            DataBar => ZxBarcodeFormat::DataBar,
            DataBarExp => ZxBarcodeFormat::DataBarExp,
            DataBarExpStk => ZxBarcodeFormat::DataBarExpStk,
            DataBarLtd => ZxBarcodeFormat::DataBarLtd,
            DataBarOmni => ZxBarcodeFormat::DataBarOmni,
            DataBarStk => ZxBarcodeFormat::DataBarStk,
            DataBarStkOmni => ZxBarcodeFormat::DataBarStkOmni,
            DataMatrix => ZxBarcodeFormat::DataMatrix,
            DXFilmEdge => ZxBarcodeFormat::DXFilmEdge,
            EAN2 => ZxBarcodeFormat::EAN2,
            EAN5 => ZxBarcodeFormat::EAN5,
            EAN8 => ZxBarcodeFormat::EAN8,
            EAN13 => ZxBarcodeFormat::EAN13,
            EANUPC => ZxBarcodeFormat::EANUPC,
            ISBN => ZxBarcodeFormat::ISBN,
            ITF => ZxBarcodeFormat::ITF,
            MaxiCode => ZxBarcodeFormat::MaxiCode,
            MicroPDF417 => ZxBarcodeFormat::MicroPDF417,
            MicroQRCode => ZxBarcodeFormat::MicroQRCode,
            PDF417 => ZxBarcodeFormat::PDF417,
            PZN => ZxBarcodeFormat::PZN,
            QRCode => ZxBarcodeFormat::QRCode,
            QRCodeModel1 => ZxBarcodeFormat::QRCodeModel1,
            QRCodeModel2 => ZxBarcodeFormat::QRCodeModel2,
            RMQRCode => ZxBarcodeFormat::RMQRCode,
            UPCA => ZxBarcodeFormat::UPCA,
            UPCE => ZxBarcodeFormat::UPCE,
        }
    }
}

impl TryFrom<ZxBarcodeFormat> for BarcodeFormat {
    type Error = Error;

    fn try_from(value: ZxBarcodeFormat) -> Result<Self> {
        use ZxBarcodeFormat::*;
        match value {
            Aztec => Ok(BarcodeFormat::Aztec),
            AztecCode => Ok(BarcodeFormat::AztecCode),
            AztecRune => Ok(BarcodeFormat::AztecRune),
            Codabar => Ok(BarcodeFormat::Codabar),
            Code39 => Ok(BarcodeFormat::Code39),
            Code93 => Ok(BarcodeFormat::Code93),
            Code128 => Ok(BarcodeFormat::Code128),
            CompactPDF417 => Ok(BarcodeFormat::CompactPDF417),
            DataBar => Ok(BarcodeFormat::DataBar),
            DataBarExp => Ok(BarcodeFormat::DataBarExp),
            DataBarExpStk => Ok(BarcodeFormat::DataBarExpStk),
            DataBarLtd => Ok(BarcodeFormat::DataBarLtd),
            DataBarOmni => Ok(BarcodeFormat::DataBarOmni),
            DataBarStk => Ok(BarcodeFormat::DataBarStk),
            DataBarStkOmni => Ok(BarcodeFormat::DataBarStkOmni),
            DataMatrix => Ok(BarcodeFormat::DataMatrix),
            DXFilmEdge => Ok(BarcodeFormat::DXFilmEdge),
            EAN2 => Ok(BarcodeFormat::EAN2),
            EAN5 => Ok(BarcodeFormat::EAN5),
            EAN8 => Ok(BarcodeFormat::EAN8),
            EAN13 => Ok(BarcodeFormat::EAN13),
            EANUPC => Ok(BarcodeFormat::EANUPC),
            ISBN => Ok(BarcodeFormat::ISBN),
            ITF => Ok(BarcodeFormat::ITF),
            MaxiCode => Ok(BarcodeFormat::MaxiCode),
            MicroPDF417 => Ok(BarcodeFormat::MicroPDF417),
            MicroQRCode => Ok(BarcodeFormat::MicroQRCode),
            PDF417 => Ok(BarcodeFormat::PDF417),
            PZN => Ok(BarcodeFormat::PZN),
            QRCode => Ok(BarcodeFormat::QRCode),
            QRCodeModel1 => Ok(BarcodeFormat::QRCodeModel1),
            QRCodeModel2 => Ok(BarcodeFormat::QRCodeModel2),
            RMQRCode => Ok(BarcodeFormat::RMQRCode),
            UPCA => Ok(BarcodeFormat::UPCA),
            UPCE => Ok(BarcodeFormat::UPCE),
            other => Err(Error::UnsupportedFormat(format!("{}", other))),
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

    pub fn format(&self) -> Result<BarcodeFormat> {
        if let Some(format) = self.cached_format.get() {
            Ok(*format)
        } else {
            let format = self.inner.format().try_into()?;
            let _ = self.cached_format.set(format);
            Ok(format)
        }
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

#[derive(Debug)]
pub struct GrayImage<'a> {
    raw: Cow<'a, [u8]>,
    width: u32,
    height: u32,
    format: ImageFormat,
}

impl<'a> GrayImage<'a> {
    pub fn new(raw: impl Into<Cow<'a, [u8]>>, width: u32, height: u32) -> Self {
        let raw = raw.into();
        if raw.len() != (width * height) as usize {
            panic!("the length of the `raw` must be equal to `width` * `height`");
        }
        Self {
            raw,
            width,
            height,
            format: ImageFormat::Lum,
        }
    }
}

impl<'a> From<&'a GrayImage<'a>> for ImageView<'a> {
    fn from(value: &'a GrayImage<'a>) -> Self {
        ImageView::from_slice(&value.raw, value.width, value.height, value.format).unwrap()
    }
}

fn decode<'a>(
    image: GrayImage<'a>,
    formats: &[BarcodeFormat],
    multi: bool,
) -> Result<Vec<Barcode>> {
    let mut read_barcodes = if formats.is_empty() {
        zxingcpp::read().formats(&[ZxBarcodeFormat::All])
    } else {
        let mut zx_formats_buf = [ZxBarcodeFormat::None; 32];
        for (i, f) in formats.iter().enumerate() {
            zx_formats_buf[i] = (*f).into();
        }
        zxingcpp::read().formats(&zx_formats_buf[..formats.len()])
    };

    if !multi {
        read_barcodes.set_max_number_of_symbols(1);
    }
    read_barcodes
        .from(Into::<ImageView>::into(&image))
        .map_err(|e| Error::DecodeError(e.to_string()))
}

pub fn decode_multiple<'a>(
    image: GrayImage<'a>,
    formats: &[BarcodeFormat],
) -> Result<Vec<DecodeResult>> {
    let barcodes = decode(image, formats, true)?;
    Ok(barcodes
        .into_iter()
        .map(DecodeResult::new)
        .collect::<Vec<_>>())
}

pub fn decode_single<'a>(
    image: GrayImage<'a>,
    formats: &[BarcodeFormat],
) -> Result<Option<DecodeResult>> {
    match decode(image, formats, false) {
        Ok(mut results) => {
            if results.is_empty() {
                Ok(None)
            } else {
                Ok(Some(DecodeResult::new(results.remove(0))))
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barcode_format_display() {
        assert_eq!(format!("{}", BarcodeFormat::DataBarExp), "DataBarExpanded");
        assert_eq!(format!("{}", BarcodeFormat::DataBarLtd), "DataBarLimited");
        assert_eq!(format!("{}", BarcodeFormat::DataBarExpStk), "DataBarExpandedStacked");
    }
}
