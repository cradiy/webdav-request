use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    StdError(std::io::Error),
    RequestError(reqwest::Error),
    DeError(quick_xml::DeError),
    ResponseError(StatusCode),
    Utf8Error(std::str::Utf8Error)
}

impl Error {
    pub fn is_std_err(&self) -> bool {
        match self {
            Self::StdError(_) => true,
            _ => false
        }
    }

    pub fn is_request_err(&self) -> bool {
        match self {
            Self::RequestError(_) => true,
            _ => false
        }
    }
    pub fn is_de_err(&self) -> bool {
        match self {
            Self::DeError(_) => true,
            _ => false
        }
    }
    pub fn is_response_err(&self) -> bool {
        match self {
            Self::ResponseError(_) => true,
            _ => false
        }
    }

    pub fn is_invalid_utf8_err(&self) -> bool {
        match self {
            Self::Utf8Error(_) => true,
            _ => false
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::StdError(value)
    }
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
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
            Self::StdError(arg0) => arg0.fmt(f),
            Self::RequestError(arg0) =>arg0.fmt(f),
            Self::DeError(arg0) => arg0.fmt(f),
            Self::ResponseError(arg) => arg.fmt(f),
            Error::Utf8Error(arg) => arg.fmt(f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdError(arg0) => arg0.fmt(f),
            Self::RequestError(arg0) =>arg0.fmt(f),
            Self::DeError(arg0) => arg0.fmt(f),
            Self::ResponseError(arg) => arg.fmt(f),
            Self::Utf8Error(arg) => arg.fmt(f)
        }
    }
}
impl std::error::Error for Error {
    
}