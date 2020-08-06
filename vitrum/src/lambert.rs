use geometry::{Ray, Collision, Plane};

fn lambert(ray: &Ray<f32>, collision: &Collision<f32>) -> f32 {
    1.0 - (collision.normal.normalize() * ray.direction.normalize())
}

pub fn trace<T:Plane<f32>>(ray: &Ray<f32>, model: &T) -> f32 {
     // println!("{:?}", ray);
     let hit = model.hits(&ray);
     if let Some(c) = hit {
        lambert(&ray, &c)
     } else {
        0.0
     }
}