use super::{Collision, Vector3D, Ray};
use std::f32;

pub trait Plane<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision<T>>;
    fn min_extents(&self) -> Vector3D;
    fn max_extents(&self) -> Vector3D;
    fn translate(&self, t: Vector3D) -> Self;
}


impl <T: Plane<S>, S> Plane<S> for Vec<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision<S>> {
        let mut plane = None;
        for p in self {
            plane = match p.hits(ray) {
                None => plane,
                Some(c) => match plane {
                    None => Some(c),
                    Some(c2) if c.distance < c2.distance => Some(c),
                    _ => plane
                }
            }
        }
        plane
    }

    fn min_extents(&self) -> Vector3D {
        let mut min = Vector3D::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        for f in self {
            min = min.min(f.min_extents());
        }
        min
    }

    fn max_extents(&self) -> Vector3D {
        let mut max = Vector3D::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for f in self {
            max = max.max(f.max_extents());
        }
        max
    }

    fn translate(&self, t: Vector3D) -> Self {
        self.iter().map(|value| {
            value.translate(t)
        }).collect()
    }
}