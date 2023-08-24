use geometry::{Ray, Collision, Plane};

fn lambert(ray: &Ray, collision: &Collision) -> f64 {
//   println!("{:?}", collision.normal);
    1.0 - (collision.normal.dot(&ray.direction))
}

pub fn trace<T:Plane>(ray: &Ray, model: &T) -> f64 {
     // println!("{:?}", ray);
     let hit = model.hits(&ray);
     if let Some(c) = hit {
        lambert(&ray, &c)
     } else {
        0.0
     }
}