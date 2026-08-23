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

#[pyclass(extends = PyBaseError, name = "UnsupportedFormatError", module = "pyrxing")]
#[derive(Debug)]
pub(crate) struct PyUnsupportedFormatError {
    /// The rejected barcode format name.
    #[pyo3(get)]
    format: String,
}

#[pymethods]
impl PyUnsupportedFormatError {
    #[new]
    fn new(message: String, format: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PyBaseError { message }).add_subclass(Self { format })
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

fn unsupported_format_error(format: String) -> PyErr {
    let message = format!("`{}` is not a supported barcode format", format);
    raise(PyClassInitializer::from(PyBaseError { message }).add_subclass(PyUnsupportedFormatError { format }))
}

fn image_error(message: String) -> PyErr {
    raise(PyClassInitializer::from(PyBaseError { message }).add_subclass(PyImageError))
}

#[derive(Debug)]
pub(crate) enum ImageError {
    Decode(image::ImageError),
    UnsupportedMode(String),
    Invalid(String),
}

/// A failure this module detects on its own.
#[derive(Debug)]
pub(crate) enum ModuleError {
    Image(ImageError),
    Io(std::io::Error),
    Type(String),
    UnsupportedFormat(String),
    BarcodeDecode(String),
}

/// A failure of this module, or an exception raised by Python itself.
#[derive(Debug)]
pub(crate) enum Error {
    Own(ModuleError),
    /// An exception raised by Python itself, carried through unchanged.
    Python(PyErr),
}

impl From<ModuleError> for Error {
    fn from(value: ModuleError) -> Self {
        Self::Own(value)
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Self::Own(ModuleError::Image(value))
    }
}

impl From<PyErr> for Error {
    fn from(value: PyErr) -> Self {
        Self::Python(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Own(ModuleError::Io(value))
    }
}

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        Self::Own(ModuleError::Image(ImageError::Decode(value)))
    }
}

impl From<reader_core::Error> for Error {
    fn from(value: reader_core::Error) -> Self {
        Self::Own(match value {
            reader_core::Error::InvalidInput(message) => {
                ModuleError::Image(ImageError::Invalid(message))
            }
            reader_core::Error::DecodeError(message) => ModuleError::BarcodeDecode(message),
            reader_core::Error::UnsupportedFormat(name) => ModuleError::UnsupportedFormat(name),
        })
    }
}

impl From<ModuleError> for PyErr {
    fn from(value: ModuleError) -> Self {
        use ModuleError::*;

        match value {
            Image(e) => match e {
                ImageError::Decode(e) => image_error(e.to_string()),
                ImageError::UnsupportedMode(e) => image_error(e),
                ImageError::Invalid(e) => image_error(e),
            },
            Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => pyo3::exceptions::PyFileNotFoundError::new_err(e),
                _ => e.into(),
            },
            Type(message) => pyo3::exceptions::PyTypeError::new_err(message),
            UnsupportedFormat(format) => unsupported_format_error(format),
            BarcodeDecode(e) => barcode_decode_error(e),
        }
    }
}

impl From<Error> for PyErr {
    fn from(value: Error) -> Self {
        match value {
            Error::Own(e) => e.into(),
            Error::Python(e) => e,
        }
    }
}
