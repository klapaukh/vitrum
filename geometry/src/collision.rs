use super::Vector3D;

#[derive(Clone)]
pub struct Collision<T> {
    pub object: T,
    pub contact_point: Vector3D,
    pub distance: f32
}

impl <T> Collision<T> {
    pub fn min(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }

    pub fn min_optional(self, other: Option<Collision<T>>) -> Collision<T> {
        if let Some(o) = other {
            self.min(o)
        } else {
            self
        }
    }
}