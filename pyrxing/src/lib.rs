mod error;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use image::{DynamicImage, ImageDecoder, ImageReader};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyBytes, PyString};

use reader_core::{self, Dimensions, GrayImage};

type Result<T> = std::result::Result<T, error::Error>;

#[pyclass(module = "pyrxing", skip_from_py_object)]
#[derive(Clone)]
struct Point {
    #[pyo3(get)]
    x: i32,
    #[pyo3(get)]
    y: i32,
}

#[pymethods]
impl Point {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Point(x={}, y={})", self.x, self.y))
    }
}

#[pyclass(module = "pyrxing")]
struct DecodeResult {
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    points: Vec<Point>,
    #[pyo3(get)]
    format: String,
}

impl TryFrom<reader_core::DecodeResult> for DecodeResult {
    type Error = error::Error;

    fn try_from(value: reader_core::DecodeResult) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            text: value.text().to_owned(),
            points: value
                .points()
                .iter()
                .map(|p| Point { x: p.x(), y: p.y() })
                .collect::<Vec<_>>(),
            format: format!(
                "{}",
                value
                    .format()
                    .map_err(|e| error::Error::BarcodeDecode(e.to_string()))?
            ),
        })
    }
}

/// Pixels owned by Rust, or borrowed from a Python `bytes`.
#[derive(Debug)]
enum PixelBytes<'a> {
    Owned(Vec<u8>),
    Borrowed(Bound<'a, PyBytes>),
}

impl AsRef<[u8]> for PixelBytes<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            PixelBytes::Owned(data) => data,
            PixelBytes::Borrowed(data) => data.as_bytes(),
        }
    }
}

#[derive(Debug)]
enum ImageSource<'a> {
    Path(PathBuf),
    Pixels {
        data: PixelBytes<'a>,
        dimensions: Dimensions,
    },
}

impl<'a, 'b: 'a> TryInto<GrayImage<'a>> for &'a ImageSource<'b> {
    type Error = error::Error;

    fn try_into(self) -> std::result::Result<GrayImage<'a>, Self::Error> {
        match self {
            ImageSource::Path(path) => {
                let file = File::open(path)?;
                let file_size = file.metadata()?.len();

                let sizes = [file_size as usize, 10_000_000];
                let capacity = sizes.iter().min().unwrap();

                let buf_reader = BufReader::with_capacity(*capacity, file);
                let decoder = ImageReader::new(buf_reader)
                    .with_guessed_format()?
                    .into_decoder()?;

                // The header alone carries the size, so an oversized image is turned away
                // before the pixels get decoded and converted below.
                let (width, height) = decoder.dimensions();
                let dimensions = Dimensions::new(width, height)?;

                let image_buffer = DynamicImage::from_decoder(decoder)?.into_luma8();
                Ok(GrayImage::new(image_buffer.into_raw(), dimensions)?)
            }
            ImageSource::Pixels { data, dimensions } => {
                Ok(GrayImage::new(data.as_ref(), *dimensions)?)
            }
        }
    }
}

/// Reads the pixels out of `tobytes`.
///
/// Borrowing the `bytes` avoids copying the pixels into a `Vec`. Any other buffer
/// still goes through `extract`, at the cost of that copy.
fn get_pixel_bytes<'a>(obj: &Bound<'a, PyAny>) -> Result<PixelBytes<'a>> {
    match obj.call_method0("tobytes")?.cast_into::<PyBytes>() {
        Ok(data) => Ok(PixelBytes::Borrowed(data)),
        Err(err) => Ok(PixelBytes::Owned(err.into_inner().extract::<Vec<u8>>()?)),
    }
}

fn get_array_source<'a>(obj: &Bound<'a, PyAny>) -> Result<ImageSource<'a>> {
    let array_interface = obj.getattr("__array_interface__")?;
    let type_str = array_interface
        .get_item("typestr")?
        .extract::<PyBackedStr>()?;
    let is_uint8 = type_str.len() == 3
        && matches!(type_str.as_bytes()[0], b'|' | b'<' | b'>' | b'=')
        && &type_str[1..] == "u1";
    if !is_uint8 {
        return Err(error::ImageError::Invalid(format!(
            "array dtype must be uint8, got {type_str}"
        ))
        .into());
    }

    let shape = array_interface.get_item("shape")?.extract::<Vec<usize>>()?;
    let (height, width) = match shape.as_slice() {
        [height, width] => (*height, *width),
        [height, width, 1] => (*height, *width),
        [_, _, channels] => {
            return Err(error::ImageError::Invalid(format!(
                "array must be single-channel grayscale, got {} channels",
                channels
            ))
            .into());
        }
        _ => {
            return Err(error::ImageError::Invalid(format!(
                "array shape must be HxW or HxWx1, got {shape:?}"
            ))
            .into());
        }
    };

    if height == 0 || width == 0 {
        return Err(error::ImageError::Invalid(
            "array width and height must be greater than zero".to_string(),
        )
        .into());
    }
    let height = u32::try_from(height)
        .map_err(|_| error::ImageError::Invalid("array height is too large".to_string()))?;
    let width = u32::try_from(width)
        .map_err(|_| error::ImageError::Invalid("array width is too large".to_string()))?;
    let dimensions = Dimensions::new(width, height)?;

    Ok(ImageSource::Pixels {
        data: get_pixel_bytes(obj)?,
        dimensions,
    })
}

fn get_image_source<'a>(obj: &Bound<'a, PyAny>) -> Result<ImageSource<'a>> {
    // `os.fspath` accepts a str as well as any `__fspath__` implementation.
    if obj.is_instance_of::<PyString>() || obj.hasattr("__fspath__")? {
        Ok(ImageSource::Path(obj.extract::<PathBuf>()?))
    } else {
        let mut conform_to_image_protocol = true;
        if !obj.hasattr("mode")?
            || !obj.hasattr("width")?
            || !obj.hasattr("height")?
            || !obj.hasattr("tobytes")?
            || !obj.hasattr("convert")?
            || !obj.hasattr("load")?
        {
            conform_to_image_protocol = false;
        } else {
            if !obj.getattr("tobytes")?.is_callable()
                || !obj.getattr("convert")?.is_callable()
                || !obj.getattr("load")?.is_callable()
            {
                conform_to_image_protocol = false;
            }
        }
        if !conform_to_image_protocol {
            if obj.hasattr("__array_interface__")? {
                return get_array_source(obj);
            }
            return Err(error::Error::Type(
                "image must be a str, os.PathLike or ImageProtocol value or a uint8 array"
                    .to_string(),
            ));
        }

        let mode = obj.getattr("mode")?.extract::<PyBackedStr>()?;

        let mut converted_opt = None;
        match &*mode {
            "L" => {}
            "RGB" | "RGBA" | "P" | "1" => {
                converted_opt = Some(obj.call_method1("convert", ("L",))?);
            }
            _ => {
                let message = format!(
                    "The specified image has an unsupported mode({}). Only grayscale, RGB(A), or palette mode images are supported.",
                    mode
                );
                return Err(error::ImageError::UnsupportedMode(message).into());
            }
        }

        let target_obj: &Bound<'a, PyAny> = match &converted_opt {
            Some(v) => {
                v.call_method0("load")?;
                v
            }
            None => obj,
        };

        let width = target_obj.getattr("width")?.extract::<u32>()?;
        let height = target_obj.getattr("height")?.extract::<u32>()?;
        let dimensions = Dimensions::new(width, height)?;

        Ok(ImageSource::Pixels {
            data: get_pixel_bytes(target_obj)?,
            dimensions,
        })
    }
}

enum Decoded {
    Single(Option<DecodeResult>),
    Multi(Vec<DecodeResult>),
}

fn decode(obj: &Bound<'_, PyAny>, formats: Option<Vec<String>>, multi: bool) -> Result<Decoded> {
    let image_source = &get_image_source(obj)?;
    let gray_image = (image_source).try_into()?;

    let formats = formats
        .unwrap_or_else(|| vec![])
        .into_iter()
        .map(|name| {
            name.parse::<reader_core::BarcodeFormat>()
                .map_err(|_| error::Error::UnsupportedFormat(name))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if multi {
        reader_core::decode_multiple(gray_image, formats.as_slice())
            .map_err(error::Error::from)
            .and_then(|result| {
                result
                    .into_iter()
                    .map(|r| r.try_into())
                    .collect::<std::result::Result<Vec<DecodeResult>, _>>()
                    .map(Decoded::Multi)
            })
    } else {
        reader_core::decode_single(gray_image, formats.as_slice())
            .map_err(error::Error::from)
            .and_then(|opt| {
                opt.map(|r| r.try_into())
                    .transpose()
                    .map(Decoded::Single)
            })
    }
}

#[pyfunction]
#[pyo3(signature = (image, formats = None))]
fn read_barcode(
    image: &Bound<'_, PyAny>,
    formats: Option<Vec<String>>,
) -> PyResult<Option<DecodeResult>> {
    decode(image, formats, false)
        .map(|decoded| match decoded {
            Decoded::Single(r) => r,
            _ => unreachable!(),
        })
        .map_err(PyErr::from)
}

#[pyfunction]
#[pyo3(signature = (image, formats = None))]
fn read_barcodes(
    image: &Bound<'_, PyAny>,
    formats: Option<Vec<String>>,
) -> PyResult<Vec<DecodeResult>> {
    decode(image, formats, true)
        .map(|decoded| match decoded {
            Decoded::Multi(results) => results,
            _ => unreachable!(),
        })
        .map_err(PyErr::from)
}

#[pymodule]
fn pyrxing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<error::PyBaseError>()?;
    m.add_class::<error::PyBarcodeDecodeError>()?;
    m.add_class::<error::PyImageError>()?;
    m.add_class::<error::PyUnsupportedFormatError>()?;
    m.add_class::<DecodeResult>()?;
    m.add_function(wrap_pyfunction!(read_barcode, m)?)?;
    m.add_function(wrap_pyfunction!(read_barcodes, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crc32(bytes: &[u8]) -> u32 {
        let mut crc = 0xFFFF_FFFFu32;
        for byte in bytes {
            crc ^= *byte as u32;
            for _ in 0..8 {
                crc = if crc & 1 == 0 {
                    crc >> 1
                } else {
                    (crc >> 1) ^ 0xEDB8_8320
                };
            }
        }
        !crc
    }

    /// Builds a grayscale PNG sized `width` by `height`, holding an `IHDR` chunk alone.
    fn png_without_pixels(width: u32, height: u32) -> Vec<u8> {
        let mut ihdr = Vec::with_capacity(17);
        ihdr.extend_from_slice(b"IHDR");
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        // 8 bits per sample, grayscale, deflate, adaptive filtering, no interlacing
        ihdr.extend_from_slice(&[8, 0, 0, 0, 0]);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        bytes.extend_from_slice(&13u32.to_be_bytes());
        bytes.extend_from_slice(&ihdr);
        bytes.extend_from_slice(&crc32(&ihdr).to_be_bytes());
        // An empty `IDAT` lets the header parse succeed while leaving no pixels to decode.
        for name in [b"IDAT", b"IEND"] {
            bytes.extend_from_slice(&0u32.to_be_bytes());
            bytes.extend_from_slice(name);
            bytes.extend_from_slice(&crc32(name).to_be_bytes());
        }
        bytes
    }

    fn write_temp_file(name: &str, bytes: &[u8]) -> PathBuf {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn test_path_decodes_a_stored_barcode() {
        let source = ImageSource::Path(PathBuf::from("../assets/test_qr_code.png"));

        let image: GrayImage<'_> = (&source).try_into().unwrap();
        let decoded = reader_core::decode_single(image, &[]).unwrap().unwrap();

        assert_eq!(decoded.text(), "https://demo.net/demo/7809");
    }

    #[test]
    fn test_path_rejects_a_side_beyond_the_reader_limit_before_decoding() {
        let path = write_temp_file("pyrxing_oversized_side.png", &png_without_pixels(0x10000, 1));
        let source = ImageSource::Path(path);

        let result: Result<GrayImage<'_>> = (&source).try_into();

        // The pixels are missing, so decoding would fail first unless the side is
        // checked against the reader limit beforehand.
        let error = result.unwrap_err();
        assert!(
            matches!(&error, error::Error::Image(error::ImageError::Invalid(m)) if m.contains("65535")),
            "unexpected error: {error:?}"
        );
    }
}
