use std::{borrow::Cow, cell::OnceCell, fmt::Display, str::FromStr};

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

impl FromStr for BarcodeFormat {
    type Err = Error;

    /// Parses the name [`Display`] produces for a variant.
    fn from_str(s: &str) -> Result<Self> {
        use BarcodeFormat::*;
        let v = match s {
            "Aztec" => Aztec,
            "AztecCode" => AztecCode,
            "AztecRune" => AztecRune,
            "Codabar" => Codabar,
            "Code39" => Code39,
            "Code93" => Code93,
            "Code128" => Code128,
            "CompactPDF417" => CompactPDF417,
            "DataBar" => DataBar,
            "DataBarExpanded" => DataBarExp,
            "DataBarExpandedStacked" => DataBarExpStk,
            "DataBarLimited" => DataBarLtd,
            "DataBarOmni" => DataBarOmni,
            "DataBarStacked" => DataBarStk,
            "DataBarStackedOmni" => DataBarStkOmni,
            "DataMatrix" => DataMatrix,
            "DXFilmEdge" => DXFilmEdge,
            "EAN2" => EAN2,
            "EAN5" => EAN5,
            "EAN8" => EAN8,
            "EAN13" => EAN13,
            "EANUPC" => EANUPC,
            "ISBN" => ISBN,
            "ITF" => ITF,
            "MaxiCode" => MaxiCode,
            "MicroPDF417" => MicroPDF417,
            "MicroQRCode" => MicroQRCode,
            "PDF417" => PDF417,
            "PZN" => PZN,
            "QRCode" => QRCode,
            "QRCodeModel1" => QRCodeModel1,
            "QRCodeModel2" => QRCodeModel2,
            "RMQRCode" => RMQRCode,
            "UPCA" => UPCA,
            "UPCE" => UPCE,
            other => return Err(Error::UnsupportedFormat(other.to_owned())),
        };
        Ok(v)
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

/// Largest side length the zxing reader accepts.
const MAX_SIDE: u32 = 65535;

/// Image size within the reader limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    /// Builds an image size from a width and a height.
    ///
    /// Returns [`Error::InvalidInput`] unless both sides stay within [`MAX_SIDE`].
    pub fn new(width: u32, height: u32) -> Result<Self> {
        if width > MAX_SIDE || height > MAX_SIDE {
            return Err(Error::InvalidInput(format!(
                "the maximum image width and height is {}, got {}x{}",
                MAX_SIDE, width, height
            )));
        }
        Ok(Self { width, height })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Luminance byte count for an image of this size.
    pub fn pixel_count(&self) -> usize {
        // Both sides stay within MAX_SIDE, so the product never exceeds
        // 65535 * 65535 and fits a 32-bit `usize` as well.
        self.width as usize * self.height as usize
    }
}

#[derive(Debug)]
pub struct GrayImage<'a> {
    raw: Cow<'a, [u8]>,
    dimensions: Dimensions,
    format: ImageFormat,
}

impl<'a> GrayImage<'a> {
    /// Builds a grayscale image from one luminance byte per pixel.
    ///
    /// Returns [`Error::InvalidInput`] unless `raw` holds exactly
    /// [`Dimensions::pixel_count`] bytes.
    pub fn new(raw: impl Into<Cow<'a, [u8]>>, dimensions: Dimensions) -> Result<Self> {
        let raw = raw.into();
        if raw.len() != dimensions.pixel_count() {
            return Err(Error::InvalidInput(format!(
                "the length of the `raw` must be equal to `width` * `height`: expected {}, got {}",
                dimensions.pixel_count(),
                raw.len()
            )));
        }
        Ok(Self {
            raw,
            dimensions,
            format: ImageFormat::Lum,
        })
    }
}

impl<'a> TryFrom<&'a GrayImage<'a>> for ImageView<'a> {
    type Error = Error;

    fn try_from(value: &'a GrayImage<'a>) -> Result<Self> {
        ImageView::from_slice(
            &value.raw,
            value.dimensions.width(),
            value.dimensions.height(),
            value.format,
        )
        .map_err(|e| Error::InvalidInput(e.to_string()))
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
        let zx_formats = formats
            .iter()
            .map(|f| ZxBarcodeFormat::from(*f))
            .collect::<Vec<_>>();
        zxingcpp::read().formats(zx_formats.as_slice())
    };

    if !multi {
        read_barcodes.set_max_number_of_symbols(1);
    }
    read_barcodes
        .from(ImageView::try_from(&image)?)
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

    /// Pixels of a rendered barcode along with its size.
    type Rendered = (Vec<u8>, u32, u32);

    /// Builds a grayscale image from `pixels` sized `width` by `height`.
    fn gray_image(pixels: &[u8], width: u32, height: u32) -> GrayImage<'_> {
        GrayImage::new(pixels, Dimensions::new(width, height).unwrap()).unwrap()
    }

    /// Renders `text` as a barcode and returns the pixels along with the size.
    fn render(format: BarcodeFormat, text: &str) -> Rendered {
        let barcode = zxingcpp::create(format.into()).from_str(text).unwrap();
        let image = barcode.to_image_with(&zxingcpp::write().scale(3)).unwrap();
        (image.data(), image.width() as u32, image.height() as u32)
    }

    /// Places `tiles` side by side on a white canvas, framed and separated by `margin` pixels.
    fn tile_horizontally(tiles: &[Rendered], margin: u32) -> Rendered {
        let height = tiles.iter().map(|(_, _, h)| h).max().unwrap() + margin * 2;
        let width = tiles.iter().map(|(_, w, _)| w + margin).sum::<u32>() + margin;
        let mut canvas = vec![0xFFu8; (width * height) as usize];
        let mut left = margin;
        for (pixels, tile_width, tile_height) in tiles {
            for row in 0..*tile_height {
                let from = (row * tile_width) as usize;
                let onto = ((row + margin) * width + left) as usize;
                canvas[onto..onto + *tile_width as usize]
                    .copy_from_slice(&pixels[from..from + *tile_width as usize]);
            }
            left += tile_width + margin;
        }
        (canvas, width, height)
    }

    const ALL_FORMATS: [BarcodeFormat; 35] = {
        use BarcodeFormat::*;
        [
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
        ]
    };

    #[test]
    fn test_barcode_format_display() {
        assert_eq!(format!("{}", BarcodeFormat::DataBarExp), "DataBarExpanded");
        assert_eq!(format!("{}", BarcodeFormat::DataBarLtd), "DataBarLimited");
        assert_eq!(format!("{}", BarcodeFormat::DataBarExpStk), "DataBarExpandedStacked");
    }

    #[test]
    fn test_barcode_format_names_are_distinct() {
        let mut names = ALL_FORMATS.map(|f| f.to_string()).to_vec();
        names.sort();
        names.dedup();

        assert_eq!(names.len(), ALL_FORMATS.len());
    }

    #[test]
    fn test_barcode_format_parses_back_from_its_display_name() {
        for format in ALL_FORMATS {
            let name = format.to_string();

            assert_eq!(name.parse::<BarcodeFormat>().unwrap(), format, "{}", name);
        }
    }

    #[test]
    fn test_barcode_format_rejects_an_unknown_name() {
        let error = "Bogus".parse::<BarcodeFormat>().unwrap_err();

        assert!(
            matches!(&error, Error::UnsupportedFormat(name) if name == "Bogus"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_dimensions_new_accepts_the_largest_side() {
        let dimensions = Dimensions::new(MAX_SIDE, MAX_SIDE).unwrap();

        assert_eq!(dimensions.pixel_count(), 65535usize * 65535);
    }

    #[test]
    fn test_dimensions_new_rejects_a_side_beyond_the_reader_limit() {
        let error = Dimensions::new(0x10000, 1).unwrap_err();

        assert!(
            matches!(&error, Error::InvalidInput(m) if m.contains("65535") && m.contains("65536x1")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_dimensions_new_rejects_a_side_wrapping_a_u32_product() {
        // 0x80000000 * 2 is 2^32, so a u32 multiplication would report zero pixels
        // and let an empty buffer through.
        let error = Dimensions::new(0x8000_0000, 2).unwrap_err();

        assert!(
            matches!(&error, Error::InvalidInput(m) if m.contains("65535") && m.contains("2147483648x2")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_gray_image_new_accepts_a_matching_length() {
        let image = GrayImage::new(vec![0u8; 12], Dimensions::new(4, 3).unwrap()).unwrap();

        assert_eq!(
            (image.dimensions.width(), image.dimensions.height()),
            (4, 3)
        );
    }

    #[test]
    fn test_gray_image_new_rejects_a_mismatched_length() {
        let error = GrayImage::new(vec![0u8; 11], Dimensions::new(4, 3).unwrap()).unwrap_err();

        assert!(
            matches!(&error, Error::InvalidInput(m) if m.contains("11") && m.contains("12")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_barcode_format_converts_back_from_the_zxing_format() {
        for format in ALL_FORMATS {
            let zx = ZxBarcodeFormat::from(format);

            assert_eq!(BarcodeFormat::try_from(zx).unwrap(), format, "{}", format);
        }
    }

    #[test]
    fn test_barcode_format_rejects_a_zxing_format_without_a_counterpart() {
        for zx in [
            ZxBarcodeFormat::None,
            ZxBarcodeFormat::All,
            ZxBarcodeFormat::Telepen,
            ZxBarcodeFormat::OtherBarcode,
        ] {
            let error = BarcodeFormat::try_from(zx).unwrap_err();

            assert!(
                matches!(error, Error::UnsupportedFormat(_)),
                "unexpected error for {}: {}",
                zx,
                error
            );
        }
    }

    /// Payloads rendered and read back, paired with the text and format zxing reports.
    const ROUND_TRIPS: [(BarcodeFormat, &str, &str, BarcodeFormat); 17] = {
        use BarcodeFormat::*;
        [
            (Aztec, "AZTEC_DATA", "AZTEC_DATA", Aztec),
            (Codabar, "A12345D", "A12345D", Codabar),
            (Code39, "CODE39", "CODE39", Code39),
            (Code93, "CODE93", "CODE93", Code93),
            (Code128, "Code 128 Data", "Code 128 Data", Code128),
            (ITF, "1234567890", "1234567890", ITF),
            (DataMatrix, "DataMatrix_31802", "DataMatrix_31802", DataMatrix),
            (PDF417, "PDF417_12417", "PDF417_12417", PDF417),
            (QRCode, "QR_PAYLOAD", "QR_PAYLOAD", QRCode),
            (MicroQRCode, "0ZFXY9", "0ZFXY9", MicroQRCode),
            (RMQRCode, "V4GQ68D2", "V4GQ68D2", RMQRCode),
            (EAN8, "07727931", "07727931", EAN8),
            (EAN13, "2708639496369", "2708639496369", EAN13),
            // zxing normalises a UPC-A symbol into its EAN-13 form.
            (UPCA, "041935974561", "0041935974561", EAN13),
            (MaxiCode, "MaxiCode_46753", "MaxiCode_46753", MaxiCode),
            (
                DataBar,
                "0101234567890128",
                "(01)01234567890128",
                DataBarOmni,
            ),
            (
                DataBarLtd,
                "0101234567890128",
                "(01)01234567890128",
                DataBarLtd,
            ),
        ]
    };

    #[test]
    fn test_decode_single_reads_back_a_rendered_barcode() {
        for (format, payload, text, reported) in ROUND_TRIPS {
            let (pixels, width, height) = render(format, payload);
            let image = gray_image(&pixels[..], width, height);

            let decoded = decode_single(image, &[]).unwrap().expect(payload);

            assert_eq!(decoded.text(), text, "{}", format);
            assert_eq!(decoded.format().unwrap(), reported, "{}", format);
        }
    }

    #[test]
    fn test_decode_single_returns_nothing_for_a_blank_image() {
        let image =
            GrayImage::new(vec![0xFFu8; 64 * 64], Dimensions::new(64, 64).unwrap()).unwrap();

        assert!(decode_single(image, &[]).unwrap().is_none());
    }

    #[test]
    fn test_decode_single_keeps_a_symbol_of_a_requested_format() {
        let (pixels, width, height) = render(BarcodeFormat::QRCode, "QR_PAYLOAD");
        let image = gray_image(&pixels[..], width, height);

        let decoded = decode_single(image, &[BarcodeFormat::QRCode, BarcodeFormat::EAN13]).unwrap();

        assert_eq!(decoded.unwrap().text(), "QR_PAYLOAD");
    }

    #[test]
    fn test_decode_single_drops_a_symbol_of_another_format() {
        let (pixels, width, height) = render(BarcodeFormat::QRCode, "QR_PAYLOAD");
        let image = gray_image(&pixels[..], width, height);

        let decoded = decode_single(image, &[BarcodeFormat::EAN13]).unwrap();

        assert!(decoded.is_none());
    }

    #[test]
    fn test_decode_single_accepts_more_formats_than_the_zxing_format_count() {
        let (pixels, width, height) = render(BarcodeFormat::QRCode, "QR_PAYLOAD");
        let formats = [BarcodeFormat::QRCode; 33];

        let image = gray_image(&pixels[..], width, height);
        let decoded = decode_single(image, &formats).unwrap();

        assert_eq!(decoded.unwrap().text(), "QR_PAYLOAD");
    }

    /// Renders two QR codes side by side on one canvas.
    fn two_qr_codes() -> Rendered {
        tile_horizontally(
            &[
                render(BarcodeFormat::QRCode, "LEFT_PAYLOAD"),
                render(BarcodeFormat::QRCode, "RIGHT_PAYLOAD"),
            ],
            12,
        )
    }

    #[test]
    fn test_decode_stops_at_one_symbol_outside_multi_mode() {
        let (pixels, width, height) = two_qr_codes();
        let multi = decode(gray_image(&pixels[..], width, height), &[], true);
        assert_eq!(multi.unwrap().len(), 2, "the canvas must carry two symbols");

        let single = decode(gray_image(&pixels[..], width, height), &[], false);

        assert_eq!(single.unwrap().len(), 1);
    }

    #[test]
    fn test_decode_multiple_reads_every_symbol() {
        let (pixels, width, height) = two_qr_codes();
        let image = gray_image(&pixels[..], width, height);

        let mut texts = decode_multiple(image, &[])
            .unwrap()
            .iter()
            .map(|r| r.text().to_owned())
            .collect::<Vec<_>>();
        texts.sort();

        assert_eq!(texts, ["LEFT_PAYLOAD", "RIGHT_PAYLOAD"]);
    }

    #[test]
    fn test_decode_result_points_trace_the_position_clockwise() {
        let (pixels, width, height) = render(BarcodeFormat::QRCode, "QR_PAYLOAD");
        let image = gray_image(&pixels[..], width, height);
        let decoded = decode_single(image, &[]).unwrap().unwrap();

        let position = decoded.position();
        let (top_left, bottom_right) = (position.top_left(), position.bottom_right());

        assert!(top_left.x() < bottom_right.x());
        assert!(top_left.y() < bottom_right.y());
        assert_eq!(
            decoded.points(),
            [
                top_left,
                Point::new(bottom_right.x(), top_left.y()),
                bottom_right,
                Point::new(top_left.x(), bottom_right.y()),
            ]
        );
    }
}
