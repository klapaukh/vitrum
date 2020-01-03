use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::convert::TryInto;

use super::common::StlError;

use geometry::{Face, Vector3D};

/// Read a binary STL file and return the list of faces.
/// Binary STL files are assumed to be written in little endian byte order.
pub fn read_file_binary(file: &mut File) -> Result<Vec<Face>, StlError> {
    // Skip the header and jump to position 80
    file.seek(SeekFrom::Start(80))?;
    let mut buff = [0;4];
    file.read_exact(&mut buff)?;
    let n_faces = u32::from_le_bytes(buff);
    let n_faces: usize = n_faces.try_into()?;

    println!("Reading {} faces", n_faces);
    // Make sure there are actually faces to read
    if n_faces < 1 {
        return Err(StlError::NoFacesFound)
    }

    let mut faces: Vec<Face> = Vec::with_capacity(n_faces);

    for _ in 1..n_faces {
        // Each face is 12, 4 byte reals + a 2 byte uint16
        let mut face_buffer = [0; 12 * 4 + 2];
        file.read_exact(&mut face_buffer)?;

        let normal = read_vec(&face_buffer, 0);
        let f = if normal.is_zero() {
            Face::from_points(
                read_vec(&face_buffer, 1),
                read_vec(&face_buffer, 2),
                read_vec(&face_buffer, 3))
        } else {
            Face::new(normal,
                read_vec(&face_buffer, 1),
                read_vec(&face_buffer, 2),
                read_vec(&face_buffer, 3))
        };
        // We ignore the last 2 bytes for now as we don't need them for the shape

        faces.push(f);
    }

    Ok(faces)
}

fn read_vec(buff: &[u8;50], offset: usize) -> Vector3D {
    let offset = offset * 4 * 3;
    Vector3D::new(f32::from_le_bytes(buff[offset..offset + 4].try_into().expect("Must be 4 bytes")),
                  f32::from_le_bytes(buff[offset + 4..offset + 8].try_into().expect("Must be 4 bytes")),
                  f32::from_le_bytes(buff[offset + 8..offset + 12].try_into().expect("Must be 4 bytes")))
}
