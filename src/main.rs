// #![windows_subsystem = "windows"]

mod body;
mod camera;
mod color;
mod debug;
mod font;
mod frustum;
mod physics;
mod polygons;
mod shape;
mod simulation;
mod text_writer;
mod vectors;
mod vertices;
mod shaders;
mod configurations;
mod model;

use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::KeyScancode;
use speedy2d::window::MouseScrollDistance;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use std::time::Duration;
use std::time::Instant;

use crate::camera::Camera;
use crate::simulation::Simulation;

fn main() {
    let width: u32 = 1920;
    let height: u32 = 1080;
    let resolution: (u32, u32) = (width, height);

    let window: Window = Window::new_centered("Physics System", resolution).unwrap();
    let camera = Camera::new(width, height);

    let mut simulation: Simulation = Simulation::new(camera, resolution);
    let background_color = Color::from_rgb(0.15, 0.15, 0.15);
    let frame_timer = FrameTimeHandler::new();
    simulation.setup_objects();

    window.run_loop(MyWindowHandler {
        simulation,
        background_color,
        frame_timer,
    });
}

struct FrameTimeHandler {
    frame_st: Instant,
    frame_time: Duration,
}

impl FrameTimeHandler {
    pub fn new() -> FrameTimeHandler {
        let frame_st: Instant = Instant::now();
        let frame_time: Duration = Instant::now().duration_since(frame_st);

        FrameTimeHandler {
            frame_st,
            frame_time,
        }
    }

    pub fn set_frame_timing(&mut self) {
        self.frame_time = Instant::now().duration_since(self.frame_st);
        self.frame_st = Instant::now();
    }

    pub fn get_frame_time(&self) -> f32 {
        let frame_time: f32 = self.frame_time.as_secs_f32();
        frame_time
    }
}

struct MyWindowHandler {
    simulation: Simulation,
    background_color: Color,
    frame_timer: FrameTimeHandler,
}

impl WindowHandler for MyWindowHandler {
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {}

    fn on_mouse_wheel_scroll(&mut self, _: &mut WindowHelper, distance: MouseScrollDistance) {
        if let MouseScrollDistance::Lines { y, .. } = distance {
            self.simulation.camera.increment_planes(y);
        }
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        let dx = position.x as f64;
        let dy = position.y as f64;
        self.simulation.camera.handle_mouse_movement(dx, dy);
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        let step_val = 80.0;
        let mut camera = &mut self.simulation.camera;

        if let Some(VirtualKeyCode::W) = virtual_key_code {
            camera.increment_position_z(step_val);
        }

        if let Some(VirtualKeyCode::S) = virtual_key_code {
            camera.increment_position_z(-step_val);
        }

        if let Some(VirtualKeyCode::D) = virtual_key_code {
            camera.increment_position_x(step_val);
        }

        if let Some(VirtualKeyCode::A) = virtual_key_code {
            camera.increment_position_x(-step_val);
        }
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.frame_timer.set_frame_timing();
        let frame_time: f32 = self.frame_timer.get_frame_time();

        graphics.clear_screen(self.background_color);
        self.simulation.simulate(graphics, frame_time);
        helper.request_redraw();
    }
}
