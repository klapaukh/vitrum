use argparse::{ArgumentParser, Store, StoreOption, StoreTrue};

use geometry::{Face, Plane, Ray, Vec3};

use nalgebra::{Rotation3, Unit};

use bvh::BoundingVolumeHierarchy;

use std::f64;
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
    pub model: BoundingVolumeHierarchy<Face>,
    pub renderer: RenderSetup,
}

#[derive(Debug, Copy, Clone)]
struct RenderSetup {
    pub algorithm: Renderer,
    pub max_depth: u8
}

#[derive(Debug, Copy, Clone)]
struct Camera {
    pub position: Vec3,
    pub forwards: Vec3,
    pub up: Vec3,
}

fn deg_to_rad(deg: f64) -> f64 {
    std::f64::consts::PI * deg / 180.0
}

impl World {
    pub fn render(&self, data: &mut [u8], x_res: usize, y_res: usize) {
        //print!("Rendering start - ");
        let y_fov: f64 = deg_to_rad(90.0);

        let left = self.camera.forwards.cross(&self.camera.up).normalize();

        let y_dist_up: f64 = f64::tan(y_fov / 2.0);
        let x_dist_left: f64 = y_dist_up * x_res as f64 / y_res as f64;

        let right_step = left * x_dist_left / (x_res as f64 / -2.0);
        let down_step = self.camera.up * y_dist_up / (y_res as f64 / -2.0);

        let top_center = self.camera.position + self.camera.forwards + y_dist_up * self.camera.up;

        let top_left = top_center + left * x_dist_left;

        let light = vec![Vec3::new(-100.0, 0.0, 0.0)];

        let ambient_intensity = 0.2; // Ia
        let diffuse_reflection_constant = 0.9; // kd
        let specular_reflection_constant = 0.9; // ks
        let transmission_coefficient = 0.0; // kt

        let colour = Vec3::new(20.0, 120.0, 220.0); // white

        for y in 0..y_res {
            for x in 0..x_res {
                let point = top_left + right_step * (x as f64) + down_step * (y as f64);
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
                data[(y * x_res * 4) + (x * 4)    ] = (i * colour.x) as u8;
                data[(y * x_res * 4) + (x * 4) + 1] = (i * colour.y) as u8;
                data[(y * x_res * 4) + (x * 4) + 2] = (i * colour.z) as u8;
                data[(y * x_res * 4) + (x * 4) + 3] = 255;
            }
        }

        //println!("complete");
    }

    pub fn step_left(&mut self) {
        let left = self.camera.forwards.cross(&self.camera.up).normalize();
        self.camera.position += left;
    }

    pub fn step_right(&mut self) {
        let right = -1.0 * self.camera.forwards.cross(&self.camera.up).normalize();
        self.camera.position += right;
    }

    pub fn step_forwards(&mut self) {
        self.camera.position += self.camera.forwards;
    }

    pub fn step_back(&mut self) {
        self.camera.position -= self.camera.forwards;
    }

    pub fn rotate(&mut self, rot: Rotation3<f64>, screen_ratio: f64) {
        let y_fov: f64 = deg_to_rad(90.0);

        let left = self.camera.forwards.cross(&self.camera.up).normalize();

        let y_dist_up: f64 = f64::tan(y_fov / 2.0);
        let x_dist_left: f64 = y_dist_up * screen_ratio;

        let forwards_dist = f64::min(y_dist_up, x_dist_left).abs();
        let center =  self.camera.position +  self.camera.forwards * forwards_dist;

        self.camera.position = rot * (self.camera.position - center) + center;
        self.camera.up       = rot * self.camera.up;
        self.camera.forwards = rot * self.camera.forwards;
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

    let model = BoundingVolumeHierarchy::new(model);
    //let model = stack(model);
    println!(
        "BVH has {} faces with extents {} {}",
        model.size(),
        model.min_extents(),
        model.max_extents()
    );

    let up = Vec3::new(0.0, 1.0, 0.0).normalize();
    let forwards = Vec3::new(0.0, 0.0, 1.0).normalize();

    let model_center = (model.min_extents() + model.max_extents()) / 2.0;

    let dx = model.max_extents().x - model.min_extents().x;
    let dist = (dx / 1.0) / f64::tan(90.0 / 2.0);
    let dist = (model.max_extents() - model.min_extents()).norm();

    let origin = model_center - (dist * forwards);


    // println!("x = {}, y = {}, z = {}", left, up, forwards);

    // println!("origin = {:.2}, forwards = {:.2}", origin, forwards);

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
        let mut data: Vec<u8> = vec![0; buffer_size];

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

    let mut last_mouse_pos: Option<Vec3> = None;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.render(pixels.get_frame(), x_res as usize, y_res as usize);
            if let Err(e) = pixels.render() {
                println!("pixels.render() failed: {}", e);
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

            if input.mouse_pressed(0) {
                println!("Click!");
                if let Some((x,y)) = input.mouse() {
                    last_mouse_pos = Some(screen_to_sphere(x as f64, y as f64, x_res as f64, y_res as f64));
                }
            } else if input.mouse_held(0){
                println!("Hold!");
                if let Some((x,y)) = input.mouse() {
                    let current_mouse_pos = screen_to_sphere(x as f64, y as f64, x_res as f64, y_res as f64);
                    if let Some(p1) = last_mouse_pos {
                        // Do a movement
                        arcball_rotate(&p1, &current_mouse_pos, &mut world, x_res as f64/ y_res as f64);
                    }
                    last_mouse_pos = Some(current_mouse_pos);
                }
            } else if input.mouse_released(0) {
                println!("Release!");
                if let Some((x,y)) = input.mouse() {
                    let current_mouse_pos = screen_to_sphere(x as f64, y as f64, x_res as f64, y_res as f64);
                    if let Some(p1) = last_mouse_pos {
                        // Do a movement
                        arcball_rotate(&p1, &current_mouse_pos, &mut world, x_res as f64/ y_res as f64);
                    }
                }
                last_mouse_pos = None;
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

/// Convert xy screen coordinates to a unit sphere mapped to the screen for arc ball
fn screen_to_sphere(x: f64, y: f64, x_res: f64, y_res: f64) -> Vec3 {

    print!("Mouse at point ({:0.1}, {:0.1}) -> ", x, y);

    let x = 2.0 * (x / x_res as f64 - 0.5);
    let y = -2.0 * (y / y_res as f64 - 0.5);

    let sum_sq = x*x + y*y;
    let z = if sum_sq >  1.0 {
            0.0
        } else {
            f64::sqrt(1.0 - sum_sq)
        };

    println!("({:0.2}, {:0.2}, {:0.2}) {:0.2}", x, y, z, sum_sq);
    Vec3::new(x,y,z)
}

fn arcball_rotate(start: &Vec3, end: &Vec3, world: &mut World, screen_ratio: f64) {
    // This assumes UP is the y axis, which isn't right, it is the camera up
    // Align up and left with the camera

    let up = world.camera.up;
    let forwards = world.camera.forwards;
    let left = up.cross(&forwards).normalize();

    let start = start.x * left + start.y * up + start.z * forwards;
    let end   =   end.x * left +   end.y * up +   end.z * forwards;

    let axis = Unit::new_normalize(start.cross(&end));
    let angle: f64 = f64::acos(f64::min(1.0, start.dot(&end)/ (start.norm() * end.norm())));

    //println!("Arcballing {:?} -> {:?} Giving {:0.3} around {:?}", start, end, angle, axis);
    if angle < 1e-2 {
        return;
    }

    let rotation = Rotation3::from_axis_angle(&axis, angle);
    world.rotate(rotation, screen_ratio);
}