use geometry::{Ray, Collision, Face, Plane};

fn lambert(ray: &Ray, collision: &Collision<Face>) -> f32 {
    1.0 - (collision.object.normal.normalize() * ray.direction.normalize())
}

pub fn trace<T:Plane<Face>>(ray: &Ray, model: &T) -> f32 {
     // println!("{:?}", ray);
     let hit = model.hits(&ray);
     if let Some(c) = hit {
        lambert(&ray, &c)
     } else {
        0.0
     }
}