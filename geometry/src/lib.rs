
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

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn test_test() {
        assert_eq!(2,2);
    }
}