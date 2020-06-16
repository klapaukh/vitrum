use std::ops;
use std::cmp;

use num::{Num, Float, Signed};

/// Data structure to represent a 3 space numeric vector
#[derive(Debug, Copy, Clone)]
pub struct Vector3D<T: Num + Copy> {
    /// X coordinate
    pub x: T,
    /// Y coordinate
    pub y: T,
    /// Z coordinate
    pub z: T
}

/// Data structure to represent a 3 space numeric vector
#[derive(Debug, Copy, Clone)]
pub struct Vector4D<T: Num> {
    /// X coordinate
    pub x: T,
    /// Y coordinate
    pub y: T,
    /// Z coordinate
    pub z: T,
    /// W coordinate
    pub w: T
}

impl<T: Num + Copy> Vector3D<T> {
    /// Create a new vector from 3 points
    pub fn new(x: T, y: T,z: T) -> Vector3D<T> {
        Vector3D {
            x,
            y,
            z
        }
    }

    /// Test if this is the zero vector
    pub fn is_zero(&self) -> bool {
        self.x == T::zero() &&
        self.y == T::zero() &&
        self.z == T::zero()

    }

    pub fn set_x(&self, value: T) -> Vector3D<T> {
        Vector3D::new(value,
                      self.y,
                      self.z)
    }

    pub fn set_y(&self, value: T) -> Vector3D<T> {
        Vector3D::new(self.x,
                      value,
                      self.z)
    }

    pub fn set_z(&self, value: T) -> Vector3D<T> {
        Vector3D::new(self.x,
                      self.y,
                      value)
    }

    pub fn proj_x(&self) -> Vector3D<T> {
        Vector3D::new(self.x,
                      T::zero(),
                      T::zero())
    }

    pub fn proj_y(&self) -> Vector3D<T> {
        Vector3D::new(T::zero(),
                      self.y,
                      T::zero())
    }

    pub fn proj_z(&self) -> Vector3D<T> {
        Vector3D::new(T::zero(),
                      T::zero(),
                      self.z)
    }

    /// The Zero vector
    pub fn zero() -> Vector3D<T> {
        Vector3D { x: T::zero(), y: T::zero(), z: T::zero()}
    }

    /// The X axis vector
    pub fn x() -> Vector3D<T> {
        Vector3D { x: T::one(), y: T::zero(), z: T::zero()}
    }
    /// The Y axis Vector
    pub fn y() -> Vector3D<T> {
        Vector3D { x: T::zero(), y: T::one(), z: T::zero()}
    }
    /// The Z axis Vector
    pub fn z() -> Vector3D<T> {
        Vector3D { x: T::zero(), y: T::zero(), z: T::one()}
    }
}

impl<T: Float>  Vector3D<T> {
    /// Vectorised pairwise minimum. Returns a vector of the minimums for each cooridinate.
    pub fn min(&self , b: Vector3D<T>) -> Vector3D<T> {
        Vector3D::new(self.x.min(b.x),
                      self.y.min(b.y),
                      self.z.min(b.z))
    }

    /// Vectorised pairwise maximum. Returns a vector of the maximums for each cooridinate.
    pub fn max(&self, b: Vector3D<T>) -> Vector3D<T> {
        Vector3D::new(self.x.max(b.x),
                      self.y.max(b.y),
                      self.z.max(b.z))
    }

    pub fn min_value(&self) -> T {
        self.x.min(self.y).min(self.z)
    }

    pub fn max_value(&self) -> T {
        self.x.max(self.y).max(self.z)
    }

    /// Create a normalised (unit length) version of the vector
    pub fn normalize(&self) -> Vector3D<T> {
        *self / self.length()
    }

    /// Compute the Euclidean length of the vector
    pub fn length(&self) -> T {
        T::sqrt(self.x * self.x +
                self.y * self.y +
                self.z * self.z)
    }
}

impl<T: Num + Copy + Signed>  Vector3D<T> {
    /// Component-wise absolute value
    pub fn abs(&self) -> Vector3D<T> {
        Vector3D::new(self.x.abs(),
                      self.y.abs(),
                      self.z.abs())
    }
}

impl<T: Num + Copy> ops::Add<Vector3D<T>> for Vector3D<T> {
    type Output = Vector3D<T>;

    fn add(self, rhs: Vector3D<T>) -> Vector3D<T> {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: Num + Copy> ops::Sub<Vector3D<T>> for Vector3D<T> {
    type Output = Vector3D<T>;

    fn sub(self, rhs: Vector3D<T>) -> Vector3D<T> {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Num + Copy> ops::BitXor for Vector3D<T> {
    type Output = Self;

    /// Vector Cross Product
    fn bitxor(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl<T: Num + Copy> ops::Mul<Vector3D<T>> for Vector3D<T> {
    type Output = T;

    /// Implements the dot product
    fn mul(self, rhs: Vector3D<T>) -> T {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
}

impl ops::Mul<Vector3D<f32>> for f32 {
    type Output = Vector3D<f32>;

    /// Vector * Scalar
    fn mul(self, rhs: Vector3D<f32>) -> Vector3D<f32> {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

impl ops::Mul<Vector3D<f64>> for f64 {
    type Output = Vector3D<f64>;

    /// Vector * Scalar
    fn mul(self, rhs: Vector3D<f64>) -> Vector3D<f64> {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

impl<T: Num + Copy> ops::Mul<T> for Vector3D<T> {
    type Output = Vector3D<T>;

    /// Implements the scalar multiplcation
    fn mul(self, rhs: T) -> Vector3D<T> {
        Vector3D {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z
        }
    }
}

impl <T: Num + Copy> ops::Div<T> for Vector3D<T> {
    type Output = Vector3D<T>;

    /// Implements scalar division
    fn div(self, rhs: T) -> Vector3D<T> {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl <T: Num + Copy> cmp::PartialEq for Vector3D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl <T: Num + Copy + std::fmt::Display> std::fmt::Display for Vector3D<T> {
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

    #[test]
    fn test_min() {
        let x: Vector3D<f32> = Vector3D::new(1.0, 4.0, 3.0);
        let y = Vector3D::new(7.0, 2.0, 5.0);
        let z = x.min(y);
        let zz = y.min(x);
        assert_eq!(z, zz);
        assert_eq!(z, Vector3D::new(1.0, 2.0, 3.0));

    }

    #[test]
    fn test_max() {
        let x = Vector3D::new(1.0, 4.0, 3.0);
        let y = Vector3D::new(7.0, 2.0, 5.0);
        let z = x.max(y);
        let zz = y.max(x);
        assert_eq!(z, zz);
        assert_eq!(z, Vector3D::new(7.0, 4.0, 5.0));
    }
}