use std::f32;

mod vector3;
mod face;

pub use vector3::{ZERO, X, Y, Z};
pub use vector3::Vector3D;
pub use face::Face;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D
}

impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3D {
        self.origin + (t * self.direction)
    }
}

pub trait Plane<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision<T>>;
    fn min_extents(&self) -> Vector3D;
    fn max_extents(&self) -> Vector3D;
}

#[derive(Clone)]
pub struct Collision<T> {
    pub object: T,
    pub contact_point: Vector3D,
    pub distance: f32
}

impl <T> Collision<T> {
    pub fn min(self, other: Self) -> Self {
        if self.distance < other.distance {
            return self
        }
        return other
    }

    pub fn min_optional(self, other: Option<Collision<T>>) -> Collision<T> {
        if let Some(o) = other {
            return self.min(o);
        } else {
            return self;
        }
    }
}

impl <T: Plane<T>> Plane<T> for Vec<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision<T>> {
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
}

#[cfg(test)]
mod tests {
    use super::{Face, Vector3D, Ray, Plane};

    #[test]
    fn test_hit_face_on() {
        let face = Face::from_points(
            Vector3D::new(-2.0, -1.0,  1.0),
            Vector3D::new( 1.0,  3.0,  1.0),
            Vector3D::new( 1.0, -1.0,  1.0)
        );
        let r = Ray::new(Vector3D::new(0.0, 0.0, 0.0),
                         Vector3D::new(0.0, 0.0, 1.0));
        let h  = face.hits(&r);
        assert!(h.is_some());
        let h = h.unwrap();
        assert_eq!(h.distance, 1.0);
        assert_eq!(r.at(h.distance), h.contact_point);
    }

    #[test]
    fn test_collision_1() {
        let t = Face::new(
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(-1.0, 0.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0)
        );
        let h = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, 10.0), Vector3D::new(0.0, 0.0, -1.0)));
        assert!(h.is_some());
    }

    #[test]
    fn test_collision_2() {
        let t = Face::from_points(Vector3D::new( 1.0, 0.0, 0.0),
                                  Vector3D::new(-1.0, 0.0, 0.0),
                                  Vector3D::new( 0.0, 1.0, 0.0));

        let hit = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, -10.0), Vector3D::new(0.0, 0.0, 1.0)));
        assert!(hit.is_some());

        let hit = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, 10.0), Vector3D::new(0.0, 0.0, -1.0)));
        assert!(hit.is_none());
    }

    #[test]
    fn test_backface_culling(){
        let t =  Face::new(
            Vector3D::new( 0.0, 0.0, 1.0),
            Vector3D::new(-1.0, 0.0, 0.0),
            Vector3D::new( 1.0, 0.0, 0.0),
            Vector3D::new( 0.0, 1.0, 0.0)
        );

        let h = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, -10.0), Vector3D::new(0.0, 0.0, 1.0)));
        assert!(h.is_none());
        let h = t.hits(&Ray::new(Vector3D::new(0.0, 0.5,  10.0), Vector3D::new(0.0, 0.0, 1.0)));
        assert!(h.is_none());
    }

    #[test]
    fn test_hit_face_on_vec() {
        let face1 = Face::from_points(
            Vector3D::new(-2.0, -1.0,  1.0),
            Vector3D::new( 1.0,  3.0,  1.0),
            Vector3D::new( 1.0, -1.0,  1.0)
        );
        let face2 = Face::from_points(
            Vector3D::new(-2.0, -1.0,  2.0),
            Vector3D::new( 1.0,  3.0,  2.0),
            Vector3D::new( 1.0, -1.0,  2.0)
        );
        let face3 = Face::from_points(
            Vector3D::new(-2.0, -1.0,  3.0),
            Vector3D::new( 1.0,  3.0,  3.0),
            Vector3D::new( 1.0, -1.0,  3.0)
        );
        let face4 = Face::from_points(
            Vector3D::new(-2.0, -1.0,  -3.0),
            Vector3D::new( 1.0,  3.0,  -3.0),
            Vector3D::new( 1.0, -1.0,  -3.0)
        );
        let v = vec![face3, face1.clone(), face2, face4];
        let r = Ray::new(Vector3D::new(0.0, 0.0, 0.0),
                         Vector3D::new(0.0, 0.0, 1.0));

        let h  = v.hits(&r);
        assert!(h.is_some());
        let h = h.unwrap();
        assert_eq!(h.distance, 1.0);
        assert_eq!(r.at(h.distance), h.contact_point);
        assert_eq!(h.object, face1);
    }

}