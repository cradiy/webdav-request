use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IoError(std::io::Error),
    ReqeustError(reqwest::Error),
    DeError(quick_xml::DeError),
    ResponseError(StatusCode),
    Utf8Error(std::str::Utf8Error)
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqeustError(value)
    }
}
impl From<quick_xml::DeError> for Error {
    fn from(value: quick_xml::DeError) -> Self {
        Self::DeError(value)
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(arg0) => arg0.fmt(f),
            Self::ReqeustError(arg0) =>arg0.fmt(f),
            Self::DeError(arg0) => arg0.fmt(f),
            Self::ResponseError(arg) => arg.fmt(f),
            Error::Utf8Error(arg) => arg.fmt(f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(arg0) => arg0.fmt(f),
            Self::ReqeustError(arg0) =>arg0.fmt(f),
            Self::DeError(arg0) => arg0.fmt(f),
            Self::ResponseError(arg) => arg.fmt(f),
            Self::Utf8Error(arg) => arg.fmt(f)
        }
    }
}
impl std::error::Error for Error {
    
}