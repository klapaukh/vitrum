use file_loader;
use geometry::{Vector3D, Ray, Plane};
use std::vec::Vec;
use bvh::BoundingVolumeHierarchy;

use std::f32;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

fn deg_to_rad(deg: f32) -> f32{
    std::f32::consts::PI * deg / 180.0
}

fn main() {
    let filename = "data/sphere.stl";

    println!("You have selected the file {} to open", filename);

    let model = file_loader::load_file(filename).unwrap();

    let model = BoundingVolumeHierarchy::new(&model);

    let origin = Vector3D::new(1.0, 0.0, -2.2);

    let up = Vector3D::new(0.0, 1.0, 0.0).normalize();
    let forwards = Vector3D::new(0.0, 0.0, 1.0).normalize();
    let left = (forwards ^ up).normalize();

    // println!("x = {}, y = {}, z = {}", left, up, forwards);

    let x_fov: f32 = deg_to_rad(90.0);
    let y_fov: f32 = deg_to_rad(90.0);

    let x_dist_left: f32 = f32::tan(x_fov / 2.0);
    let y_dist_up: f32   = f32::tan(y_fov / 2.0);

    // println!("x_dist = {:.2}, y_dist = {:.2}", x_dist_left, y_dist_up);

    let top_center = origin + forwards + y_dist_up * up;

    let top_left = top_center + left * x_dist_left;

    let x_res: usize = 500;
    let y_res: usize = 500;

    let right_step = left * x_dist_left / (x_res as f32 / -2.0);
    let down_step = up * y_dist_up / (y_res as f32 / -2.0);

    let mut data: Vec<u8> = Vec::with_capacity(x_res * y_res * 4);
    for y in 0..y_res {
        for x in 0..x_res {
            let point = top_left + right_step * (x as f32) + down_step * (y as f32);
            let ray = Ray::new(origin, point - origin);
            // println!("{:?}", ray);
            let hit = model.hits(&ray);
            if let Some(_) = hit {
                data.push(0);
                data.push(255);
                data.push(0);
                data.push(255);
                // println!("Hit");
            } else {
                data.push(255);
                data.push(0);
                data.push(0);
                data.push(255);
            }
        }
    }
    // For reading and opening files
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, x_res as u32, y_res as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data[..]).unwrap(); // Save

}
