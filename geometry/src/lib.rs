use std::ops;

#[derive(Debug, Clone)]
pub struct Face {
    pub normal: Vector3D,
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3D {
    pub fn new(x: f32, y: f32,z: f32) -> Vector3D {
        Vector3D {
            x,
            y,
            z
        }
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::BitXor for Vector3D {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = f32;

    /// Implements the dot product
    fn mul(self, rhs: Vector3D) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
}

impl ops::Mul<Vector3D> for f32 {
    type Output = Vector3D;

    /// Implements the dot product
    fn mul(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl Plane<Face> for Face {
    fn hits(&self, ray: &Ray) -> Option<Collision<Face>> {
        let epsilon: f32 = 1e-5;

        // Check if ray parallel to triangle (i.e. orthogonal to normal)
        // Check if ray facing back of triangle
        // Note: ray . norm == 0 if they are parallel
        // ray . norm > 0 if the ray is facing the *back* of the triangle
        let ray_norm_dot: f32 = ray.direction * self.normal;
        if ray_norm_dot > -epsilon {
            return None;
        }

        // Find intersection of ray and triangle
        let d: f32 = self.normal * self.a;
        let t: f32 = -((self.normal * ray.origin) + d) / ray_norm_dot;

        if t < 0f32 {
            // point behind ray origin
            return None;
        }

        let hit: Vector3D = ray.origin + t * ray.direction;
        // Check if this point is inside the triangle

        // edge 0
        let edge: Vector3D = self.b - self.a;
        let other: Vector3D = hit - self.a;
        let c: Vector3D = edge ^ other;
        if self.normal * c < 0f32 {
            return None;  // P is on the right side
        }

        // edge 1
        let edge = self.c - self.b;
        let other = hit - self.b;
        let c = edge ^ other;
        if self.normal * c < 0f32 {
            return None;  // P is on the right side
        }

        // edge 2
        let edge = self.a - self.c;
        let other = hit - self.c;
        let c = edge ^ other;
        if self.normal * c < 0f32 {
            return None;  // P is on the right side
        }

        // Assign the collision point
        Some(
            Collision {
                object: self.clone(),
                contact_point: hit,
                distance: t
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D
}

pub trait Plane<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision<T>>;
}

pub struct Collision<T> {
    pub object: T,
    pub contact_point: Vector3D,
    pub distance: f32
}

impl <T: Plane<T>> Plane<T> for Vec<T> {
    fn hits(&self, ray: &Ray) -> Option<Collision<T>> {
        let mut plane = None;
        for p in self {
            plane = match p.hits(ray) {
                None => plane,
                Some(c) => match plane {
                    None => Some(c),
                    Some(c2) if c2.distance < c.distance => Some(c2),
                    _ => plane
                }
            }
        }
        plane
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_test() {
        assert_eq!(2,2);
    }
}