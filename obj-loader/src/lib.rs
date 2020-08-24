//!  The stl-loader reads binary and ASCII STL files and converts them into `Faces`.

use std::str;
use std::vec::Vec;
use scanner_rust::Scanner;

mod errors;
pub  use errors::ObjError;

mod helpers;
use helpers::{ObjVertex, ObjNormal, ObjParam, ObjFace, from_homogenous};

pub use geometry::{Face, Vec3};

/// Read in and parse an ascii OBJ file
///
/// # Arguments
///
/// * `filename` - The path to the file to read. This must be either an ASCII OBJ file.
pub fn read_obj_file(filename: &str) -> Result<Vec<Face>, ObjError> {
    //  Check to make sure that it is not a binary file first
    let mut scan = Scanner::scan_path(filename)?;

    // Create lists to fill populate with data from the file
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut textures = Vec::new();
    let mut faces = Vec::new();
    while let Some(line) = scan.next_line()? {
        process_line(line.trim(), &mut vertices, &mut normals, &mut textures, &mut faces)?
    }

    // We now have the list of faces, but currently as indexes into other arrays.
    // Convert them now into actual faces
    let mut model = Vec::with_capacity(faces.len());

    for face in faces {
        let av = get_element_from(face.av, &vertices[..]);
        let bv = get_element_from(face.bv, &vertices[..]);
        let cv = get_element_from(face.cv, &vertices[..]);

        model.push(Face::from_points(from_homogenous(&av), from_homogenous(&bv), from_homogenous(&cv)));
    }

    Ok(model)
}

pub fn get_element_from<T: Copy>(index: usize, data: &[T]) -> T {
    if index > 0 {
        // 1 based index going forwards
        data[index as usize -  1]
    } else {
        panic!("Cannot have an index of {} in a face!", index)
    }
}

fn process_line(line: &str, vertices: &mut Vec<ObjVertex>, normals: &mut Vec<ObjNormal>,
                    textures: &mut Vec<ObjParam>, faces: &mut Vec<ObjFace>) -> Result<(), ObjError>{
    if line.is_empty() || line.starts_with('#') {
        // Empty and comment lines can be ignored
        return Ok(());
    }

    // This line should have some interesting content.
    // Lines are read as words, split by spaces
    let mut token_iter = line.split_whitespace();

    // Read the first token to determine the meaning of the line
    match token_iter.next() {
        Some(line_type) => match line_type{
            "v" => process_vertex(&mut token_iter, vertices)?,
            "vn" => process_normal(&mut token_iter, normals)?,
            "vt" => process_texture(&mut token_iter, textures)?,
            "f" => process_face(&mut token_iter, faces, vertices.len(), textures.len(), normals.len())?,
            "l" => (), // we are ignoring lines for now
            "g" => (), // we are ignoring group for now
            "o" => (), // we are ignoring objects for now
            "s" => (), // we are ignoring smooth shading instructions for now
            "mtllib" => (), // we are ignoring materials files for now
            "usemtl" => (), // we are ignoring materials for now
            other => return Err(ObjError::UnknownCommand(other.to_string()))
        },
        None => return Ok(())
    };

    Ok(())

}

fn process_vertex(token_iter: &mut str::SplitWhitespace, vertices: &mut Vec<ObjVertex>) -> Result<(), ObjError> {
    vertices.push(ObjVertex::new(
        ensure(token_iter.next(), "vertex x")?,
        ensure(token_iter.next(), "vertex y")?,
        ensure(token_iter.next(), "vertex z")?,
        maybe(token_iter.next(), 1.0, "vertex w")?
        ));

    Ok(())
}

fn process_normal(token_iter: &mut str::SplitWhitespace, normals: &mut Vec<ObjNormal>) -> Result<(), ObjError> {
    normals.push(ObjNormal::new(
        ensure(token_iter.next(), "normal x")?,
        ensure(token_iter.next(), "normal y")?,
        ensure(token_iter.next(), "normal z")?,
    ));
    Ok(())
}

fn process_texture(token_iter: &mut str::SplitWhitespace, textures: &mut Vec<ObjParam>) -> Result<(), ObjError> {
    textures.push(ObjParam {
        u: ensure(token_iter.next(), "texture x")?,
        v: maybe(token_iter.next(), 0.0, "texture y")?,
        w: maybe(token_iter.next(), 0.0, "texture z")?,
    });

    Ok(())
}

fn process_face(token_iter: &mut str::SplitWhitespace, faces: &mut Vec<ObjFace>,
                vertex_size: usize, texture_size: usize, normal_size: usize)  -> Result<(), ObjError> {
    let triples: Vec<(usize, usize, usize)> = token_iter.map(to_triple).map(|b| b.and_then(|a| Ok(to_positive_triple(a, vertex_size, texture_size, normal_size)))).collect::<Result<Vec<(usize, usize, usize)>, ObjError>>()?;

    if triples.len() < 3 {
        return Err(ObjError::NotEnoughVerticesInFace(triples.len()));
    }

    for index in 1 .. triples.len() - 1 {
        let a = triples[0];
        let b = triples[index];
        let c = triples[index + 1];

        faces.push(ObjFace {
            av: a.0,
            at: a.1,
            an: a.2,
            bv: b.0,
            bt: b.1,
            bn: b.2,
            cv: c.0,
            ct: c.1,
            cn: c.2,
        });
    }
    Ok(())
}

fn to_triple(token: &str) -> Result<(isize, isize, isize), ObjError> {
    let mut parts = token.split('/');
    let vertex: isize = ensure(parts.next(), "to_triple vertex")?;
    let texture: isize = maybe(parts.next(),0, "to_triple texture")?;
    let normal: isize = maybe(parts.next(),0, "to_triple normal")?;
    assert!(parts.next().is_none());
    Ok((vertex,texture,normal))
}

fn to_positive_triple(fields: (isize, isize, isize), vertex_size: usize, texture_size: usize, normal_size: usize) -> (usize, usize, usize) {
    let vertex = if fields.0 < 0 {
        vertex_size as isize + fields.0 + 1
    } else {
        fields.0
    };
    let texture: isize = if fields.1 < 0 {
        texture_size as isize + fields.1 + 1
    } else {
        fields.1
    };
    let normal: isize = if fields.2 < 0 {
        normal_size as isize + fields.2 + 1
    } else {
        fields.2
    };
    (vertex as usize, texture as usize, normal as usize)
}

/// Read the next token as an <F>, returning an appropriate error if it fails.
///
/// # Arguments
/// * `scan` - The scanner to read.
fn ensure<F: str::FromStr>(token: Option<&str>, context: &str) -> Result<F, ObjError> {
    match token {
        None => Err(ObjError::UnexpectedEndOfFile(String::from("Expected a value"))),
        Some(s) => match s.parse() {
            Ok(v) => Ok(v),
            Err(_) => {println!("Error!!"); Err(ObjError::NotAFloat(format!("Failed to ensure convert {} in {}", s, context)))}
        }
    }
}

/// Read the next token as an <F> if there is one. Else return the default.
///
/// # Arguments
/// * `scan` - The scanner to read.
/// * `default` - The value to return if there are no more values to read
fn maybe<F: str::FromStr>(token: Option<&str>, default: F, context: &str) -> Result<F, ObjError> {
    match token {
        None => Ok(default),
        Some("") => Ok(default),
        Some(s) => match s.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ObjError::NotAFloat(format!("Failed to maybe convert {} in {}", s, context)))
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ensure() {
        assert!(super::ensure::<f32>(None, "test").is_err());
        assert_eq!(5.5, super::ensure(Some("5.5"), "test").unwrap());
        assert_ne!(5.5, super::ensure(Some("5"), "test").unwrap());
    }

    #[test]
    fn test_maybe() {
        assert_eq!(6, super::maybe(None, 6, "test").unwrap());
        assert_eq!(5.5, super::maybe(Some("5.5"), 3.0, "test").unwrap());
        assert_ne!(3, super::maybe(Some("5"),3, "test").unwrap());
    }

    #[test]
    fn test_vertex() {
        let mut v = Vec::new();
        let mut t = Vec::new();
        let mut n = Vec::new();
        let mut f = Vec::new();
        assert!(super::process_line("v 1 2 3", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(v.len() == 1);
        assert!(t.is_empty());
        assert!(n.is_empty());
        assert!(f.is_empty());
        let d = &v[0];
        assert_eq!(1.0, d.x);
        assert_eq!(2.0, d.y);
        assert_eq!(3.0, d.z);
        assert_eq!(1.0, d.w);
    }

    #[test]
    fn test_face() {
        let mut v = Vec::new();
        let mut t = Vec::new();
        let mut n = Vec::new();
        let mut f = Vec::new();
        assert!(super::process_line("f 1 2 3", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(super::process_line("f 1//2 2//3 3//4", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(super::process_line("f 1/1 2/2 3/3", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(super::process_line("f 1/1/2 2/3/4 3/5/6", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(v.is_empty());
        assert!(t.is_empty());
        assert!(n.is_empty());
        assert_eq!(4, f.len());
    }

    #[test]
    fn test_normal() {
        let mut v = Vec::new();
        let mut t = Vec::new();
        let mut n = Vec::new();
        let mut f = Vec::new();
        assert!(super::process_line("vn 1 2 3", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(v.is_empty());
        assert!(t.is_empty());
        assert!(n.len() == 1);
        assert!(f.is_empty());
        let d = &n[0];
        assert_eq!(1.0, d.x);
        assert_eq!(2.0, d.y);
        assert_eq!(3.0, d.z);
    }

    #[test]
    fn test_texture() {
        let mut v = Vec::new();
        let mut t = Vec::new();
        let mut n = Vec::new();
        let mut f = Vec::new();
        assert!(super::process_line("vt 1", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(v.is_empty());
        assert!(t.len() == 1);
        assert!(n.is_empty());
        assert!(f.is_empty());
        let d = &t[0];
        assert_eq!(1.0, d.u);
        assert_eq!(0.0, d.v);
        assert_eq!(0.0, d.w);
    }

    #[test]
    fn test_divide_face() {
        let mut v = Vec::new();
        let mut t = Vec::new();
        let mut n = Vec::new();
        let mut f = Vec::new();
        assert!(super::process_line("f 1 2 3 4", &mut v, &mut n, &mut t, &mut f).is_ok());
        assert!(v.is_empty());
        assert!(t.is_empty());
        assert!(n.is_empty());
        assert_eq!(2, f.len());
    }

    #[test]
    fn test_get_element_from() {
        let data = vec![1,2,3,4,5];
        for i in 1 .. 6  {
            assert_eq!(super::get_element_from(i, &data[..]), i);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_panic() {
        let data = vec![1,2,3,4,5];
        super::get_element_from(0, &data[..]);
    }

    #[test]
    fn test_read_triples() {
        let line = "1/2/3";
        assert_eq!(super::to_triple(line).unwrap(), (1,2,3));
        assert_eq!(super::to_triple(line).and_then(|a| Ok(super::to_positive_triple(a, 5, 5, 5))).unwrap(), (1,2,3));
        let line = "1/-2/3";
        assert_eq!(super::to_triple(line).and_then(|a| Ok(super::to_positive_triple(a, 5, 5, 5))).unwrap(), (1,4,3));
    }
}
