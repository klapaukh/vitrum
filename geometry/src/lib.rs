mod face;
mod collision;
mod plane;

use nalgebra::{Vector3, Vector4};
pub use face::Face;
pub use collision::{Collision, CollisionDirection};
pub use plane::Plane;

pub type Vec3 = Vector3<f64>;
pub type Vec4 = Vector4<f64>;

/// A ray is a line in 3 space with a defined origin and direction
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    /// Create a new ray. The direction will be normalised.
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction: direction.normalize() }
    }

    /// Computer the position of the ray at a distance t
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::{Face, Vec3, Ray, Plane, CollisionDirection};

    #[test]
    fn test_hit_face_on() {
        let face = Face::from_points(
            Vec3::new(-2.0, -1.0,  1.0),
            Vec3::new( 1.0,  3.0,  1.0),
            Vec3::new( 1.0, -1.0,  1.0)
        );
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0),
                         Vec3::new(0.0, 0.0, 1.0));
        let h  = face.hits(&r);
        assert!(h.is_some());
        let h = h.unwrap();
        assert_eq!(h.distance, 1.0);
        assert_eq!(r.at(h.distance), h.contact_point);
    }

    #[test]
    fn test_collision_1() {
        let t = Face::from_points_with_face(
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let h = t.hits(&Ray::new(Vec3::new(0.0, 0.5, 10.0), Vec3::new(0.0, 0.0, -1.0)));
        assert!(h.is_some());
    }

    #[test]
    fn test_collision_2() {
        let t = Face::from_points(Vec3::new( 1.0, 0.0, 0.0),
                                  Vec3::new(-1.0, 0.0, 0.0),
                                  Vec3::new( 0.0, 1.0, 0.0));

        // Front face hit
        let hit = t.hits(&Ray::new(Vec3::new(0.0, 0.5, -10.0), Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(hit.unwrap().direction, CollisionDirection::FrontFace);

        // Back face hit
        let hit = t.hits(&Ray::new(Vec3::new(0.0, 0.5, 10.0), Vec3::new(0.0, 0.0, -1.0)));
        assert_eq!(hit.unwrap().direction, CollisionDirection::BackFace);
    }

    #[test]
    fn test_backface_culling(){
        let t =  Face::from_points_with_face(
            Vec3::new( 0.0, 0.0, 1.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new( 1.0, 0.0, 0.0),
            Vec3::new( 0.0, 1.0, 0.0)
        );

        // Ray hits back face of triangle
        let h = t.hits(&Ray::new(Vec3::new(0.0, 0.5, -10.0), Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(h.unwrap().direction, CollisionDirection::BackFace);
        // Ray in front of triangle (t < 0)
        let h = t.hits(&Ray::new(Vec3::new(0.0, 0.5,  10.0), Vec3::new(0.0, 0.0, 1.0)));
        assert!(h.is_none());
    }

    #[test]
    fn test_hit_face_on_vec() {
        let face1 = Face::from_points(
            Vec3::new(-2.0, -1.0,  1.0),
            Vec3::new( 1.0,  3.0,  1.0),
            Vec3::new( 1.0, -1.0,  1.0)
        );
        let face2 = Face::from_points(
            Vec3::new(-2.0, -1.0,  2.0),
            Vec3::new( 1.0,  3.0,  2.0),
            Vec3::new( 1.0, -1.0,  2.0)
        );
        let face3 = Face::from_points(
            Vec3::new(-2.0, -1.0,  3.0),
            Vec3::new( 1.0,  3.0,  3.0),
            Vec3::new( 1.0, -1.0,  3.0)
        );
        let face4 = Face::from_points(
            Vec3::new(-2.0, -1.0,  -3.0),
            Vec3::new( 1.0,  3.0,  -3.0),
            Vec3::new( 1.0, -1.0,  -3.0)
        );
        let v = vec![face3, face1, face2, face4];
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0),
                         Vec3::new(0.0, 0.0, 1.0));

        let h  = v.hits(&r);
        assert!(h.is_some());
        let h = h.unwrap();
        assert_eq!(h.distance, 1.0);
        assert_eq!(r.at(h.distance), h.contact_point);
    }

}