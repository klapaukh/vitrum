use super::vector3::Vector3D;
use super::{Plane, Ray, Collision, CollisionDirection};
use std::cmp::PartialEq;
use std::fmt::Display;
use num::Float;

#[derive(Debug, Clone)]
pub struct Face<T: Float> {
    pub face_normal: Vector3D<T>,

    pub a: Vector3D<T>,
    pub b: Vector3D<T>,
    pub c: Vector3D<T>,

    pub a_normal: Vector3D<T>,
    pub b_normal: Vector3D<T>,
    pub c_normal: Vector3D<T>,

    pub a_texture: Vector3D<T>,
    pub b_texture: Vector3D<T>,
    pub c_texture: Vector3D<T>,
}

impl<T: Float> Face<T> {
    pub fn new(normal: Vector3D<T>, a: Vector3D<T>, b: Vector3D<T>, c: Vector3D<T>) -> Face<T> {
        Face {
            face_normal: normal,

            a,
            b,
            c,

            a_normal: normal,
            b_normal: normal,
            c_normal: normal,

            a_texture: Vector3D::zero(),
            b_texture: Vector3D::zero(),
            c_texture: Vector3D::zero()
        }
    }

    pub fn from_points(a: Vector3D<T>, b: Vector3D<T>, c: Vector3D<T>) -> Face<T> {
        let normal = ((b - a) ^ (c - b)).normalize();
        Self::new(normal, a, b, c)
    }
}

impl<T: Float + std::fmt::Display> Display for Face<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {} | {})", self.a, self.b, self.c, self.face_normal)
    }
}

impl<T: Float> PartialEq for Face<T> {
    fn eq(&self, other: &Self) -> bool {
        self.face_normal == other.face_normal &&
        self.face_normal * (self.a - other.a) == T::zero()
    }
}

impl<T: Float> Plane<Face<T>, T> for Face<T> {
    fn hits(&self, ray: &Ray<T>) -> Option<Collision<Face<T>, T>> {
        let epsilon: T = T::from(1e-5)?;

        // Check if ray parallel to triangle (i.e. orthogonal to normal)
        // Check if ray facing back of triangle
        // Note: ray . norm == 0 if they are perpendicular
        // ray . norm > 0 if the ray is facing the *back* of the triangle
        let ray_norm_dot: T = ray.direction * self.face_normal;
        // println!("{} * {} = {}", ray.direction, self.normal, ray_norm_dot);
        let collision_face = if ray_norm_dot > -epsilon {
                // println!("Ray facing back of triangle");
                CollisionDirection::BackFace
            } else {
                CollisionDirection::FrontFace
            };

        // Find intersection of ray and triangle
        let d: T = self.face_normal * self.a;
        // println!("{} * {} = {}", self.normal, self.a, d);
        let t: T = (d - (self.face_normal * ray.origin)) / ray_norm_dot;

        // println!("-(({} * {}) + {})/ {} = {}", self.normal, ray.origin, d, ray_norm_dot, t);
        if t < T::zero() {
            // point behind ray origin
            // println!("Hit behind ray origin at t = {}", t);
            return None;
        }

        let hit: Vector3D<T> = ray.origin + ray.direction * t;
        // Check if this point is inside the triangle

        // edge 0
        let edge: Vector3D<T> = self.b - self.a;
        let other: Vector3D<T> = hit - self.a;
        let c: Vector3D<T> = edge ^ other;
        if self.face_normal * c < T::zero() {
            // println!("Missed Edge 1");
            return None;  // P is on the right side
        }

        // edge 1
        let edge = self.c - self.b;
        let other = hit - self.b;
        let c = edge ^ other;
        if self.face_normal * c < T::zero() {
            // println!("Missed Edge 2");
            return None;  // P is on the right side
        }

        // edge 2
        let edge = self.a - self.c;
        let other = hit - self.c;
        let c = edge ^ other;
        if self.face_normal * c < T::zero() {
            // println!("Missed Edge 3");
            return None;  // P is on the right side
        }

        // Assign the collision point
        // println!("Hit!");
        Some(
            Collision {
                object: self.clone(),
                contact_point: hit,
                distance: t,
                direction: collision_face
            }
        )
    }

    fn min_extents(&self) -> Vector3D<T> {
        self.a.min(self.b).min(self.c)
    }

    fn max_extents(&self) -> Vector3D<T> {
        self.a.max(self.b).max(self.c)
    }

    fn translate(&self, t: Vector3D<T>) -> Self {
        // TODO FIX to handle point normals
        Face::new(self.face_normal,
                  self.a + t,
                  self.b + t,
                  self.c + t)
    }
}

#[cfg(test)]
mod tests {
    use super::{Face, Vector3D};

    #[test]
    fn test_compute_normal() {
        let a = Vector3D::new(1.0, 2.0, 0.0);
        let b = Vector3D::new(-1.0, 2.0, 1.0);
        let c = Vector3D::new(-1.0, 2.0, -1.0);
        let f = Face::from_points(a, b, c);

        let normal = Vector3D::new(0.0, -1.0, 0.0);

        assert_eq!(f.face_normal, normal);
    }
}