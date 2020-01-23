use argparse::{ArgumentParser, StoreOption};

use file_loader;
use geometry::{Vector3D, Ray, Plane};

use bvh::BoundingVolumeHierarchy;

use std::vec::Vec;
use std::f32;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod stack;
use stack::stack;

mod lambert;
mod whitted;

pub enum Renderer {
    Whitted,
    Lambert
}

fn deg_to_rad(deg: f32) -> f32{
    std::f32::consts::PI * deg / 180.0
}

fn main() {
    let mut filename: Option<String> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Render output to image.png");
        ap.refer(&mut filename)
            .add_option(&["-f", "--file"], StoreOption,
            "File to parse")
            .required();
        ap.parse_args_or_exit();
    }

    let filename = filename.unwrap();

    println!("You have selected the file {} to open", filename);

    let model = file_loader::load_file(&filename).unwrap();

    let mut min = Vector3D::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Vector3D::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

    for face in &model {
        min = min.min(face.a);
        min = min.min(face.b);
        min = min.min(face.c);

        max = max.max(face.a);
        max = max.max(face.b);
        max = max.max(face.c);
    }

    let model = BoundingVolumeHierarchy::new(model);
    let model = stack(model);
    println!("BVH with extents {} {}", model.min_extents(), model.max_extents());


    let x_fov: f32 = deg_to_rad(90.0);
    let y_fov: f32 = deg_to_rad(90.0);

    let up = Vector3D::new(0.0, 1.0, 0.0).normalize();
    let forwards = Vector3D::new(0.0, 0.0, 1.0).normalize();
    let left = (forwards ^ up).normalize();

    let model_center = (model.min_extents() + model.max_extents()) / 2.0;

    let dx = model.max_extents().x - model.min_extents().x;
    let dist = (dx / 1.0) / f32::tan(x_fov / 2.0);

    let origin = model_center - (dist * forwards);


    // println!("x = {}, y = {}, z = {}", left, up, forwards);

    let x_dist_left: f32 = f32::tan(x_fov / 2.0);
    let y_dist_up: f32   = f32::tan(y_fov / 2.0);

    // println!("x_dist = {:.2}, y_dist = {:.2}", x_dist_left, y_dist_up);

    let top_center = origin + forwards + y_dist_up * up;

    let top_left = top_center + left * x_dist_left;

    let x_res: usize = 1920;
    let y_res: usize = 1080;

    let right_step = left * x_dist_left / (x_res as f32 / -2.0);
    let down_step = up * y_dist_up / (y_res as f32 / -2.0);


    let light = vec![Vector3D::new(-100.0, 0.0, 0.0)];

    let ambient_intensity = 0.2; // Ia
    let diffuse_reflection_constant = 0.9; // kd
    let specular_reflection_constant = 0.9; // ks
    let transmission_coefficient = 0.0; // kt

    let colour = Vector3D::new(220.0, 220.0, 220.0); // white

    let algorithm = Renderer::Whitted;
    let max_depth = 3;

    let mut data: Vec<u8> = Vec::with_capacity(x_res * y_res * 4);
    for y in 0..y_res {
        for x in 0..x_res {
            let point = top_left + right_step * (x as f32) + down_step * (y as f32);
            let ray = Ray::new(origin, point - origin);
            let i = match algorithm {
                Renderer::Lambert => lambert::trace(&ray, &model),
                Renderer::Whitted => whitted::trace(&ray, &model, &light, ambient_intensity, diffuse_reflection_constant,
                    specular_reflection_constant, transmission_coefficient, max_depth)
                };
            data.push((i * colour.x) as u8);
            data.push((i * colour.y) as u8);
            data.push((i * colour.z) as u8);
            data.push(255);
        }
    }
    // For reading and opening files
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, x_res as u32, y_res as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data[..]).unwrap(); // Save

}
