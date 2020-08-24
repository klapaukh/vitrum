use super::Vec3;

#[derive(Clone, Debug)]
pub struct Collision {
    pub contact_point: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub direction: CollisionDirection
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CollisionDirection {
    FrontFace,
    BackFace
}

impl Collision {
    pub fn min<'a>(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }

    pub fn min_optional(self, other: Option<Collision>) -> Collision {
        if let Some(o) = other {
            self.min(o)
        } else {
            self
        }
    }
}
