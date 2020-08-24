use geometry::{Plane, Ray, Vec3, CollisionDirection};

pub fn trace<T:Plane>(ray: &Ray, model: &T, lights: &[Vec3],
    ambient_intensity: f64, diffuse_reflection_constant: f64,
    specular_reflection_constant: f64, transmission_coefficient: f64,
    max_depth: u8) -> f64 {
        trace_down(ray, model, lights,
            ambient_intensity, diffuse_reflection_constant,
            specular_reflection_constant, transmission_coefficient,
            max_depth)
}

fn trace_down<T:Plane>(ray: &Ray, model: &T, lights: &[Vec3],
    i_a: f64, k_d: f64, k_s: f64, k_t: f64, depth: u8) -> f64 {
        if depth == 0 {
            return 0.0;
        }
        let hit = model.hits(ray);
        match hit {
            Some(c) => {
                // Ambient Light
                let mut total_i = i_a;

                let contact_shift_factor = match c.direction {
                    CollisionDirection::BackFace => -0.00001,
                    CollisionDirection::FrontFace => 0.00001
                };
                let contact = c.contact_point + contact_shift_factor * c.normal;
                let normal = c.normal;

                // Direct diffuse illumination
                {
                    let mut total_diffuse = 0.0;
                    for light in lights {
                        let light_dir = *light - contact;
                        let light_t = light_dir.norm();
                        let light_dir = light_dir.normalize();
                        let light_ray = Ray::new(contact, light_dir);
                        let hit = model.hits(&light_ray);
                        total_diffuse += match hit {
                            Some(c) if c.distance < light_t => 0.0,
                            _ => normal.dot(&light_dir)
                        }
                    }
                    total_i += k_d * total_diffuse;
                }

                // Reflected light
                {
                    let vv = ray.direction / f64::abs(ray.direction.dot(&normal));
                    let reflected_dir = vv + (2.0 * normal);
                    let reflected_ray = Ray::new(contact, reflected_dir);
                    let s = trace_down(&reflected_ray, model, lights, i_a, k_d, k_s, k_t, depth - 1);
                    total_i += k_s * s;
                }

                // transmitted light
                if k_t > 0.0 {
                    // Do something
                }
                total_i
            },
            None => 0.0
        }
}