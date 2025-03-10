mod error;

use std::fs::File;
use std::io::BufReader;

use image::{GrayImage, ImageReader};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;

type Result<T> = std::result::Result<T, error::Error>;

#[pyclass(module = "pyrxing")]
#[derive(Clone)]
struct Point {
    #[pyo3(get)]
    x: f32,
    #[pyo3(get)]
    y: f32,
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
                .map(|p| Point { x: p.x, y: p.y })
                .collect::<Vec<_>>(),
            format: value.format().to_owned(),
        }
    }
}

#[derive(Debug)]
enum ImageSource {
    Path(String),
    PILImage(Py<PyAny>),
}

fn get_image_source<'a>(obj: &Bound<'a, PyAny>) -> Result<ImageSource> {
    let type_obj = obj.get_type();
    let type_name = type_obj.name()?;

    if type_name == "str" {
        Ok(ImageSource::Path(obj.extract::<String>()?))
    } else {
        let module = type_obj.module()?;
        if !module.to_str()?.starts_with("PIL.") {
            return Err(error::Error::Python(
                pyo3::exceptions::PyValueError::new_err(format!(
                    "value must be either str or PIL.Image.Image"
                )),
            ));
        }
        Ok(ImageSource::PILImage(obj.clone().unbind()))
    }
}

#[derive(Debug)]
struct PILImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

fn load_from_pil_image(obj: Py<PyAny>) -> Result<GrayImage> {
    let info = Python::with_gil(|py| -> Result<PILImage> {
        let bound = obj.bind(py);
        let mode = bound.getattr("mode")?.extract::<PyBackedStr>()?;
        let width = bound.getattr("width")?.extract::<u32>()?;
        let height = bound.getattr("height")?.extract::<u32>()?;

        match &*mode {
            "L" | "RGB" | "RGBA" | "P" => {
                let data = if &*mode != "L" {
                    bound
                        .call_method1("convert", ("L",))?
                        .call_method0("tobytes")?
                        .extract::<Vec<u8>>()?
                } else {
                    bound.call_method0("tobytes")?.extract::<Vec<u8>>()?
                };
                Ok(PILImage {
                    width,
                    height,
                    data,
                })
            }
            _ => {
                let message = format!("The specified image has an unsupported mode({}). Only grayscale, RGB(A), or palette mode images are supported.", mode);
                Err(error::ImageError::UnsupportedMode(message).into())
            }
        }
    })?;

    let gray_image = GrayImage::from_raw(info.width, info.height, info.data).unwrap();
    Ok(gray_image)
}

fn gen_gray_image(image_source: ImageSource) -> Result<GrayImage> {
    match image_source {
        ImageSource::Path(s) => {
            let buf_reader = BufReader::new(File::open(&s)?);
            let reader = ImageReader::new(buf_reader).with_guessed_format()?;
            Ok(reader.decode()?.to_luma8())
        }
        ImageSource::PILImage(pyobj) => Ok(load_from_pil_image(pyobj)?),
    }
}

#[pyfunction]
fn decode<'a>(py: Python<'a>, obj: &Bound<'a, PyAny>) -> PyResult<Vec<DecodeResult>> {
    let image_source = get_image_source(obj)?;

    py.allow_threads(|| {
        let gray_image = gen_gray_image(image_source)?;
        let result = reader_core::decode_multiple(gray_image)
            .map_err(|e| error::Error::Decode(e.to_string()))?;
        Ok(result
            .into_iter()
            .map(DecodeResult::from)
            .collect::<Vec<_>>())
    })
}

#[pymodule]
fn pyrxing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<error::PyDecodingError>()?;
    m.add_class::<error::PyImageError>()?;
    m.add_class::<DecodeResult>()?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}
