use geometry::{Plane, Ray, Collision, Vector3D, X, Y, Z};
use std::marker::PhantomData;

use std::boxed::Box;
use std::collections::BinaryHeap;
use std::vec::Vec;
use std::f32;
use std::fmt::Display;
use std::ops::Add;

use std::cmp::{Ord, Ordering};

struct Cost<T> {
    data: T,
    cost: f32
}

impl <T: Plane<S>, S> Ord for Cost<&BoundingVolumeHierarchy<T, S>> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Panics if it is NAN. You cannot have a NAN cost.
        self.partial_cmp(other).unwrap()
    }
}

impl <T: Plane<S>, S> PartialOrd for Cost<&BoundingVolumeHierarchy<T, S>> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl <T: Plane<S>, S> Eq for Cost<&BoundingVolumeHierarchy<T, S>> {
}

impl <T: Plane<S>, S> PartialEq for Cost<&BoundingVolumeHierarchy<T, S>> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}


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

impl <T: Plane<S> + Display, S> BoundingVolumeHierarchy<T, S> {
    pub fn pretty_print(&self) {
        self.pretty_print_helper(&"".to_owned())
    }

    fn pretty_print_helper(&self, padding: &String) {
        match self {
            BoundingVolumeHierarchy::Empty => println!("{}E", padding),
            BoundingVolumeHierarchy::Child(f) => println!("{}Child - {} {} ({})",
                padding, f.min_extents(), f.max_extents(), f),
            BoundingVolumeHierarchy::Node {min, max, left, right, ..} => {
                println!("{}Node - {} {}", padding, min, max);
                let padding = padding.clone().add(" ");
                left.pretty_print_helper(&padding);
                right.pretty_print_helper(&padding);
            }
        }
    }
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

    fn collide_child<'a>(&'a self, ray: &Ray, t: Vector3D, s: Vector3D,
                     min_t: &mut f32, result: &mut Option<Collision<S>>,
                     heap: &mut BinaryHeap<Cost<&'a BoundingVolumeHierarchy<T, S>>>) {
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

    fn collide(&self, ray: &Ray, t: Vector3D, s: Vector3D) -> Option<Collision<S>> {
        if let BoundingVolumeHierarchy::Node {..} = self {
            let mut heap: BinaryHeap<Cost<&BoundingVolumeHierarchy<T, S>>> = BinaryHeap::new();
            let mut min_t = f32::INFINITY;
            let mut result: Option<Collision<S>> = None;
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

    fn collide_box(&self, t: Vector3D, s: Vector3D) -> (bool, f32) {
        if let BoundingVolumeHierarchy::Node {min, max, ..} = self {
            let mut min_t = f32::NEG_INFINITY;
            let mut max_t = f32::INFINITY;

            let (l ,u) = Self::collide_slab(t.x, s.x, min.x, max.x);
            min_t = f32::max(min_t, l);
            max_t = f32::min(max_t, u);

            if min_t == f32::INFINITY || min_t > max_t {
                return (false, f32::INFINITY);
            }
            let (l ,u) = Self::collide_slab(t.y, s.y, min.y, max.y);
            min_t = f32::max(min_t, l);
            max_t = f32::min(max_t, u);

            if min_t == f32::INFINITY || min_t > max_t {
                return (false, f32::INFINITY);
            }
            let (l ,u) = Self::collide_slab(t.z, s.z, min.z, max.z);
            min_t = f32::max(min_t, l);
            max_t = f32::min(max_t, u);

            return if  min_t != f32::INFINITY && max_t >= min_t {
                if max_t <= 0.0 {
                    (false, f32::INFINITY)
                } else if min_t >= 0.0 {
                    (true, min_t)
                } else {
                    (true, 0.0)
                }
            } else {
                (false, f32::INFINITY)
            };
        }
        panic!("Collide_box called on a non node");
    }

    fn collide_slab(t: f32, s: f32, min: f32, max: f32) -> (f32, f32) {
        let t_min = (min - s) * t;
        let t_max = (max - s) * t;
        if t_min < t_max {
            (t_min, t_max)
        } else {
            (t_max, t_min)
        }
    }

    /// Compute helper values for the slab collision
    fn compute_t_s(ray: &Ray) -> (Vector3D, Vector3D) {
        let t = Vector3D::new(1.0 / (X * ray.direction),
                              1.0 / (Y * ray.direction),
                              1.0 / (Z * ray.direction));
        let s = Vector3D::new(X * ray.origin, Y * ray.origin, Z * ray.origin);
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

impl <T: Plane<S> + Clone, S> BoundingVolumeHierarchy<T, S> {
    pub fn new(faces: Vec<T>) -> BoundingVolumeHierarchy<T, S> {
        <BoundingVolumeHierarchy<T, S>>::new_helper(faces, 0)
    }

    fn new_helper(faces: Vec<T>, axis: u8) -> BoundingVolumeHierarchy<T, S> {
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

impl <T: Plane<S>, S> Plane<S> for BoundingVolumeHierarchy<T, S> {

    fn hits(&self, ray: &Ray) -> Option<Collision<S>> {
        match self {
            BoundingVolumeHierarchy::Empty => None,
            BoundingVolumeHierarchy::Child(f) => f.hits(ray),
            BoundingVolumeHierarchy::Node { .. } => {
                let (t, s) = <BoundingVolumeHierarchy<T,S>>::compute_t_s(ray);
                self.collide(ray, t, s)
            }
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
        let (t, s) = <BoundingVolumeHierarchy<Face,Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(t.x, s.x, 5.0, 15.0);
        assert_eq!(a, 4.0);
        assert_eq!(b, 14.0);
    }

    #[test]
    fn test_parallel() {
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), X);
        let (t, s) = <BoundingVolumeHierarchy<Face,Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(t.y, s.y, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::NEG_INFINITY);
        assert_eq!(b, f32::INFINITY);

        let ray = Ray::new(Vector3D::new(2.0, 2.0, 2.0), Y);
        let (t, s) = <BoundingVolumeHierarchy<Face,Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(t.x, s.x, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::NEG_INFINITY);
        assert_eq!(b, f32::INFINITY);
    }

    #[test]
    fn test_parallel_miss() {
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), X);
        let (t, s) = <BoundingVolumeHierarchy<Face,Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(t.y, s.y, 5.0, 10.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::INFINITY);
        assert_eq!(b, f32::INFINITY);
    }

    /// Test ray hitting when the slabs are hit min first then max
    #[test]
    fn test_hit() {
        let ray = Ray::new(Vector3D::new(1.0, 2.0, 3.0), Vector3D::new(1.0, 5.0, 3.0).normalize());
        let (t, s) = <BoundingVolumeHierarchy<Face,Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(t.x, s.x, 5.0, 15.0);

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
        let (t, s) = <BoundingVolumeHierarchy<Face,Face>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face,Face>>::collide_slab(t.x, s.x, 5.0, 15.0);

        assert_eq!(ray.at(a).x, 15.0);
        assert_eq!(ray.at(b).x, 5.0);
        assert!(a < b);
        assert!(a > f32::NEG_INFINITY);
        assert!(b < f32::INFINITY);
    }
}
