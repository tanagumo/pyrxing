mod error;

use std::fs::File;
use std::io::BufReader;

use image::ImageReader;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyBytes;

use reader_core::{self, GrayImage};

type Result<T> = std::result::Result<T, error::Error>;

#[pyclass(module = "pyrxing")]
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

impl From<reader_core::DecodeResult> for DecodeResult {
    fn from(value: reader_core::DecodeResult) -> Self {
        Self {
            text: value.text().to_owned(),
            points: value
                .points()
                .iter()
                .map(|p| Point { x: p.x(), y: p.y() })
                .collect::<Vec<_>>(),
            format: format!("{}", value.format()),
        }
    }
}

#[derive(Debug)]
enum ImageSource<'a> {
    Path(PyBackedStr),
    ImageProtocolData {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
    ImageProtocolDataView {
        _holder: Bound<'a, PyAny>,
        data: Bound<'a, PyAny>,
        width: u32,
        height: u32,
    },
}

impl<'a, 'b: 'a> TryInto<GrayImage<'a>> for &'a ImageSource<'b> {
    type Error = error::Error;

    fn try_into(self) -> std::result::Result<GrayImage<'a>, Self::Error> {
        match self {
            ImageSource::Path(path) => {
                let file = File::open(&**path)?;
                let file_size = file.metadata()?.len();

                let sizes = [file_size as usize, 10_000_000];
                let capacity = sizes.iter().min().unwrap();

                let buf_reader = BufReader::with_capacity(*capacity, file);
                let reader = ImageReader::new(buf_reader).with_guessed_format()?;
                let image_buffer = reader.decode()?.to_luma8();
                let width = image_buffer.width();
                let height = image_buffer.height();
                Ok(GrayImage::new(image_buffer.into_raw(), width, height))
            }
            ImageSource::ImageProtocolData {
                data,
                width,
                height,
            } => Ok(GrayImage::new(data, *width, *height)),
            ImageSource::ImageProtocolDataView {
                // Holds the __array_interface__ object to maintain its reference count.
                // Python's __array_interface__ property may creates temporary view objects
                // that get GC'd immediately if not referenced, invalidating data.
                _holder,
                data,
                width,
                height,
            } => {
                let py_bytes = data.downcast::<PyBytes>().unwrap().as_bytes();
                Ok(GrayImage::new(py_bytes, *width, *height))
            }
        }
    }
}

fn get_image_source<'a>(obj: &Bound<'a, PyAny>) -> Result<ImageSource<'a>> {
    let type_obj = obj.get_type();
    let type_name = type_obj.name()?;

    if type_name == "str" {
        Ok(ImageSource::Path(obj.extract::<PyBackedStr>()?))
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
            return Err(error::Error::Python(
                pyo3::exceptions::PyValueError::new_err(
                    "value must be either str or conform to ImageProtocol",
                ),
            ));
        }

        let mode = obj.getattr("mode")?.extract::<PyBackedStr>()?;

        let mut converted_opt = None;
        match &*mode {
            "L" => {}
            "RGB" | "RGBA" | "P" | "1" => {
                converted_opt = Some(
                    obj.call_method1("convert", ("L",))?
                        .extract::<Bound<'a, PyAny>>()?,
                );
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

        if target_obj.hasattr("__array_interface__")? {
            let ai = target_obj.getattr("__array_interface__")?;
            let type_str = ai.get_item("typestr")?.extract::<PyBackedStr>()?;
            if type_str == "|u1" {
                let data = ai.get_item("data")?;
                return Ok(ImageSource::ImageProtocolDataView {
                    _holder: ai,
                    data,
                    width,
                    height,
                });
            }
        }

        let data = target_obj.call_method0("tobytes")?.extract::<Vec<u8>>()?;
        Ok(ImageSource::ImageProtocolData {
            data,
            width,
            height,
        })
    }
}

enum Decoded {
    Single(Option<DecodeResult>),
    Multi(Vec<DecodeResult>),
}

struct _BarcodeFormat(String);

impl TryFrom<_BarcodeFormat> for reader_core::BarcodeFormat {
    type Error = String;
    fn try_from(value: _BarcodeFormat) -> std::result::Result<Self, Self::Error> {
        use reader_core::BarcodeFormat as BF;
        let v = match value.0.as_str() {
            "Aztec" => BF::Aztec,
            "Codabar" => BF::Codabar,
            "Code39" => BF::Code39,
            "Code93" => BF::Code93,
            "Code128" => BF::Code128,
            "DataBar" => BF::DataBar,
            "DataBarExpanded" => BF::DataBarExpanded,
            "DataBarLimited" => BF::DataBarLimited,
            "DataMatrix" => BF::DataMatrix,
            "EAN8" => BF::EAN8,
            "EAN13" => BF::EAN13,
            "ITF" => BF::ITF,
            "MaxiCode" => BF::MaxiCode,
            "PDF417" => BF::PDF417,
            "QRCode" => BF::QRCode,
            "UPCA" => BF::UPCA,
            "UPCE" => BF::UPCE,
            "MicroQRCode" => BF::MicroQRCode,
            "RMQRCode" => BF::RMQRCode,
            "DXFilmEdge" => BF::DXFilmEdge,
            other => {
                return Err(format!("`{}` is not supported", other));
            }
        };
        Ok(v)
    }
}

fn decode(obj: &Bound<'_, PyAny>, formats: Option<Vec<String>>, multi: bool) -> Result<Decoded> {
    let image_source = &get_image_source(obj)?;
    let gray_image = (image_source).try_into()?;

    let formats = formats
        .unwrap_or_else(|| vec![])
        .into_iter()
        .filter_map(|bf| TryInto::<reader_core::BarcodeFormat>::try_into(_BarcodeFormat(bf)).ok())
        .collect::<Vec<_>>();

    let result = if multi {
        reader_core::decode_multiple(gray_image, formats.as_slice()).map(|result| {
            Decoded::Multi(
                result
                    .into_iter()
                    .map(DecodeResult::from)
                    .collect::<Vec<_>>(),
            )
        })
    } else {
        reader_core::decode_single(gray_image, formats.as_slice())
            .map(|opt| Decoded::Single(opt.map(DecodeResult::from)))
    };

    result.map_err(|e| error::Error::Decode(e.to_string()))
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
    m.add_class::<error::PyBarcodeDecodeError>()?;
    m.add_class::<error::PyImageError>()?;
    m.add_class::<DecodeResult>()?;
    m.add_function(wrap_pyfunction!(read_barcode, m)?)?;
    m.add_function(wrap_pyfunction!(read_barcodes, m)?)?;
    Ok(())
}
