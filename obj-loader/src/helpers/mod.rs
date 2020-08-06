use geometry::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct ObjVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

pub type ObjNormal = Vector3D<f32>;

#[derive(Debug, Clone, Copy)]
pub struct ObjParam {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct ObjFace {
    // Vertex 1
    pub av: usize,
    pub an: usize,
    pub at: usize,
    // Vertex 2
    pub bv: usize,
    pub bn: usize,
    pub bt: usize,
    // Vertex 3
    pub cv: usize,
    pub cn: usize,
    pub ct: usize,
}

impl ObjVertex {

    /// Convert an ObjVertex to an Vector3D<f32>.
    /// This panics if the point is at infinity (w != 0)
    pub fn to_vector_3d(&self) -> Vector3D<f32> {
        //assert_relative_ne!(0.0, self.w, max_relative = 1.0);
        Vector3D::new(self.x/self.w, self.y/self.w, self.z/self.w)
    }
}
