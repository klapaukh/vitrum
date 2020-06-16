//!  The stl-loader reads binary and ASCII STL files and converts them into `Faces`.

use std::str;
use std::vec::Vec;
use scanner_rust::Scanner;

mod errors;
pub  use errors::ObjError;

mod helpers;
use helpers::{ObjVertex, ObjNormal, ObjParam, ObjFace};

pub use geometry::{Face, Vector3D};

/// Read in and parse an ascii OBJ file
///
/// # Arguments
///
/// * `filename` - The path to the file to read. This must be either an ASCII OBJ file.
pub fn read_obj_file(filename: &str) -> Result<Vec<Face<f32>>, ObjError> {
    //  Check to make sure that it is not a binary file first
    let mut scan = Scanner::scan_path(filename)?;

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut textures = Vec::new();
    let mut faces = Vec::new();
    while let Some(line) = scan.next_line()? {
        process_line(line.trim(), &mut vertices, &mut normals, &mut textures, &mut faces)?
    }

    let model = Vec::with_capacity(faces.len());

    Ok(model)
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
            "f" => process_face(&mut token_iter, faces)?,
            "l" => (), // we are ignoring lines for now
            "g" => (), // we are ignoring group for now
            "mtllib" => (), // we are ignoring materials files for now
            "usemtl" => (), // we are ignoring materials for now
            other => return Err(ObjError::UnknownCommand(other.to_string()))
        },
        None => return Ok(())
    };

    Ok(())

}

fn process_vertex(token_iter: &mut str::SplitWhitespace, vertices: &mut Vec<ObjVertex>) -> Result<(), ObjError> {
    vertices.push(ObjVertex {
        x: ensure(token_iter.next())?,
        y: ensure(token_iter.next())?,
        z: ensure(token_iter.next())?,
        w: maybe(token_iter.next(), 1.0)?,
    });

    Ok(())
}

fn process_normal(token_iter: &mut str::SplitWhitespace, normals: &mut Vec<ObjNormal>) -> Result<(), ObjError> {
    normals.push(ObjNormal {
        x: ensure(token_iter.next())?,
        y: ensure(token_iter.next())?,
        z: ensure(token_iter.next())?,
    });
    Ok(())
}

fn process_texture(token_iter: &mut str::SplitWhitespace, textures: &mut Vec<ObjParam>) -> Result<(), ObjError> {
    textures.push(ObjParam {
        u: ensure(token_iter.next())?,
        v: maybe(token_iter.next(), 0.0)?,
        w: maybe(token_iter.next(), 0.0)?,
    });

    Ok(())
}

fn process_face(token_iter: &mut str::SplitWhitespace, faces: &mut Vec<ObjFace>) -> Result<(), ObjError> {
    let triples: Vec<(usize, usize, usize)>  = token_iter.take(3).map(to_triple).collect::<Result<Vec<(usize, usize, usize)>, ObjError>>()?;

    if triples.len() < 3 {
        return Err(ObjError::NotEnoughVerticesInFace(triples.len()));
    }

    for index in 0 .. triples.len() - 2 {
        let a = triples[index];
        let b = triples[index + 1];
        let c = triples[index + 2];

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

fn to_triple(token: &str) -> Result<(usize, usize, usize), ObjError> {
    let mut parts = token.split('/');
    let vertex: usize = ensure(parts.next())?;
    let texture: usize = maybe(parts.next(),0)?;
    let normal: usize = maybe(parts.next(),0)?;
    assert!(parts.next().is_none());
    Ok((vertex,normal,texture))
}

/// Read the next token as an <F>, returning an appropriate error if it fails.
///
/// # Arguments
/// * `scan` - The scanner to read.
fn ensure<F: str::FromStr + std::fmt::Debug>(token: Option<&str>) -> Result<F, ObjError> {
    match token {
        None => Err(ObjError::UnexpectedEndOfFile(String::from("Expected a value"))),
        Some(s) => match s.parse() {
            Ok(v) => Ok(v),
            Err(_) => {println!("Error!!"); Err(ObjError::NotAFloat(s.to_string()))}
        }
    }
}

/// Read the next token as an <F> if there is one. Else return the default.
///
/// # Arguments
/// * `scan` - The scanner to read.
/// * `default` - The value to return if there are no more values to read
fn maybe<F: str::FromStr>(token: Option<&str>, default: F) -> Result<F, ObjError> {
    match token {
        None => Ok(default),
        Some("") => Ok(default),
        Some(s) => match s.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ObjError::NotAFloat(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ensure() {
        assert!(super::ensure::<f32>(None).is_err());
        assert_eq!(5.5, super::ensure(Some("5.5")).unwrap());
        assert_ne!(5.5, super::ensure(Some("5")).unwrap());
    }

    #[test]
    fn test_maybe() {
        assert_eq!(6, super::maybe(None, 6).unwrap());
        assert_eq!(5.5, super::maybe(Some("5.5"), 3.0).unwrap());
        assert_ne!(3, super::maybe(Some("5"),3).unwrap());
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
}
