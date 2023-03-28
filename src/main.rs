
mod body;
mod camera;
mod color;
mod configurations;
mod debug;
mod font;
mod frustum;
mod model;
mod physics;
mod polygons;
mod shaders;
mod shape;
mod simulation;
mod text_writer;
mod vectors;
mod vertices;

use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::KeyScancode;
use speedy2d::window::MouseScrollDistance;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use std::collections::VecDeque;
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
    let frame_timing = FrameTimeHandler::new(30);
    simulation.setup_objects();

    window.run_loop(MyWindowHandler {
        simulation,
        background_color,
        frame_timing,
    });
}

pub struct FrameTimeHandler {
    frame_times: VecDeque<Duration>,
    frame_start: Instant,
    frame_count: usize,
}

impl FrameTimeHandler {
    pub fn new(frame_count: usize) -> FrameTimeHandler {
        FrameTimeHandler {
            frame_times: VecDeque::with_capacity(frame_count),
            frame_start: Instant::now(),
            frame_count,
        }
    }

    pub fn tick(&mut self) {
        let frame_time = Instant::now().duration_since(self.frame_start);
        self.frame_start = Instant::now();

        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.frame_count {
            self.frame_times.pop_front();
        }
    }

    pub fn get_average_frame_time(&self) -> Option<Duration> {
        if self.frame_times.is_empty() {
            None
        } else {
            let total_time = self.frame_times.iter().sum::<Duration>();
            Some(total_time / self.frame_times.len() as u32)
        }
    }

    pub fn get_frames_per_second(&self) -> f64 {
        if let Some(average_frame_time) = self.get_average_frame_time() {
            1.0 / average_frame_time.as_secs_f64()
        } else {
            0.0
        }
    }
}
struct MyWindowHandler {
    simulation: Simulation,
    background_color: Color,
    frame_timing: FrameTimeHandler,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let fps = self.frame_timing.get_frames_per_second();
        graphics.clear_screen(self.background_color);
        self.simulation.simulate(graphics, fps);
        self.frame_timing.tick();
        helper.request_redraw();
    }

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
}
