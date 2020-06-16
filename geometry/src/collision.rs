use super::Vector3D;

use num::Float;

#[derive(Clone, Debug)]
pub struct Collision<T, V: Float> {
    pub object: T,
    pub contact_point: Vector3D<V>,
    pub distance: V,
    pub direction: CollisionDirection
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CollisionDirection {
    FrontFace,
    BackFace
}

impl <T, V: Float> Collision<T, V> {
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