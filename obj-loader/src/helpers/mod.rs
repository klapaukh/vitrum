#[derive(Debug, Clone)]
pub struct ObjVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

pub struct ObjNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct ObjParam {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

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
