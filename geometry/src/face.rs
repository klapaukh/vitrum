use super::{Plane, Ray, Collision, CollisionDirection, Vec3};
use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Face {
    face_normal: Vec3,

    a: Vec3,
    b: Vec3,
    c: Vec3,

    a_normal: Vec3,
    b_normal: Vec3,
    c_normal: Vec3,

    a_texture: Option<Vec3>,
    b_texture: Option<Vec3>,
    c_texture: Option<Vec3>,
}

impl Face {

    pub fn new(a: Vec3, b: Vec3, c: Vec3,
               face_normal: Vec3,
               a_normal: Vec3, b_normal: Vec3, c_normal: Vec3,
               a_texture: Option<Vec3>, b_texture: Option<Vec3>, c_texture: Option<Vec3>) -> Face {

        Face {
            face_normal: face_normal.normalize(),

            a,
            b,
            c,

            a_normal: a_normal.normalize(),
            b_normal: b_normal.normalize(),
            c_normal: c_normal.normalize(),

            a_texture,
            b_texture,
            c_texture
        }
    }

    pub fn from_points_with_normals(a: Vec3, b: Vec3, c: Vec3, an: Vec3, bn: Vec3, cn: Vec3) -> Face {
        let face_normal = ((an + bn + cn) / 3.0).normalize();
        Face {
            face_normal,
            a,
            b,
            c,
            a_normal : an.normalize(),
            b_normal : bn.normalize(),
            c_normal : cn.normalize(),
            a_texture: None,
            b_texture: None,
            c_texture: None
        }
    }

    pub fn from_points_with_face(normal: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Face {
        let n = normal.normalize();
        Face {
            face_normal: n,

            a,
            b,
            c,

            a_normal: n,
            b_normal: n,
            c_normal: n,

            a_texture: None,
            b_texture: None,
            c_texture: None
        }
    }

    /// Given 3 points convert them into a triangle. The normal is computed.
    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Face {
        let normal_dir = (b - a).cross(&(c - b));
        Self::from_points_with_face(normal_dir, a, b, c)
    }

}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {} | {})", self.a, self.b, self.c, self.face_normal)
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.face_normal == other.face_normal &&
        self.face_normal.dot(&(self.a - other.a)).abs() < 1e-9
    }
}

impl Plane for Face {
    fn hits(&self, ray: &Ray) -> Option<Collision> {
        let epsilon: f64 = 1e-5;

        // Check if ray parallel to triangle (i.e. orthogonal to normal)
        // Check if ray facing back of triangle
        // Note: ray . norm == 0 if they are perpendicular
        // ray . norm > 0 if the ray is facing the *back* of the triangle
        let ray_norm_dot = ray.direction.dot(&self.face_normal);
        let collision_face = if ray_norm_dot > -epsilon {
                CollisionDirection::BackFace
            } else {
                CollisionDirection::FrontFace
            };

        // Find intersection of ray and triangle
        let t = (self.face_normal.dot(&self.a) - self.face_normal.dot(&ray.origin)) / ray_norm_dot;

        // println!("-(({} * {}) + {})/ {} = {}", self.normal, ray.origin, d, ray_norm_dot, t);
        if t < 0.0 {
            // point behind ray origin
            // println!("Hit behind ray origin at t = {}", t);
            return None;
        }

        let hit: Vec3 = ray.at(t);

        // Check if this point is inside the triangle
        // We do this by conversion to Barycentric co-ordinates in order to interpolate
        // the normal. Note that this relies on the ratio of areas being the same.

        let ab = self.b - self.a;
        let bc = self.c - self.b;
        let ca = self.a - self.c;

        let area = 0.5 * ab.cross(&bc).dot(&self.face_normal);

        let area_bc = 0.5 * bc.cross(&(hit - self.b)).dot(&self.face_normal);
        let area_ca = 0.5 * ca.cross(&(hit - self.c)).dot(&self.face_normal);

        let u = area_bc / area;
        let v = area_ca / area;
        let w = 1.0 - u - v;

        // If any of the coordinates are negative, the point is outside
        if u < 0.0 || v < 0.0 || w < 0.0 {
            return None;
        }

        let interpolated_normal = (u * self.a_normal + v * self.b_normal + w * self.c_normal).normalize();
        //println!("-- ({:0.2},{:0.2},{:0.2}) {:?}", u, v, w, interpolated_normal);
        Some(
            Collision {
                normal: interpolated_normal,
                contact_point: hit,
                distance: t,
                direction: collision_face
            }
        )
    }

    fn min_extents(&self) -> Vec3 {
        self.a.inf(&self.b).inf(&self.c)
    }

    fn max_extents(&self) -> Vec3 {
        self.a.sup(&self.b).sup(&self.c)
    }

    fn translate(&self, t: Vec3) -> Self {
        Face::new(self.a + t,
                  self.b + t,
                  self.c + t,
                  self.face_normal,
                  self.a_normal,
                  self.b_normal,
                  self.c_normal,
                  self.a_texture,
                  self.b_texture,
                  self.c_texture,)
    }
}

#[cfg(test)]
mod tests {
    use super::{Face, Vec3};

    #[test]
    fn test_compute_normal() {
        let a = Vec3::new(1.0, 2.0, 0.0);
        let b = Vec3::new(-1.0, 2.0, 1.0);
        let c = Vec3::new(-1.0, 2.0, -1.0);
        let f = Face::from_points(a, b, c);

        let normal = Vec3::new(0.0, -1.0, 0.0);

        assert_eq!(f.face_normal, normal);
    }

    #[test]
    fn test_interpolate_normal() {
        //Do something h ere.
    }
}