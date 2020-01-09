use geometry::{Plane, Ray, Collision, Vector3D, X, Y, Z};
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

impl <T: Plane<S>, S> BoundingVolumeHierarchy<T, S> {
    fn leaf(face: T) -> BoundingVolumeHierarchy<T, S> {
        BoundingVolumeHierarchy::Child(face)
    }

    fn empty() -> BoundingVolumeHierarchy<T, S> {
        BoundingVolumeHierarchy::Empty
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
            let mut min_t = f32::NEG_INFINITY;
            let mut max_t = f32::INFINITY;

            let (l ,u) = Self::collide_slab(ray, X, min.x, max.x);
            min_t = f32::max(min_t, l);
            max_t = f32::min(max_t, u);

            if min_t == f32::INFINITY || min_t > max_t {
                return false;
            }
            let (l ,u) = Self::collide_slab(ray, Y, min.y, max.y);
            min_t = f32::max(min_t, l);
            max_t = f32::min(max_t, u);

            if min_t == f32::INFINITY || min_t > max_t {
                return false;
            }
            let (l ,u) = Self::collide_slab(ray, Z, min.z, max.z);
            min_t = f32::max(min_t, l);
            max_t = f32::min(max_t, u);

            return  min_t != f32::INFINITY && max_t >= min_t;
        }
        panic!("Collide_box called on a non node");
    }

    fn collide_slab(ray: &Ray, normal: Vector3D, min: f32, max: f32) -> (f32, f32) {
        let t = normal * ray.direction;
        let t = 1.0 / t;
        let s = normal * ray.origin;

        let t_min = (min - s) * t;
        let t_max = (max - s) * t;
        if t_min < t_max {
            (t_min, t_max)
        } else {
            (t_max, t_min)
        }
    }

    pub fn size(&self) -> usize {
        match self {
            BoundingVolumeHierarchy::Empty => 0,
            BoundingVolumeHierarchy::Child(_) => 1,
            BoundingVolumeHierarchy::Node { left, right, .. } => left.size() + right.size()
        }
    }
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

}

impl <T: Plane<S>, S> Plane<S> for BoundingVolumeHierarchy<T, S> {

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
    use super::BoundingVolumeHierarchy;
    use super::Vector3D;
    use geometry::{Face, Ray, X, Y};
    use std::f32;

    #[test]
    fn test_straight_on() {
        let ray = Ray::new(Vector3D::new(1.0, 0.0, 3.0), X);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(&ray, X, 5.0, 15.0);
        assert_eq!(a, 4.0);
        assert_eq!(b, 14.0);
    }

    #[test]
    fn test_parallel() {
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), X);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(&ray, Y, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::NEG_INFINITY);
        assert_eq!(b, f32::INFINITY);

        let ray = Ray::new(Vector3D::new(2.0, 2.0, 2.0), Y);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(&ray, X, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::NEG_INFINITY);
        assert_eq!(b, f32::INFINITY);
    }

    #[test]
    fn test_parallel_miss() {
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), X);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(&ray, Y, 5.0, 10.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::INFINITY);
        assert_eq!(b, f32::INFINITY);
    }

    /// Test ray hitting when the slabs are hit min first then max
    #[test]
    fn test_hit() {
        let ray = Ray::new(Vector3D::new(1.0, 2.0, 3.0), Vector3D::new(1.0, 5.0, 3.0).normalize());
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(&ray, X, 5.0, 15.0);

        assert_eq!(ray.at(a).x, 5.0);
        assert_eq!(ray.at(b).x, 15.0);
        assert!(a < b);
        assert!(a > f32::NEG_INFINITY);
        assert!(b < f32::INFINITY);
    }

    /// Test the ray hitting when the slabs are hit max first then min
    #[test]
    fn test_hit2() {
        let ray = Ray::new(Vector3D::new(-1.0, -2.0, -3.0), Vector3D::new(-1.0, -5.0, -3.0).normalize());
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(&ray, X, 5.0, 15.0);

        assert_eq!(ray.at(a).x, 15.0);
        assert_eq!(ray.at(b).x, 5.0);
        assert!(a < b);
        assert!(a > f32::NEG_INFINITY);
        assert!(b < f32::INFINITY);
    }
}
