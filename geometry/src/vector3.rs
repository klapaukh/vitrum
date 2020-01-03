use std::ops;
use std::cmp;

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

    pub fn normalize(&self) -> Vector3D {
        *self / self.length()
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x +
                self.y * self.y +
                self.z * self.z)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 &&
        self.y == 0.0 &&
        self.z == 0.0

    }
    pub fn min(&self , b: Vector3D) -> Vector3D {
        Vector3D::new(self.x.min(b.x),
                      self.y.min(b.y),
                      self.z.min(b.z))
    }

    pub fn max(&self, b: Vector3D) -> Vector3D {
        Vector3D::new(self.x.max(b.x),
                      self.y.max(b.y),
                      self.z.max(b.z))
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

impl ops::Mul<f32> for Vector3D {
    type Output = Vector3D;

    /// Implements the scalar multiplcation
    fn mul(self, rhs: f32) -> Vector3D {
        Vector3D {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z
        }
    }
}

impl ops::Div<f32> for Vector3D {
    type Output = Vector3D;

    /// Implements division
    fn div(self, rhs: f32) -> Vector3D {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl cmp::PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector3D};

    #[test]
    fn test_new() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(5.0, 6.0, 7.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 8.0);
        assert_eq!(v3.z, 10.0);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(5.0, 6.0, 7.0);
        let v3 = v2 - v1;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 4.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector3D::new(2.0, 3.0, 4.0);
        let v2 = Vector3D::new(3.0, 4.0, 5.0);
        let v3 = v1 * v2;
        assert_eq!(v3, 38.0);
    }

    #[test]
    fn test_times() {
        let v1 = Vector3D::new(2.0, 3.0, 4.0);
        let a = 3.0;
        let v3 = v1 * a;
        let v4 = a * v1;
        assert_eq!(v3, Vector3D::new(6.0, 9.0, 12.0));
        assert_eq!(v4, Vector3D::new(6.0, 9.0, 12.0));
    }

    #[test]
    fn test_eq() {
        let v1 = Vector3D::new(2.0, 3.0, 4.0);
        assert_eq!(v1, v1);
        assert_ne!(v1, Vector3D::new(6.0, 9.0, 13.0));
    }

    #[test]
    fn test_cross_product_basic() {
        let x = Vector3D::new(1.0, 0.0, 0.0);
        let y = Vector3D::new(0.0, 1.0, 0.0);
        let z = Vector3D::new(0.0, 0.0, 1.0);
        println!("{}", 0f32 == -0f32);
        assert_eq!(x ^ y , z);
        assert_eq!(y ^ x , -1.0 * z);
    }

    #[test]
    fn test_cross_product_2() {
        let x = Vector3D::new(1.0, 2.0, 3.0);
        let y = Vector3D::new(7.0, 4.0, 5.0);
        let z = Vector3D::new(-2.0, 16.0, -10.0);
        println!("{}", 0f32 == -0f32);
        assert_eq!(x ^ y , z);
        assert_eq!(y ^ x , -1.0 * z);
    }
}