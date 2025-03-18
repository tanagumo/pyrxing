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
            format: format!("{}", value.format()),
        }
    }
}

#[derive(Debug)]
enum ImageSource<'a, 'b> {
    Path(String),
    ImageProtocol(&'b Bound<'a, PyAny>),
}

fn get_image_source<'a, 'b>(obj: &'b Bound<'a, PyAny>) -> Result<ImageSource<'a, 'b>> {
    let type_obj = obj.get_type();
    let type_name = type_obj.name()?;

    if type_name == "str" {
        Ok(ImageSource::Path(obj.extract::<String>()?))
    } else {
        let mut conform_to_image_protocol = true;
        if !obj.hasattr("mode")?
            || !obj.hasattr("width")?
            || !obj.hasattr("height")?
            || !obj.hasattr("tobytes")?
            || !obj.hasattr("convert")?
        {
            conform_to_image_protocol = false;
        } else {
            if !obj.getattr("tobytes")?.is_callable() || !obj.getattr("convert")?.is_callable() {
                conform_to_image_protocol = false;
            }
        }
        if !conform_to_image_protocol {
            return Err(error::Error::Python(
                pyo3::exceptions::PyValueError::new_err(format!(
                    "value must be either str or conform to ImageProtocol"
                )),
            ));
        }
        Ok(ImageSource::ImageProtocol(obj))
    }
}

fn load_from_image<'a, 'b>(obj: &'b Bound<'a, PyAny>) -> Result<GrayImage> {
    let mode = obj.getattr("mode")?.extract::<PyBackedStr>()?;
    let width = obj.getattr("width")?.extract::<u32>()?;
    let height = obj.getattr("height")?.extract::<u32>()?;

    let result: Result<_> = match &*mode {
        "L" => Ok(obj.call_method0("tobytes")?.extract::<Vec<u8>>()?),
        "RGB" | "RGBA" | "P" => Ok(obj
            .call_method1("convert", ("L",))?
            .call_method0("tobytes")?
            .extract::<Vec<u8>>()?),
        _ => {
            let message = format!("The specified image has an unsupported mode({}). Only grayscale, RGB(A), or palette mode images are supported.", mode);
            Err(error::ImageError::UnsupportedMode(message).into())
        }
    };

    let gray_image = GrayImage::from_raw(width, height, result?).unwrap();
    Ok(gray_image)
}

fn gen_gray_image<'a, 'b>(image_source: ImageSource<'a, 'b>) -> Result<GrayImage> {
    match image_source {
        ImageSource::Path(s) => {
            let file = File::open(&s)?;
            let file_size = file.metadata()?.len();

            let sizes = [file_size as usize, 10_000_000];
            let capacity = sizes.iter().min().unwrap();

            let buf_reader = BufReader::with_capacity(*capacity, file);
            let reader = ImageReader::new(buf_reader).with_guessed_format()?;
            Ok(reader.decode()?.to_luma8())
        }
        ImageSource::ImageProtocol(pyobj) => Ok(load_from_image(pyobj)?),
    }
}

enum Decoded {
    Single(Option<DecodeResult>),
    Multi(Vec<DecodeResult>),
}

struct _BarcodeFormat(String);

impl From<_BarcodeFormat> for reader_core::BarcodeFormat {
    fn from(value: _BarcodeFormat) -> Self {
        use reader_core::BarcodeFormat as BF;
        match value.0.as_str() {
            "AZTEC" => BF::AZTEC,
            "CODABAR" => BF::CODABAR,
            "CODE_39" => BF::CODE_39,
            "CODE_93" => BF::CODE_93,
            "CODE_128" => BF::CODE_128,
            "DATA_MATRIX" => BF::DATA_MATRIX,
            "EAN_8" => BF::EAN_8,
            "EAN_13" => BF::EAN_13,
            "ITF" => BF::ITF,
            "MAXICODE" => BF::MAXICODE,
            "PDF_417" => BF::PDF_417,
            "QR_CODE" => BF::QR_CODE,
            "MICRO_QR_CODE" => BF::MICRO_QR_CODE,
            "RECTANGULAR_MICRO_QR_CODE" => BF::RECTANGULAR_MICRO_QR_CODE,
            "RSS_14" => BF::RSS_14,
            "RSS_EXPANDED" => BF::RSS_EXPANDED,
            "TELEPEN" => BF::TELEPEN,
            "UPC_A" => BF::UPC_A,
            "UPC_E" => BF::UPC_E,
            "UPC_EAN_EXTENSION" => BF::UPC_EAN_EXTENSION,
            "DX_FILM_EDGE" => BF::DXFilmEdge,
            _ => BF::UNSUPORTED_FORMAT,
        }
    }
}

fn decode(obj: &Bound<'_, PyAny>, formats: Option<Vec<String>>, multi: bool) -> Result<Decoded> {
    use reader_core::{decode_multiple, decode_single, BarcodeFormat};

    let image_source = get_image_source(obj)?;
    let gray_image = gen_gray_image(image_source)?;

    let formats = formats
        .unwrap_or_else(|| vec![])
        .into_iter()
        .map(|bf| BarcodeFormat::from(_BarcodeFormat(bf)))
        .filter(|bf| bf != &BarcodeFormat::UNSUPORTED_FORMAT)
        .collect::<Vec<_>>();

    let result = if multi {
        decode_multiple(gray_image, formats.as_slice()).map(|result| {
            Decoded::Multi(
                result
                    .into_iter()
                    .map(DecodeResult::from)
                    .collect::<Vec<_>>(),
            )
        })
    } else {
        decode_single(gray_image, formats.as_slice())
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
