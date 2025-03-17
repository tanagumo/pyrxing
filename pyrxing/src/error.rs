use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass(extends = PyException, name = "BarcodeDecodeError", module = "pyrxing")]
#[derive(Debug)]
pub(crate) struct PyBarcodeDecodeError {
    #[pyo3(get)]
    message: String,
}

#[pymethods]
impl PyBarcodeDecodeError {
    #[new]
    fn new(message: String) -> Self {
        PyBarcodeDecodeError { message }
    }
}

impl std::fmt::Display for PyBarcodeDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BarcodeDecodeError({})", self.message)
    }
}

impl std::error::Error for PyBarcodeDecodeError {}

impl From<PyBarcodeDecodeError> for PyErr {
    fn from(e: PyBarcodeDecodeError) -> Self {
        PyErr::new::<PyBarcodeDecodeError, _>(e.message)
    }
}

#[pyclass(extends = PyException, name = "ImageError", module = "pyrxing")]
#[derive(Debug)]
pub(crate) struct PyImageError {
    #[pyo3(get)]
    message: String,
}

#[pymethods]
impl PyImageError {
    #[new]
    fn new(message: String) -> Self {
        PyImageError { message }
    }
}

impl std::fmt::Display for PyImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageError({})", self.message)
    }
}

impl std::error::Error for PyImageError {}

impl From<PyImageError> for PyErr {
    fn from(e: PyImageError) -> Self {
        PyErr::new::<PyImageError, _>(e.message)
    }
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

impl From<Error> for PyErr {
    fn from(value: Error) -> Self {
        use Error::*;

        match value {
            Image(e) => match e {
                ImageError::Decode(e) => PyErr::new::<PyImageError, _>(e.to_string()),
                ImageError::UnsupportedMode(e) => PyErr::new::<PyImageError, _>(e.to_string()),
            },
            Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => pyo3::exceptions::PyFileNotFoundError::new_err(e),
                _ => e.into(),
            },
            Python(e) => e,
            Decode(e) => PyErr::new::<PyBarcodeDecodeError, _>(e.to_string()),
        }
    }
}
