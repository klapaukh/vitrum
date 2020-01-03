use geometry::{Plane, Ray, Collision, Vector3D};
use std::marker::PhantomData;

use std::boxed::Box;
use std::vec::Vec;
use std::f32;

pub enum BoundingVolumeHierarchy<T: Plane<S>, S> {
    Node {
        min: Vector3D,
        max: Vector3D,
        left: Box<BoundingVolumeHierarchy<T,S>>,
        right: Box<BoundingVolumeHierarchy<T,S>>,
        object_type: PhantomData<S>
    },
    Child (T),
    Empty
}

impl <T: Plane<S> + Clone, S> BoundingVolumeHierarchy<T, S> {
    pub fn new(faces: &Vec<T>) -> BoundingVolumeHierarchy<T, S> {
        let size = faces.len();
        BoundingVolumeHierarchy::construct_bvh(&faces, 0, size)
    }

    fn construct_bvh(faces: &Vec<T>, min: usize, max: usize) -> BoundingVolumeHierarchy<T, S> {
        if min == max {
            BoundingVolumeHierarchy::empty()
        } else if max - min == 1 {
            // Small enough to construct directly
            BoundingVolumeHierarchy::leaf(faces[min].clone())
        } else {
            // Recurse
            let mid = (min + max) /  2;
            let left  = BoundingVolumeHierarchy::construct_bvh(faces, min, mid);
            let right = BoundingVolumeHierarchy::construct_bvh(faces, mid, max);
            BoundingVolumeHierarchy::node(left, right)
        }
    }

    fn node(left: BoundingVolumeHierarchy<T,S> , right: BoundingVolumeHierarchy<T,S>) -> BoundingVolumeHierarchy<T, S> {
        BoundingVolumeHierarchy::Node {
            min: left.min_extents().min(right.min_extents()),
            max: left.max_extents().max(right.max_extents()),
            left: Box::from(left),
            right: Box::from(right),
            object_type: PhantomData
        }
    }

    fn leaf(face: T) -> BoundingVolumeHierarchy<T, S> {
        BoundingVolumeHierarchy::Child(face)
    }

    fn empty() -> BoundingVolumeHierarchy<T, S> {
        BoundingVolumeHierarchy::Empty
    }

    fn collide(&self, ray: &Ray) -> Option<Collision<S>> {
        if let BoundingVolumeHierarchy::Node {left, right, ..} = self {
            if self.collide_box(ray) {
                let left_hit  = left.hits(ray);
                let right_hit = right.hits(ray);
                match left_hit {
                    None => return right_hit,
                    Some(h) => return Some(h.min_optional(right_hit))
                }
            }
        }
        None
    }

    fn collide_box(&self, ray: &Ray) -> bool {
        if let BoundingVolumeHierarchy::Node {min, max, ..} = self {
            let x = Vector3D::new(1.0 , 0.0, 0.0);
            let y = Vector3D::new(1.0 , 0.0, 0.0);
            let z = Vector3D::new(1.0 , 0.0, 0.0);
            Self::collide_slab(ray, x, min.x, max.x);
            //  TODO Actually implement the relevant optimisation
            return  true
        }
        panic!("Collide_box called on  a non node");
    }

    fn collide_slab(ray: &Ray, normal: Vector3D, min: f32, max: f32) -> (f32, f32) {
        let t = normal * ray.direction;
        let t = 1.0 / t;
        let s = normal * ray.origin;

        let t_min = (min - s) * t;
        let t_max = (max - s) * t;
        (t_min, t_max)
    }
}

impl <T: Plane<S> + Clone, S> Plane<S> for BoundingVolumeHierarchy<T, S> {

    fn hits(&self, ray: &Ray) -> Option<Collision<S>> {
        match self {
            BoundingVolumeHierarchy::Empty => None,
            BoundingVolumeHierarchy::Child(f) => f.hits(ray),
            BoundingVolumeHierarchy::Node { .. } => self.collide(ray)
        }
    }

    fn min_extents(&self) -> Vector3D {
        match self {
            BoundingVolumeHierarchy::Empty => Vector3D::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            BoundingVolumeHierarchy::Child(f) => f.min_extents(),
            BoundingVolumeHierarchy::Node {min : m, ..} => m.clone()
        }
    }

    fn max_extents(&self) -> Vector3D {
        match self {
            BoundingVolumeHierarchy::Empty => Vector3D::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            BoundingVolumeHierarchy::Child(f) => f.max_extents(),
            BoundingVolumeHierarchy::Node {max : m, ..} => m.clone()
        }
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
