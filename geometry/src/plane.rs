use super::{Collision, Vec3, Ray};
use num::{Float};

pub trait Plane {
    fn hits(&self, ray: &Ray) -> Option<Collision>;
    fn min_extents(&self) -> Vec3;
    fn max_extents(&self) -> Vec3;
    fn translate(&self, t: Vec3) -> Self;
}


impl<T: Plane> Plane for Vec<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision> {
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

    fn min_extents(&self) -> Vec3 {
        let mut min = Vec3::new(f64::infinity(), f64::infinity(), f64::infinity());
        for f in self {
            min = min.inf(&f.min_extents());
        }
        min
    }

    fn max_extents(&self) -> Vec3 {
        let mut max = Vec3::new(f64::neg_infinity(), f64::neg_infinity(), f64::neg_infinity());
        for f in self {
            max = max.sup(&f.max_extents());
        }
        max
    }

    fn translate(&self, t: Vec3) -> Self {
        self.iter().map(|value| {
            value.translate(t)
        }).collect()
    }
}