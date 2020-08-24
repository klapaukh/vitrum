use geometry::{Vec3,Vec4};


pub type ObjVertex  = Vec4;
pub type ObjNormal = Vec3;

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

/// Convert an ObjVertex to an Vec3.
/// This panics if the point is at infinity (w == 0)
pub fn from_homogenous(v: &ObjVertex) -> Vec3 {
    //assert_relative_ne!(0.0, self.w, max_relative = 1.0);
    Vec3::new(v.x/v.w, v.y/v.w, v.z/v.w)
}