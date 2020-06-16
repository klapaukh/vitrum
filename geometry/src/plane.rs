use super::{Collision, Vector3D, Ray};
use num::{Float};

pub trait Plane<T, V: Float> {
    fn hits(&self, ray: &Ray<V>) -> Option<Collision<T,V>>;
    fn min_extents(&self) -> Vector3D<V>;
    fn max_extents(&self) -> Vector3D<V>;
    fn translate(&self, t: Vector3D<V>) -> Self;
}


impl <T: Plane<S, V>, S, V: Float> Plane<S, V> for Vec<T> {
    fn hits(&self, ray: &Ray<V>) -> Option<Collision<S, V>> {
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

    fn min_extents(&self) -> Vector3D<V> {
        let mut min = Vector3D::new(V::infinity(), V::infinity(), V::infinity());
        for f in self {
            min = min.min(f.min_extents());
        }
        min
    }

    fn max_extents(&self) -> Vector3D<V> {
        let mut max = Vector3D::new(V::neg_infinity(), V::neg_infinity(), V::neg_infinity());
        for f in self {
            max = max.max(f.max_extents());
        }
        max
    }

    fn translate(&self, t: Vector3D<V>) -> Self {
        self.iter().map(|value| {
            value.translate(t)
        }).collect()
    }
}