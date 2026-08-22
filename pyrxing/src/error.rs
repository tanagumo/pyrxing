use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::PyClassInitializer;

/// Base class of every exception the module defines.
#[pyclass(extends = PyException, name = "Error", module = "pyrxing", subclass)]
#[derive(Debug)]
pub(crate) struct PyBaseError {
    message: String,
}

#[pymethods]
impl PyBaseError {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }

    fn __str__(&self) -> &str {
        &self.message
    }
}

#[pyclass(extends = PyBaseError, name = "BarcodeDecodeError", module = "pyrxing")]
#[derive(Debug)]
pub(crate) struct PyBarcodeDecodeError;

#[pymethods]
impl PyBarcodeDecodeError {
    #[new]
    fn new(message: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PyBaseError { message }).add_subclass(Self)
    }
}

#[pyclass(extends = PyBaseError, name = "ImageError", module = "pyrxing")]
#[derive(Debug)]
pub(crate) struct PyImageError;

#[pymethods]
impl PyImageError {
    #[new]
    fn new(message: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PyBaseError { message }).add_subclass(Self)
    }
}

/// Raises `initializer` as a Python exception.
fn raise<T>(initializer: PyClassInitializer<T>) -> PyErr
where
    T: pyo3::PyClass,
{
    Python::attach(|py| match Bound::new(py, initializer) {
        Ok(value) => PyErr::from_value(value.into_any()),
        Err(e) => e,
    })
}

fn barcode_decode_error(message: String) -> PyErr {
    raise(PyClassInitializer::from(PyBaseError { message }).add_subclass(PyBarcodeDecodeError))
}

fn image_error(message: String) -> PyErr {
    raise(PyClassInitializer::from(PyBaseError { message }).add_subclass(PyImageError))
}

#[derive(Debug)]
pub(crate) enum ImageError {
    Decode(image::ImageError),
    UnsupportedMode(String),
}

#[derive(Debug)]
pub(crate) enum Error {
    Image(ImageError),
    Io(std::io::Error),
    Python(PyErr),
    Decode(String),
}

impl Error {
    pub(crate) fn value_error(message: impl Into<String>) -> Self {
        Self::Python(pyo3::exceptions::PyValueError::new_err(message.into()))
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}

impl From<PyErr> for Error {
    fn from(value: PyErr) -> Self {
        Self::Python(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        Self::Image(ImageError::Decode(value))
    }
}

impl From<reader_core::Error> for Error {
    fn from(value: reader_core::Error) -> Self {
        match value {
            reader_core::Error::InvalidInput(message) => {
                Self::Python(pyo3::exceptions::PyValueError::new_err(message))
            }
            other => Self::Decode(other.to_string()),
        }
    }
}

impl From<Error> for PyErr {
    fn from(value: Error) -> Self {
        use Error::*;

        match value {
            Image(e) => match e {
                ImageError::Decode(e) => image_error(e.to_string()),
                ImageError::UnsupportedMode(e) => image_error(e),
            },
            Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => pyo3::exceptions::PyFileNotFoundError::new_err(e),
                _ => e.into(),
            },
            Python(e) => e,
            Decode(e) => barcode_decode_error(e),
        }
    }
}
