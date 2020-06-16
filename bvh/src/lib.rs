use geometry::{Plane, Ray, Collision, Vector3D};

use num::Float;

use std::marker::PhantomData;

use std::boxed::Box;
use std::collections::BinaryHeap;
use std::vec::Vec;
use std::fmt::Display;
use std::ops::Add;

use std::cmp::{Ord, Ordering};

struct Cost<T, V: Float> {
    data: T,
    cost: V
}

impl <T: Plane<S, V>, S, V: Float> Ord for Cost<&BoundingVolumeHierarchy<T, S, V>, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Panics if it is NAN. You cannot have a NAN cost.
        self.partial_cmp(other).unwrap()
    }
}

impl <T: Plane<S, V>, S, V: Float> PartialOrd for Cost<&BoundingVolumeHierarchy<T, S, V>, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl <T: Plane<S, V>, S, V: Float> Eq for Cost<&BoundingVolumeHierarchy<T, S, V>, V> {
}

impl <T: Plane<S, V>, S, V: Float> PartialEq for Cost<&BoundingVolumeHierarchy<T, S, V>, V> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}


pub enum BoundingVolumeHierarchy<T: Plane<S, V>, S, V: Float> {
    Node {
        min: Vector3D<V>,
        max: Vector3D<V>,
        left: Box<BoundingVolumeHierarchy<T,S,V>>,
        right: Box<BoundingVolumeHierarchy<T,S,V>>,
        object_type: PhantomData<S>
    },
    Child (T),
    Empty
}

impl <T: Plane<S, V> + Display, S, V: Float + Display> BoundingVolumeHierarchy<T, S, V> {
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

impl <T: Plane<S, V>, S, V: Float> BoundingVolumeHierarchy<T, S, V> {
    pub fn leaf(face: T) -> BoundingVolumeHierarchy<T, S, V> {
        BoundingVolumeHierarchy::Child(face)
    }

    pub fn empty() -> BoundingVolumeHierarchy<T, S, V> {
        BoundingVolumeHierarchy::Empty
    }

    pub fn node(left: BoundingVolumeHierarchy<T, S, V> , right: BoundingVolumeHierarchy<T, S, V>) -> BoundingVolumeHierarchy<T, S, V> {
        BoundingVolumeHierarchy::Node {
            min: left.min_extents().min(right.min_extents()),
            max: left.max_extents().max(right.max_extents()),
            left: Box::from(left),
            right: Box::from(right),
            object_type: PhantomData
        }
    }

    fn collide_child<'a>(&'a self, ray: &Ray<V>, t: Vector3D<V>, s: Vector3D<V>,
                     min_t: &mut V, result: &mut Option<Collision<S, V>>,
                     heap: &mut BinaryHeap<Cost<&'a BoundingVolumeHierarchy<T, S, V>, V>>) {
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

    fn collide(&self, ray: &Ray<V>, t: Vector3D<V>, s: Vector3D<V>) -> Option<Collision<S, V>> {
        if let BoundingVolumeHierarchy::Node {..} = self {
            let mut heap: BinaryHeap<Cost<&BoundingVolumeHierarchy<T, S, V>, V>> = BinaryHeap::new();
            let mut min_t = V::infinity();
            let mut result: Option<Collision<S, V>> = None;
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

    fn collide_box(&self, t: Vector3D<V>, s: Vector3D<V>) -> (bool, V) {
        if let BoundingVolumeHierarchy::Node {min, max, ..} = self {
            let mut min_t = V::neg_infinity();
            let mut max_t = V::infinity();

            let (l ,u) = Self::collide_slab(t.x, s.x, min.x, max.x);
            min_t = V::max(min_t, l);
            max_t = V::min(max_t, u);

            if min_t == V::infinity() || min_t > max_t {
                return (false, V::infinity());
            }
            let (l ,u) = Self::collide_slab(t.y, s.y, min.y, max.y);
            min_t = V::max(min_t, l);
            max_t = V::min(max_t, u);

            if min_t == V::infinity() || min_t > max_t {
                return (false, V::infinity());
            }
            let (l ,u) = Self::collide_slab(t.z, s.z, min.z, max.z);
            min_t = V::max(min_t, l);
            max_t = V::min(max_t, u);

            return if  min_t != V::infinity() && max_t >= min_t {
                if max_t <= V::zero() {
                    (false, V::infinity())
                } else if min_t >= V::zero() {
                    (true, min_t)
                } else {
                    (true, V::zero())
                }
            } else {
                (false, V::infinity())
            };
        }
        panic!("Collide_box called on a non node");
    }

    fn collide_slab(t: V, s: V, min: V, max: V) -> (V, V) {
        let t_min = (min - s) * t;
        let t_max = (max - s) * t;
        if t_min < t_max {
            (t_min, t_max)
        } else {
            (t_max, t_min)
        }
    }

    /// Compute helper values for the slab collision
    fn compute_t_s(ray: &Ray<V>) -> (Vector3D<V>, Vector3D<V>) {
        let t = Vector3D::new(V::one() / (Vector3D::x() * ray.direction),
                              V::one() / (Vector3D::y() * ray.direction),
                              V::one() / (Vector3D::z() * ray.direction));
        let s = Vector3D::new(Vector3D::x() * ray.origin, Vector3D::y() * ray.origin, Vector3D::z() * ray.origin);
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

impl <T: Plane<S, V> + Clone, S, V: Float> BoundingVolumeHierarchy<T, S, V> {
    pub fn new(faces: Vec<T>) -> BoundingVolumeHierarchy<T, S, V> {
        <BoundingVolumeHierarchy<T, S, V>>::new_helper(faces, 0)
    }

    fn new_helper(faces: Vec<T>, axis: u8) -> BoundingVolumeHierarchy<T, S, V> {
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

impl <T: Plane<S, V>, S, V: Float> Plane<S, V> for BoundingVolumeHierarchy<T, S, V> {

    fn hits(&self, ray: &Ray<V>) -> Option<Collision<S, V>> {
        match self {
            BoundingVolumeHierarchy::Empty => None,
            BoundingVolumeHierarchy::Child(f) => f.hits(ray),
            BoundingVolumeHierarchy::Node { .. } => {
                let (t, s) = <BoundingVolumeHierarchy<T, S, V>>::compute_t_s(ray);
                self.collide(ray, t, s)
            }
        }
    }

    fn min_extents(&self) -> Vector3D<V> {
        match self {
            BoundingVolumeHierarchy::Empty => Vector3D::new(V::infinity(), V::infinity(), V::infinity()),
            BoundingVolumeHierarchy::Child(f) => f.min_extents(),
            BoundingVolumeHierarchy::Node {min : m, ..} => *m
        }
    }

    fn max_extents(&self) -> Vector3D<V> {
        match self {
            BoundingVolumeHierarchy::Empty => Vector3D::new(V::neg_infinity(), V::neg_infinity(), V::neg_infinity()),
            BoundingVolumeHierarchy::Child(f) => f.max_extents(),
            BoundingVolumeHierarchy::Node {max : m, ..} => *m
        }
    }

    fn translate(&self, t: Vector3D<V>) -> Self {
        match self {
            BoundingVolumeHierarchy::Empty => BoundingVolumeHierarchy::Empty,
            BoundingVolumeHierarchy::Child(f) => BoundingVolumeHierarchy::Child(f.translate(t)),
            BoundingVolumeHierarchy::Node {max, min, left, right, .. } => {
                BoundingVolumeHierarchy::Node {
                    min: *min + t,
                    max: *max + t,
                    left:  Box::from(left.translate(t)),
                    right: Box::from(right.translate(t)),
                    object_type: PhantomData
                }
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::BoundingVolumeHierarchy;
    use super::Vector3D;
    use geometry::{Face, Ray};
    use std::f32;

    #[test]
    fn test_straight_on() {
        let ray = Ray::new(Vector3D::new(1.0, 0.0, 3.0), Vector3D::x());
        let (t, s) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::collide_slab(t.x, s.x, 5.0, 15.0);
        assert_eq!(a, 4.0);
        assert_eq!(b, 14.0);
    }

    #[test]
    fn test_parallel() {
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::x());
        let (t, s) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::collide_slab(t.y, s.y, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::NEG_INFINITY);
        assert_eq!(b, f32::INFINITY);

        let ray = Ray::new(Vector3D::new(2.0, 2.0, 2.0), Vector3D::y());
        let (t, s) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::collide_slab(t.x, s.x, -5.0, 5.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::NEG_INFINITY);
        assert_eq!(b, f32::INFINITY);
    }

    #[test]
    fn test_parallel_miss() {
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::x());
        let (t, s) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::collide_slab(t.y, s.y, 5.0, 10.0);
        println!("From {} to {}", a, b);
        assert_eq!(a, f32::INFINITY);
        assert_eq!(b, f32::INFINITY);
    }

    /// Test ray hitting when the slabs are hit min first then max
    #[test]
    fn test_hit() {
        let ray = Ray::new(Vector3D::new(1.0, 2.0, 3.0), Vector3D::new(1.0, 5.0, 3.0).normalize());
        let (t, s) = <BoundingVolumeHierarchy<Face<f32>,Face<f32>, f32>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face<f32>, Face<f32>, f32>>::collide_slab(t.x, s.x, 5.0, 15.0);

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
        let (t, s) = <BoundingVolumeHierarchy<Face<f32>, Face<f32>, f32>>::compute_t_s(&ray);
        let (a, b) = <BoundingVolumeHierarchy<Face<f32>,  Face<f32>, f32>>::collide_slab(t.x, s.x, 5.0, 15.0);

        assert_eq!(ray.at(a).x, 15.0);
        assert_eq!(ray.at(b).x, 5.0);
        assert!(a < b);
        assert!(a > f32::NEG_INFINITY);
        assert!(b < f32::INFINITY);
    }
}
