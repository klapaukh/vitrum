use scanner_rust::ScannerError;
use std::io::Error;
use std::num::TryFromIntError;
use std::str::Utf8Error;

/// An STL error wraps all the different types of errors you can get back from reading
/// an STL file.
#[derive(Debug)]
pub enum StlError {
    /// Error from converting bytes to UTF-8
    UTF8Error(Utf8Error),
    /// An error that came from IO
    IOError(Error),
    /// An error that came from the scanner
    ScanError(ScannerError),
    /// No header line in the ASCII STL file
    MissingHeader,
    /// Trailing characters after the name in the header (solid) line of the ASCII STL file
    TooMuchInHeader,
    /// File terminated with no endsolid
    NoEndSolid,
    /// Found no faces
    NoFacesFound,
    /// There are more faces than can be allocated in memory (usize < u32)
    TooManyFacesError,
    /// Saw this text in a place where it was not expected
    UnknownSymbol(String),
    /// Solid names at start and end of the file were different
    MissmatchedSolidNames(Option<String>, Option<String>),
    /// File terminated when something else was expected
    UnexpectedEndOfFile(String),
}

impl std::convert::From<ScannerError> for StlError {
    fn from(error: ScannerError) -> Self {
        StlError::ScanError(error)
    }
}

impl std::convert::From<Error> for StlError {
    fn from(error: Error) -> Self {
        StlError::IOError(error)
    }
}

impl std::convert::From<Utf8Error> for StlError {
    fn from(error: Utf8Error) -> Self {
        StlError::UTF8Error(error)
    }
}

impl std::convert::From<TryFromIntError> for StlError {
    fn from(_error: TryFromIntError) -> Self {
        StlError::TooManyFacesError
    }
}

