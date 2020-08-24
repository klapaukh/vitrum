//! The file loader handles reading in 3D models in different file formats and ensuring that
//! the resulting geometry is created using Virtum's data types so it can be used in the
//! renderer.

use stl_loader::StlError;
use obj_loader::ObjError;
use geometry::Face;

use std::vec::Vec;
use std::path::Path;


/// Errors that can be returned from file reading.
#[derive(Debug)]
pub enum MeshError {
    UnknownFileType,
    StlScanError(StlError),
    ObjScanError(ObjError)
}

/// Load a mesh from a file. The file extension is used to determine how to read the file.
///
/// If an extension is not recognised, the file will not be read.
///
/// # Arguments
/// * `filename` - the path to the 3D model file.
///
/// # Errors
/// Errors are triggered when:
/// * File extension is not supported
/// * File extension did not match content (even if it could have been read with a diffferent extension)
/// * The file was unable to be opened / read
/// * There is an error (or unsupported format feature) in the file
pub fn load_file(filename: &str) -> Result<Vec<Face>, MeshError> {
    println!("Loading file {}", filename);

    let extension = Path::new(filename).extension().and_then(|e| e.to_str());
    match extension {
        None => Err(MeshError::UnknownFileType),
        Some("stl") => match stl_loader::read_stl_file(filename) {
                Ok(o) => Ok(o),
                Err(e) => Err(MeshError::StlScanError(e))
            },
        Some("obj") => match obj_loader::read_obj_file(filename) {
                Ok(o) => Ok(o),
                Err(e) => Err(MeshError::ObjScanError(e))
            },
        Some(_) => Err(MeshError::UnknownFileType)
        }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reject_not_stl() {
        assert!(crate::load_file(&"test.3ds".to_owned()).is_err(), "Must reject 3ds file");
        assert!(crate::load_file(&"teststl".to_owned()).is_err(), "Must stl must be prefixed by a . in filename");
    }
}
