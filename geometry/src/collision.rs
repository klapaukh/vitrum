use super::Vector3D;

use num::Num;

#[derive(Clone)]
pub struct Collision<T, V: Num> {
    pub object: T,
    pub contact_point: Vector3D<V>,
    pub distance: f32,
    pub direction: CollisionDirection
}

#[derive(Copy, Clone)]
pub enum CollisionDirection {
    FrontFace,
    BackFace
}

impl <T, V: Num> Collision<T, V> {
    pub fn min(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }

    pub fn min_optional(self, other: Option<Collision<T, V>>) -> Collision<T, V> {
        if let Some(o) = other {
            self.min(o)
        } else {
            self
        }
    }
}