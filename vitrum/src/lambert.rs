use geometry::{Ray, Collision, Face, Plane};

fn lambert(ray: &Ray<f32>, collision: &Collision<Face<f32>, f32>) -> f32 {
    1.0 - (collision.object.face_normal.normalize() * ray.direction.normalize())
}

pub fn trace<T:Plane<Face<f32>, f32>>(ray: &Ray<f32>, model: &T) -> f32 {
     // println!("{:?}", ray);
     let hit = model.hits(&ray);
     if let Some(c) = hit {
        lambert(&ray, &c)
     } else {
        0.0
     }
}