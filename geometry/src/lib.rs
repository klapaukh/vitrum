use std::f32;

mod vector3;
mod face;
mod collision;
mod plane;

pub use vector3::{ZERO, X, Y, Z};
pub use vector3::Vector3D;
pub use face::Face;
pub use collision::{Collision, CollisionDirection};
pub use plane::Plane;

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