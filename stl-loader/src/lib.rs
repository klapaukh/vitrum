//!  The stl-loader reads binary and ASCII STL files and converts them into `Faces`.

use std::io::Read;
use std::fs::File;
use std::str;
use std::vec::Vec;

mod ascii;
mod binary;
mod common;

pub use common::StlError;
pub use geometry::{Face, Vector3D};

/// Read in and parse and STL file
///
/// # Arguments
///
/// * `filename` - The path to the file to read. This can be either an ASCII or binary STL.
pub fn read_stl_file(filename: &str) -> Result<Vec<Face<f32>>, StlError> {
    //  Check to make sure that it is not a binary file first
    let mut f = File::open(filename)?;
    let mut buf = [0;6];
    f.read_exact(&mut buf)?;
    let header = str::from_utf8(&buf)?;
    if header == "solid " { // Technically the binary header could start like that. But it shouldn't.
        ascii::read_file_ascii(filename)
    } else {
        binary::read_file_binary(&mut f)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(2, 2);
    }
}
