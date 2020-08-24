use geometry::{Plane, Ray, Collision, Vec3};

use num::Float;

use std::boxed::Box;
use std::collections::BinaryHeap;
use std::vec::Vec;
use std::fmt::Display;
use std::ops::Add;

use std::cmp::{Ord, Ordering};

struct Cost<T> {
    data: T,
    cost: f64
}

impl <T: Plane> Ord for Cost<&BoundingVolumeHierarchy<T>> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Panics if it is NAN. You cannot have a NAN cost.
        self.partial_cmp(other).unwrap()
    }
}

impl <T: Plane> PartialOrd for Cost<&BoundingVolumeHierarchy<T>> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl <T: Plane> Eq for Cost<&BoundingVolumeHierarchy<T>> {}

impl <T: Plane> PartialEq for Cost<&BoundingVolumeHierarchy<T>> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Debug)]
pub enum BoundingVolumeHierarchy<T: Plane> {
    Node {
        min: Vec3,
        max: Vec3,
        left: Box<BoundingVolumeHierarchy<T>>,
        right: Box<BoundingVolumeHierarchy<T>>,
    },
    Child (T),
    Empty
}

impl <T: Plane + Display> BoundingVolumeHierarchy<T> {
    pub fn pretty_print(&self) {
        self.pretty_print_helper(&"".to_owned())
    }

    fn pretty_print_helper(&self, padding: &str) {
        match self {
            BoundingVolumeHierarchy::Empty => println!("{}E", padding),
            BoundingVolumeHierarchy::Child(f) => println!("{}Child - {} {} ({})",
                padding, f.min_extents(), f.max_extents(), f),
            BoundingVolumeHierarchy::Node {min, max, left, right, ..} => {
                println!("{}Node - {} {}", padding, min, max);
                let padding = padding.to_owned().add(" ");
                left.pretty_print_helper(&padding);
                right.pretty_print_helper(&padding);
            }
        }
    }
}

impl <T: Plane> BoundingVolumeHierarchy<T> {
    pub fn leaf(face: T) -> BoundingVolumeHierarchy<T> {
        BoundingVolumeHierarchy::Child(face)
    }

    pub fn empty() -> BoundingVolumeHierarchy<T> {
        BoundingVolumeHierarchy::Empty
    }

    pub fn node(left: BoundingVolumeHierarchy<T> , right: BoundingVolumeHierarchy<T>) -> BoundingVolumeHierarchy<T> {
        BoundingVolumeHierarchy::Node {
            min: left.min_extents().inf(&right.min_extents()),
            max: left.max_extents().sup(&right.max_extents()),
            left: Box::from(left),
            right: Box::from(right)
        }
    }

    fn collide_child<'a>(&'a self, ray: &Ray, t: Vec3, s: Vec3,
                     min_t: &mut f64, result: &mut Option<Collision>,
                     heap: &mut BinaryHeap<Cost<&'a BoundingVolumeHierarchy<T>>>) {
        match self {
            BoundingVolumeHierarchy::Empty => (),
            BoundingVolumeHierarchy::Child(p) => {
                let hit = p.hits(ray);
                if let Some(c) = hit {
                    if c.distance < *min_t {
                        *min_t = c.distance;
                        *result = Some(c);
                    }
                }
            },
            BoundingVolumeHierarchy::Node { .. } => {
                let (h, t) = self.collide_box(t, s);
                if h && t < *min_t {
                    heap.push(Cost { data: &self , cost: t});
                }
            }
        }
    }

    fn collide(&self, ray: &Ray, t: Vec3, s: Vec3) -> Option<Collision> {
        if let BoundingVolumeHierarchy::Node {..} = self {
            let mut heap: BinaryHeap<Cost<&BoundingVolumeHierarchy<T>>> = BinaryHeap::new();
            let mut min_t = f64::infinity();
            let mut result: Option<Collision> = None;
            {
                let (h, t) = self.collide_box(t, s);
                if h {
                    heap.push(Cost{ data: &self, cost: t});
                }
            }
            while !heap.is_empty() {
                let element = heap.pop().unwrap();
                if element.cost > min_t {
                    return result;
                }
                if let BoundingVolumeHierarchy::Node {left, right, ..} = element.data {
                    left.collide_child(ray, t, s, &mut min_t, &mut result, &mut heap);
                    right.collide_child(ray, t, s, &mut min_t, &mut result, &mut heap);
                } else {
                    panic!("Non node child put into queue");
                }
            }
            return result;
        }
        None
    }

    fn collide_box(&self, t: Vec3, s: Vec3) -> (bool, f64) {
        if let BoundingVolumeHierarchy::Node {min, max, ..} = self {
            let mut min_t = f64::neg_infinity();
            let mut max_t = f64::infinity();

            let (l ,u) = Self::collide_slab(t.x, s.x, min.x, max.x);
            min_t = f64::max(min_t, l);
            max_t = f64::min(max_t, u);

            if (min_t.is_infinite() && min_t.is_sign_positive()) || min_t > max_t {
                return (false, f64::infinity());
            }
            let (l ,u) = Self::collide_slab(t.y, s.y, min.y, max.y);
            min_t = f64::max(min_t, l);
            max_t = f64::min(max_t, u);

            if (min_t.is_infinite() && min_t.is_sign_positive()) || min_t > max_t {
                return (false, f64::infinity());
            }
            let (l ,u) = Self::collide_slab(t.z, s.z, min.z, max.z);
            min_t = f64::max(min_t, l);
            max_t = f64::min(max_t, u);

            return if  min_t.is_infinite() && min_t.is_sign_positive() && max_t >= min_t {
                if max_t <= 0.0 {
                    (false, f64::infinity())
                } else if min_t >= 0.0 {
                    (true, min_t)
                } else {
                    (true, 0.0)
                }
            } else {
                (false, f64::infinity())
            };
        }
        panic!("Collide_box called on a non node");
    }

    fn collide_slab(t: f64, s: f64, min: f64, max: f64) -> (f64, f64) {
        let t_min = (min - s) * t;
        let t_max = (max - s) * t;
        if t_min < t_max {
            (t_min, t_max)
        } else {
            (t_max, t_min)
        }
    }

    /// Compute helper values for the slab collision
    fn compute_t_s(ray: &Ray) -> (Vec3, Vec3) {
        let t = Vec3::new(1.0 / Vec3::x().dot(&ray.direction),
                          1.0 / Vec3::y().dot(&ray.direction),
                          1.0 / Vec3::z().dot(&ray.direction));
        let s = Vec3::new(Vec3::x().dot(&ray.origin), Vec3::y().dot(&ray.origin), Vec3::z().dot(&ray.origin));
        (t, s)
    }

    pub fn size(&self) -> usize {
        match self {
            BoundingVolumeHierarchy::Empty => 0,
            BoundingVolumeHierarchy::Child(_) => 1,
            BoundingVolumeHierarchy::Node { left, right, .. } => left.size() + right.size()
        }
    }
}

impl <T: Plane + Clone> BoundingVolumeHierarchy<T> {
    pub fn new(faces: Vec<T>) -> BoundingVolumeHierarchy<T> {
        <BoundingVolumeHierarchy<T>>::new_helper(faces, 0)
    }

    fn new_helper(faces: Vec<T>, axis: u8) -> BoundingVolumeHierarchy<T> {
        let helper = |a: &T, b: &T| {
            match axis {
                0 => a.min_extents().x.partial_cmp(&b.min_extents().x).unwrap(),
                1 => a.min_extents().y.partial_cmp(&b.min_extents().y).unwrap(),
                2 => a.min_extents().z.partial_cmp(&b.min_extents().z).unwrap(),
                _ => panic!("Unknown axis!")
            }
        };
        if faces.is_empty() {
            BoundingVolumeHierarchy::empty()
        } else if faces.len() == 1 {
            // Small enough to construct directly
            BoundingVolumeHierarchy::leaf(faces[0].clone())
        } else {
            // Recurse
            let mut faces = faces;
            let mut faces2 = faces.split_off(faces.len() / 2);
            faces.sort_by(helper);
            faces2.sort_by(helper);
            let left  = BoundingVolumeHierarchy::new_helper(faces, (axis + 1) % 3);
            let right = BoundingVolumeHierarchy::new_helper(faces2, (axis + 1) % 3);
            BoundingVolumeHierarchy::node(left, right)
        }
    }

}

impl <T: Plane> Plane for BoundingVolumeHierarchy<T> {

    fn hits(&self, ray: &Ray) -> Option<Collision> {
        match self {
            BoundingVolumeHierarchy::Empty => None,
            BoundingVolumeHierarchy::Child(f) => f.hits(ray),
            BoundingVolumeHierarchy::Node { .. } => {
                let (t, s) = <BoundingVolumeHierarchy<T>>::compute_t_s(ray);
                self.collide(ray, t, s)
            }
        }
    }

    fn min_extents(&self) -> Vec3 {
        match self {
            BoundingVolumeHierarchy::Empty => Vec3::new(f64::infinity(), f64::infinity(), f64::infinity()),
            BoundingVolumeHierarchy::Child(f) => f.min_extents(),
            BoundingVolumeHierarchy::Node {min : m, ..} => *m
        }
    }

    fn max_extents(&self) -> Vec3 {
        match self {
            BoundingVolumeHierarchy::Empty => Vec3::new(f64::neg_infinity(), f64::neg_infinity(), f64::neg_infinity()),
            BoundingVolumeHierarchy::Child(f) => f.max_extents(),
            BoundingVolumeHierarchy::Node {max : m, ..} => *m
        }
    }

    fn translate(&self, t: Vec3) -> Self {
        match self {
            BoundingVolumeHierarchy::Empty => BoundingVolumeHierarchy::Empty,
            BoundingVolumeHierarchy::Child(f) => BoundingVolumeHierarchy::Child(f.translate(t)),
            BoundingVolumeHierarchy::Node {max, min, left, right, .. } => {
                BoundingVolumeHierarchy::Node {
                    min: *min + t,
                    max: *max + t,
                    left:  Box::from(left.translate(t)),
                    right: Box::from(right.translate(t)),
                }
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::BoundingVolumeHierarchy;
    use super::Vec3;
    use geometry::{Face, Ray};
    use std::f64;

    #[test]
    fn test_straight_on() {
        let ray = Ray::new(Vec3::new(1.0, 0.0, 3.0), Vec3::x());
        let (t, s) = <BoundingVolumeHierarchy<Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face>>::collide_slab(t.x, s.x, 5.0, 15.0);
        assert!(f64::abs(a - 4.0) < 1e-9);
        assert!(f64::abs(b - 14.0) < 1e-9);
    }

    #[test]
    fn test_parallel() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::x());
        let (t, s) = <BoundingVolumeHierarchy<Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face>>::collide_slab(t.y, s.y, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert!(a.is_infinite() && a.is_sign_negative());
        assert!(b.is_infinite() && b.is_sign_positive());

        let ray = Ray::new(Vec3::new(2.0, 2.0, 2.0), Vec3::y());
        let (t, s) = <BoundingVolumeHierarchy<Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face>>::collide_slab(t.x, s.x, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert!(a.is_infinite() && a.is_sign_negative());
        assert!(b.is_infinite() && b.is_sign_positive());
    }

    #[test]
    fn test_parallel_miss() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::x());
        let (t, s) = <BoundingVolumeHierarchy<Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face>>::collide_slab(t.y, s.y, 5.0, 10.0);
        println!("From {} to {}", a, b);
        assert!(a.is_infinite() && a.is_sign_positive());
        assert!(b.is_infinite() && b.is_sign_positive());
    }

    /// Test ray hitting when the slabs are hit min first then max
    #[test]
    fn test_hit() {
        let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 5.0, 3.0).normalize());
        let (t, s) = <BoundingVolumeHierarchy<Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face>>::collide_slab(t.x, s.x, 5.0, 15.0);

        assert!(f64::abs(ray.at(a).x - 5.0) < 1e-9);
        assert!(f64::abs(ray.at(b).x - 15.0) < 1e-9);
        assert!(a < b);
        assert!(a > f64::NEG_INFINITY);
        assert!(b < f64::INFINITY);
    }

    /// Test the ray hitting when the slabs are hit max first then min
    #[test]
    fn test_hit2() {
        let ray = Ray::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(-1.0, -5.0, -3.0).normalize());
        let (t, s) = <BoundingVolumeHierarchy<Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face>>::collide_slab(t.x, s.x, 5.0, 15.0);

        assert!(f64::abs(ray.at(a).x - 15.0) < 1e-9);
        assert!(f64::abs(ray.at(b).x - 5.0) < 1e-9);
        assert!(a < b);
        assert!(a > f64::NEG_INFINITY);
        assert!(b < f64::INFINITY);
    }
}
