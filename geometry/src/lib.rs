use num::Float;

mod vector3;
mod face;
mod collision;
mod plane;

pub use vector3::Vector3D;
pub use face::Face;
pub use collision::{Collision, CollisionDirection};
pub use plane::Plane;

#[derive(Debug, Clone)]
pub struct Ray<T: Float> {
    pub origin: Vector3D<T>,
    pub direction: Vector3D<T>
}

impl<T: Float> Ray<T> {
    pub fn new(origin: Vector3D<T>, direction: Vector3D<T>) -> Ray<T> {
        Ray { origin, direction }
    }

    pub fn at(&self, t: T) -> Vector3D<T> {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::{Face, Vector3D, Ray, Plane, CollisionDirection};

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

        // Front face hit
        let hit = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, -10.0), Vector3D::new(0.0, 0.0, 1.0)));
        assert_eq!(hit.unwrap().direction, CollisionDirection::FrontFace);

        // Back face hit
        let hit = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, 10.0), Vector3D::new(0.0, 0.0, -1.0)));
        assert_eq!(hit.unwrap().direction, CollisionDirection::BackFace);
    }

    #[test]
    fn test_backface_culling(){
        let t =  Face::new(
            Vector3D::new( 0.0, 0.0, 1.0),
            Vector3D::new(-1.0, 0.0, 0.0),
            Vector3D::new( 1.0, 0.0, 0.0),
            Vector3D::new( 0.0, 1.0, 0.0)
        );

        // Ray hits back face of triangle
        let h = t.hits(&Ray::new(Vector3D::new(0.0, 0.5, -10.0), Vector3D::new(0.0, 0.0, 1.0)));
        assert_eq!(h.unwrap().direction, CollisionDirection::BackFace);
        // Ray in front of triangle (t < 0)
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