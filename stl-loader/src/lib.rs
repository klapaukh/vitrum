use scanner_rust::{Scanner, ScannerError};
use std::vec::Vec;

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

impl std::convert::From<ScannerError> for StlError {
    fn from(error: ScannerError) -> Self {
        StlError::ScanError(error)
    }
}

#[derive(Debug, Clone)]
pub struct Face {
    pub normal: Vector3D,
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3D {
    pub fn new(x: f32, y: f32,z: f32) -> Vector3D {
        Vector3D {
            x,
            y,
            z
        }
    }

    pub fn read<T: std::io::Read>(scan: &mut Scanner<T>) -> Result<Vector3D, StlError> {
        Ok(Vector3D {
            x: ensure_f32(scan)?,
            y: ensure_f32(scan)?,
            z: ensure_f32(scan)?
        })
    }
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

/// Read in and parse and STL file
///
/// # Arguments
///
/// * `filename` - The path to the file to read. This can be either an ASCII or binary STL.
pub fn read_stl_file(filename: &str) -> Result<Vec<Face>, StlError> {
    println!("Reading STL file {}", filename);
    let mut scan = Scanner::scan_path(filename)?;

    let name = read_header_ascii(&mut scan)?;
    println!("Reading solid with name {:?}", name);
    let mut faces: Vec<Face> = Vec::new();
    loop {
        let result = read_body_ascii(&mut scan, &name)?;
        if let Some(f) = result {
            faces.push(f);
        } else {
            break;
        }
    }
    Ok(faces)
}

/// Read the header of the ASCII STL file and return the name of the solid
///
/// # Arguments
/// * `scan` - A scanner to the ascii file
fn read_header_ascii<T: std::io::Read>(scan: &mut Scanner<T>) -> Result<Option<String>, StlError> {
    let line = match scan.next_line()? {
        None => return Err(StlError::MissingHeader),
        Some(header) => header
    };

    let mut iter = line.split_whitespace();
    match iter.next() {
        None => return Err(StlError::MissingHeader),
        Some("solid") => (),
        Some(_) => return Err(StlError::MissingHeader)
    };

    let name = match iter.next() {
        Some(name) => Ok(Some(name.to_owned())),
        None => Ok(None)
    };

    match iter.next() {
        Some(_) => return Err(StlError::TooMuchInHeader),
        None => ()
    };

    return name;
}

fn read_body_ascii<T: std::io::Read>(scan: &mut Scanner<T>, name: &Option<String>) -> Result<Option<Face>, StlError>  {
    let result = scan.next()?;
    match result {
        None => Err(StlError::NoEndSolid),
        Some(s) => {
            if s == String::from("endsolid") {
                let end_name = scan.next()?;
                if *name != end_name {
                    Err(StlError::MissmatchedSolidNames(name.clone(), end_name))
                }else {
                    Ok(None)
                }
            } else if s == String::from("facet") {
                ensure_next(scan, String::from("normal"))?;
                let normal = Vector3D::read(scan)?;
                ensure_next(scan, String::from("outer"))?;
                ensure_next(scan, String::from("loop"))?;
                ensure_next(scan, String::from("vertex"))?;
                let a = Vector3D::read(scan)?;
                ensure_next(scan, String::from("vertex"))?;
                let b = Vector3D::read(scan)?;
                ensure_next(scan, String::from("vertex"))?;
                let c = Vector3D::read(scan)?;
                ensure_next(scan, String::from("endloop"))?;
                ensure_next(scan, String::from("endfacet"))?;
                Ok(Some(Face {
                    normal,
                    a,
                    b,
                    c
                }))
            } else {
                Err(StlError::UnknownSymbol(s))
            }
        }
    }
}

/// Ensure that the next token in the scanner stream is the one you expect.
///
/// # Arguments
///
/// * `scan` - The scanner to read from
/// * `expected` - The string which the next token must match
fn ensure_next<T: std::io::Read>(scan: &mut Scanner<T>, expected: String) -> Result<(), StlError> {
    let result = scan.next()?;
    match result {
        None => Err(StlError::UnexpectedEndOfFile(expected)),
        Some(s) =>  if s == expected {
                        Ok(())
                    } else {
                        Err(StlError::UnknownSymbol(s))
                    }
    }
}

fn ensure_f32<T: std::io::Read>(scan: &mut Scanner<T>) -> Result<f32, StlError> {
    let result = scan.next_f32()?;
    match result {
        None => Err(StlError::UnexpectedEndOfFile(String::from("<float>"))),
        Some(s) => Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use scanner_rust::Scanner;

    #[test]
    fn test_read_header() {
        let mut scan = Scanner::scan_slice("solid dog");
        let name = crate::read_header_ascii(&mut scan).unwrap().unwrap();
        assert_eq!(name, "dog");

        let mut scan = Scanner::scan_slice("solid ");
        let name = crate::read_header_ascii(&mut scan).unwrap();
        assert!(name.is_none());

        let mut scan = Scanner::scan_slice("solid \n\tfacet normal");
        let name = crate::read_header_ascii(&mut scan).unwrap();
        assert!(name.is_none());

        let mut scan = Scanner::scan_slice("");
        let name = crate::read_header_ascii(&mut scan);
        assert!(name.is_err());

        let mut scan = Scanner::scan_slice("fish fish");
        let name = crate::read_header_ascii(&mut scan);
        assert!(name.is_err());

        let mut scan = Scanner::scan_slice("solid dog bowl");
        let name = crate::read_header_ascii(&mut scan);
        assert!(name.is_err());
    }

    #[test]
    fn test_ensure_next() {
        let mut scan = Scanner::scan_slice("dog fish cat");
        assert!(crate::ensure_next(&mut scan, String::from("dog")).is_ok());
        assert!(crate::ensure_next(&mut scan, String::from("crate")).is_err());
        assert!(crate::ensure_next(&mut scan, String::from("cat")).is_ok());
        assert!(crate::ensure_next(&mut scan, String::from("cat")).is_err());
     }
}
