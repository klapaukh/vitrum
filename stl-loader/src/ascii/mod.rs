//! Functions for reading ASCII STL files

use geometry::{Face, Vector3D};
use scanner_rust::Scanner;
use super::common::StlError;

pub fn read_file_ascii(filename: &str) -> Result<Vec<Face>, StlError> {
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

/// Read a single face from the body of an ASCII STL file
///
/// # Arguments
///
/// * `scan` - scanner to read the data from
/// * `name` - the name of the solid being read
///
/// # Returns
///
/// * Err - if there was a reading error
/// * Ok(None) - if there were no more faces in the file and the end of file marker was reached
/// * OK(Some(face)) - if there was a face read
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
                let normal = read_vector3d(scan)?;
                ensure_next(scan, String::from("outer"))?;
                ensure_next(scan, String::from("loop"))?;
                ensure_next(scan, String::from("vertex"))?;
                let a = read_vector3d(scan)?;
                ensure_next(scan, String::from("vertex"))?;
                let b = read_vector3d(scan)?;
                ensure_next(scan, String::from("vertex"))?;
                let c = read_vector3d(scan)?;
                ensure_next(scan, String::from("endloop"))?;
                ensure_next(scan, String::from("endfacet"))?;
                Ok(Some(
                    if normal.is_zero() {
                        Face::from_points(a, b, c)
                    } else {
                        Face::new(normal, a, b, c)
                    }
                ))
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

pub fn read_vector3d<T: std::io::Read>(scan: &mut Scanner<T>) -> Result<Vector3D, StlError> {
    Ok(Vector3D {
        x: ensure_f32(scan)?,
        y: ensure_f32(scan)?,
        z: ensure_f32(scan)?
    })
}


#[cfg(test)]
mod tests {
    use scanner_rust::Scanner;

    #[test]
    fn test_read_header() {
        let mut scan = Scanner::scan_slice("solid dog");
        let name = super::read_header_ascii(&mut scan).unwrap().unwrap();
        assert_eq!(name, "dog");

        let mut scan = Scanner::scan_slice("solid ");
        let name = super::read_header_ascii(&mut scan).unwrap();
        assert!(name.is_none());

        let mut scan = Scanner::scan_slice("solid \n\tfacet normal");
        let name = super::read_header_ascii(&mut scan).unwrap();
        assert!(name.is_none());

        let mut scan = Scanner::scan_slice("");
        let name = super::read_header_ascii(&mut scan);
        assert!(name.is_err());

        let mut scan = Scanner::scan_slice("fish fish");
        let name = super::read_header_ascii(&mut scan);
        assert!(name.is_err());

        let mut scan = Scanner::scan_slice("solid dog bowl");
        let name = super::read_header_ascii(&mut scan);
        assert!(name.is_err());
    }

    #[test]
    fn test_ensure_next() {
        let mut scan = Scanner::scan_slice("dog fish cat");
        assert!(super::ensure_next(&mut scan, String::from("dog")).is_ok());
        assert!(super::ensure_next(&mut scan, String::from("crate")).is_err());
        assert!(super::ensure_next(&mut scan, String::from("cat")).is_ok());
        assert!(super::ensure_next(&mut scan, String::from("cat")).is_err());
    }
}
