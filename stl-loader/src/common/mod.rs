use scanner_rust::ScannerError;

/// An STL error wraps all the different types of errors you can get back from reading
/// an STL file.
#[derive(Debug)]
pub enum StlError {
    /// An error that came from the scanner
    ScanError(ScannerError),
    /// No header line in the ASCII STL file
    MissingHeader,
    /// Trailing characters after the name in the header (solid) line of the ASCII STL file
    TooMuchInHeader,
    /// File terminated with no endsolid
    NoEndSolid,
    /// Saw this text in a place where it was not expected
    UnknownSymbol(String),
    /// Solid names at start and end of the file were different
    MissmatchedSolidNames(Option<String>, Option<String>),
    /// File terminated when something else was expected
    UnexpectedEndOfFile(String),
}