use stl_loader;
use std::vec::Vec;

#[derive(Debug)]
pub enum MeshError {
    UnknownFileType,
    ScanError(stl_loader::StlError)
}

pub fn load_file(filename: &str) -> Result<Vec<stl_loader::Face>, MeshError> {
    println!("Loading file {}", filename);

    if filename.ends_with(".stl") {
        match stl_loader::read_stl_file(filename) {
            Ok(o) => Ok(o),
            Err(e) => Err(MeshError::ScanError(e))
        }
    } else {
        Err(MeshError::UnknownFileType)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reject_not_stl() {
        assert!(crate::load_file("test.3ds").is_err(), "Must reject 3ds file");
        assert!(crate::load_file("teststl").is_err(), "Must stl must be prefixed by a . in filename");
    }
}
