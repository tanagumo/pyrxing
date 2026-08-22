mod error;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use image::ImageReader;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyBytes, PyString};

use reader_core::{self, GrayImage};

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
                    .map_err(|e| error::Error::Decode(e.to_string()))?
            ),
        })
    }
}

#[derive(Debug)]
enum ImageSource<'a> {
    Path(PathBuf),
    ImageProtocolData {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
    ImageProtocolDataView {
        data: Bound<'a, PyBytes>,
        width: u32,
        height: u32,
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
                let reader = ImageReader::new(buf_reader).with_guessed_format()?;
                let image_buffer = reader.decode()?.to_luma8();
                let width = image_buffer.width();
                let height = image_buffer.height();
                Ok(GrayImage::new(image_buffer.into_raw(), width, height)?)
            }
            ImageSource::ImageProtocolData {
                data,
                width,
                height,
            } => Ok(GrayImage::new(data, *width, *height)?),
            ImageSource::ImageProtocolDataView {
                data,
                width,
                height,
            } => Ok(GrayImage::new(data.as_bytes(), *width, *height)?),
        }
    }
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
            return Err(error::Error::type_error(
                "image must be a str, os.PathLike or ImageProtocol value",
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

        // Borrowing the `bytes` from `tobytes` avoids copying the pixels into a `Vec`.
        // Any other buffer still goes through `extract`, at the cost of that copy.
        match target_obj.call_method0("tobytes")?.cast_into::<PyBytes>() {
            Ok(data) => Ok(ImageSource::ImageProtocolDataView {
                data,
                width,
                height,
            }),
            Err(err) => Ok(ImageSource::ImageProtocolData {
                data: err.into_inner().extract::<Vec<u8>>()?,
                width,
                height,
            }),
        }
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
                .map_err(|_| error::Error::Python(error::unsupported_format_error(name)))
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
