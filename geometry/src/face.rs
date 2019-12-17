use super::vector3::Vector3D;
use super::{Plane, Ray, Collision};
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Face {
    pub normal: Vector3D,
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D
}

impl Face {
    pub fn new(normal: Vector3D, a: Vector3D, b: Vector3D, c: Vector3D) -> Face {
        Face {
            normal, a, b, c
        }
    }

    pub fn from_points(a: Vector3D, b: Vector3D, c: Vector3D) -> Face {
        Face {
            normal: ((b - a) ^ (c - b)).normalize(),
            a, b, c
        }
    }
}



impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.normal.normalize() == other.normal.normalize() &&
        self.normal * (self.a - other.a) == 0.0
    }
}

impl Plane<Face> for Face {
    fn hits(&self, ray: &Ray) -> Option<Collision<Face>> {
        let epsilon: f32 = 1e-5;

        // Check if ray parallel to triangle (i.e. orthogonal to normal)
        // Check if ray facing back of triangle
        // Note: ray . norm == 0 if they are perpendicular
        // ray . norm > 0 if the ray is facing the *back* of the triangle
        let ray_norm_dot: f32 = ray.direction * self.normal;
        println!("{} * {} = {}", ray.direction, self.normal, ray_norm_dot);
        if ray_norm_dot > -epsilon {
            println!("Ray facing back of triangle");
            return None;
        }

        // Find intersection of ray and triangle
        let d: f32 = self.normal * self.a;
        println!("{} * {} = {}", self.normal, self.a, d);
        let t: f32 = (d- (self.normal * ray.origin)) / ray_norm_dot;

        println!("-(({} * {}) + {})/ {} = {}", self.normal, ray.origin, d, ray_norm_dot, t);
        if t < 0f32 {
            // point behind ray origin
            println!("Hit behind ray origin at t = {}", t);
            return None;
        }

        let hit: Vector3D = ray.origin + t * ray.direction;
        // Check if this point is inside the triangle

        // edge 0
        let edge: Vector3D = self.b - self.a;
        let other: Vector3D = hit - self.a;
        let c: Vector3D = edge ^ other;
        if self.normal * c < 0f32 {
            println!("Missed Edge 1");
            return None;  // P is on the right side
        }

        // edge 1
        let edge = self.c - self.b;
        let other = hit - self.b;
        let c = edge ^ other;
        if self.normal * c < 0f32 {
            println!("Missed Edge 2");
            return None;  // P is on the right side
        }

        // edge 2
        let edge = self.a - self.c;
        let other = hit - self.c;
        let c = edge ^ other;
        if self.normal * c < 0f32 {
            println!("Missed Edge 3");
            return None;  // P is on the right side
        }

        // Assign the collision point
        println!("Hit!");
        Some(
            Collision {
                object: self.clone(),
                contact_point: hit,
                distance: t
            }
        )
    }
}