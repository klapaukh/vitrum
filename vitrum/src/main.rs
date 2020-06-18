use argparse::{ArgumentParser, Store, StoreOption, StoreTrue};

use geometry::{Face, Plane, Ray, Vector3D};

use bvh::BoundingVolumeHierarchy;

use std::f32;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::vec::Vec;

mod stack;
use stack::stack;

mod lambert;
mod whitted;

use enum_from_str::ParseEnumVariantError;
use enum_from_str_derive::FromStr;

// Windowing
use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Copy, Clone, PartialEq, FromStr)]
enum Renderer {
    Whitted,
    Lambert,
}

#[derive(Debug)]
struct World {
    pub camera: Camera,
    pub model: BoundingVolumeHierarchy<Face<f32>, Face<f32>, f32>,
    pub renderer: RenderSetup,
}

#[derive(Debug, Copy, Clone)]
struct RenderSetup {
    pub algorithm: Renderer,
    pub max_depth: u8
}

#[derive(Debug, Copy, Clone)]
struct Camera {
    pub position: Vector3D<f32>,
    pub forwards: Vector3D<f32>,
    pub up: Vector3D<f32>,
}

fn deg_to_rad(deg: f32) -> f32 {
    std::f32::consts::PI * deg / 180.0
}

impl World {
    pub fn render(&self, data: &mut [u8], x_res: usize, y_res: usize) {

        let x_fov: f32 = deg_to_rad(90.0);
        let y_fov: f32 = deg_to_rad(90.0);


        let left = (self.camera.forwards ^ self.camera.up).normalize();

        let x_dist_left: f32 = f32::tan(x_fov / 2.0);
        let y_dist_up: f32 = f32::tan(y_fov / 2.0);

        let right_step = left * x_dist_left / (x_res as f32 / -2.0);
        let down_step = self.camera.up * y_dist_up / (y_res as f32 / -2.0);

        let top_center = self.camera.position + self.camera.forwards + y_dist_up * self.camera.up;

        let top_left = top_center + left * x_dist_left;

        let light = vec![Vector3D::new(-100.0, 0.0, 0.0)];

        let ambient_intensity = 0.2; // Ia
        let diffuse_reflection_constant = 0.9; // kd
        let specular_reflection_constant = 0.9; // ks
        let transmission_coefficient = 0.0; // kt

        let colour = Vector3D::new(20.0, 120.0, 220.0); // white

        for y in 0..y_res {
            for x in 0..x_res {
                let point = top_left + right_step * (x as f32) + down_step * (y as f32);
                let ray = Ray::new(self.camera.position, point - self.camera.position);
                let i = match self.renderer.algorithm {
                    Renderer::Lambert => lambert::trace(&ray, &self.model),
                    Renderer::Whitted => whitted::trace(
                        &ray,
                        &self.model,
                        &light,
                        ambient_intensity,
                        diffuse_reflection_constant,
                        specular_reflection_constant,
                        transmission_coefficient,
                        self.renderer.max_depth,
                    ),
                };
                data[(y * x_res * 4) + (x * 4) + 0] = (i * colour.x) as u8;
                data[(y * x_res * 4) + (x * 4) + 1] = (i * colour.y) as u8;
                data[(y * x_res * 4) + (x * 4) + 2] = (i * colour.z) as u8;
                data[(y * x_res * 4) + (x * 4) + 3] = 255;
            }
        }
    }

    pub fn step_left(&mut self) {
        let left = (self.camera.forwards ^ self.camera.up).normalize();
        self.camera.position = self.camera.position + left;
    }

    pub fn step_right(&mut self) {
        let right = -1.0 * (self.camera.forwards ^ self.camera.up).normalize();
        self.camera.position = self.camera.position + right;
    }

    pub fn step_forwards(&mut self) {
        self.camera.position = self.camera.position + self.camera.forwards;
    }

    pub fn step_back(&mut self) {
        self.camera.position = self.camera.position + -1.0 * self.camera.forwards;
    }
}

fn main() {
    let mut filename: Option<String> = None;
    let mut show_window = false;
    let mut output_filename = String::from("image.png");
    let mut algorithm = Renderer::Lambert;
    let x_res = 1024;
    let y_res = 768;

    let max_depth = 3;


    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Render output to image.png");
        ap.refer(&mut algorithm).add_option(
            &["-a", "--algorithm"],
            Store,
            "Rendering algorithm to use. Options are: Lambert (default), Whitted.",
        );
        ap.refer(&mut filename)
            .add_option(&["-f", "--file"], StoreOption, "File to parse")
            .required();
        ap.refer(&mut output_filename).add_option(
            &["-o", "--output"],
            Store,
            "Output image filename. Format  will always be PNG.",
        );
        ap.refer(&mut show_window).add_option(
            &["-w", "--window"],
            StoreTrue,
            "Show the image in an interactive window instead of writing out to a file",
        );
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
    println!(
        "BVH with extents {} {}",
        model.min_extents(),
        model.max_extents()
    );

    let up = Vector3D::new(0.0, 1.0, 0.0).normalize();
    let forwards = Vector3D::new(0.0, 0.0, 1.0).normalize();

    let model_center = (model.min_extents() + model.max_extents()) / 2.0;

    let dx = model.max_extents().x - model.min_extents().x;
    let dist = (dx / 1.0) / f32::tan(90.0 / 2.0);

    let origin = model_center - (dist * forwards);

    // println!("x = {}, y = {}, z = {}", left, up, forwards);

    // println!("x_dist = {:.2}, y_dist = {:.2}", x_dist_left, y_dist_up);

    let mut world = World {
        camera: Camera {
            position: origin,
            forwards,
            up
        },
        renderer: RenderSetup {
            algorithm,
            max_depth
        },
        model,
    };

    if !show_window {
        let buffer_size = (x_res * y_res * 4) as usize;
        let mut data: Vec<u8> = Vec::with_capacity(buffer_size);
        data.resize(buffer_size, 0);

        world.render(&mut data[..], x_res as usize, y_res as usize);

        // For reading and opening files
        let path = Path::new(&output_filename);
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x_res as u32, y_res as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data[..]).unwrap(); // Save
        return;
    }

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(x_res as f64, y_res as f64);
        WindowBuilder::new()
            .with_title("Vitrum Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut hidpi_factor = window.scale_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new((x_res as f64 / hidpi_factor) as u32, (y_res as f64 / hidpi_factor) as u32, surface);
        Pixels::new(x_res as u32, y_res as u32, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.render(pixels.get_frame(), x_res as usize, y_res as usize);
            if pixels
                .render()
                .map_err(|e| {
                    println!("pixels.render() failed: {}", e);
                    e
                })
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            } else if input.key_pressed(VirtualKeyCode::Left) {
                world.step_left();
            } else if input.key_pressed(VirtualKeyCode::Right) {
                world.step_right();
            } else if input.key_pressed(VirtualKeyCode::Up) {
                world.step_forwards();
            }else if input.key_pressed(VirtualKeyCode::Down) {
                world.step_back();
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
