use super::Vector3D;

use num::Float;

#[derive(Clone, Debug)]
pub struct Collision<V: Float> {
    pub contact_point: Vector3D<V>,
    pub normal: Vector3D<V>,
    pub distance: V,
    pub direction: CollisionDirection
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CollisionDirection {
    FrontFace,
    BackFace
}

impl <V: Float> Collision<V> {
    pub fn min<'a>(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }

    pub fn min_optional(self, other: Option<Collision<V>>) -> Collision<V> {
        if let Some(o) = other {
            self.min(o)
        } else {
            self
        }
    }
}
