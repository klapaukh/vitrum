//! Errors for OBJ file reading

use scanner_rust::ScannerError;
use std::io::Error;
use std::num::TryFromIntError;
use std::str::Utf8Error;

/// An OBJ error wraps all the different types of errors you can get back from reading
/// an OBJ file.
#[derive(Debug)]
pub enum ObjError {
    /// Error from converting bytes to UTF-8
    UTF8Error(Utf8Error),
    /// An error that came from IO
    IOError(Error),
    /// An error that came from the scanner
    ScanError(ScannerError),
    /// Found no faces
    NoFacesFound,
    /// There are more faces than can be allocated in memory (usize < u32)
    TooManyFacesError,
    /// Line started with an unknown command
    UnknownCommand(String),
    /// File terminated when something else was expected
    UnexpectedEndOfFile(String),
    /// Expect a float, but did not get one
    NotAFloat(String),
    // Face without enough vertices to specify it (< 3)
    NotEnoughVerticesInFace(usize)
}

impl std::convert::From<ScannerError> for ObjError {
    fn from(error: ScannerError) -> Self {
        ObjError::ScanError(error)
    }
}

impl std::convert::From<Error> for ObjError {
    fn from(error: Error) -> Self {
        ObjError::IOError(error)
    }
}

impl std::convert::From<Utf8Error> for ObjError {
    fn from(error: Utf8Error) -> Self {
        ObjError::UTF8Error(error)
    }
}

impl std::convert::From<TryFromIntError> for ObjError {
    fn from(_error: TryFromIntError) -> Self {
        ObjError::TooManyFacesError
    }
}

