use std::io::{
    Error as IoError,
    ErrorKind as IoErrorKind,
};
use http::Error as HttpError;
use std::string::FromUtf8Error;

pub(crate) enum HandlerError {
    IoError(IoError),
    HttpError(HttpError),
    FromUtf8Error(FromUtf8Error),
}

impl From<IoError> for HandlerError {
    fn from(e: IoError) -> Self {
        HandlerError::IoError(e)
    }
}

impl From<HandlerError> for IoError {
    fn from(e: HandlerError) -> Self {
        match e {
            HandlerError::IoError(e) => e,
            _ => IoError::from(IoErrorKind::Other),
        }
    }
}

impl From<HttpError> for HandlerError {
    fn from(e: HttpError) -> Self { 
        HandlerError::HttpError(e)
    }
}

impl From<FromUtf8Error> for HandlerError {
    fn from(e: FromUtf8Error) -> Self { 
        HandlerError::FromUtf8Error(e)
    }
}
