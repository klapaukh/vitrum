pub use geometry::{Face, Vector3D};
use std::vec::Vec;

mod common;
mod ascii;

pub use common::StlError;

/// Read in and parse and STL file
///
/// # Arguments
///
/// * `filename` - The path to the file to read. This can be either an ASCII or binary STL.
pub fn read_stl_file(filename: &str) -> Result<Vec<Face>, StlError> {
    //  Check to make sure that it is not a binary file first
    ascii::read_file_ascii(filename)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(2, 2);
    }
}
