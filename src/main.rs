// #![windows_subsystem = "windows"]
mod body;
mod camera;
mod debug;
mod particle;
mod physics;
mod shape;
mod simulation;
mod vectors;
mod vertices;
mod grid;
mod text_writer;
mod color;


use speedy2d::dimen::Vec2;
use speedy2d::window::KeyScancode;
use speedy2d::window::MouseScrollDistance;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use crate::camera::Camera;
use crate::simulation::Simulation;

fn main() {
    let width: u32 = 1920;
    let height: u32 = 1080;
    let canvas_resolution: (u32, u32) = (width, height);
    let center_x: f64 = canvas_resolution.0 as f64 / 2.0;
    let center_y: f64 = canvas_resolution.1 as f64 / 2.0;
    let center_point: (f64, f64) = (center_x, center_y);

    let window: Window = Window::new_centered("Physics System", canvas_resolution).unwrap();
    let camera = Camera::new(width, height);

    let mut simulation: Simulation = Simulation::new(camera, canvas_resolution);
    simulation.setup_collision_configuration();
    // simulation.setup_gravity_configuration();
    window.run_loop(MyWindowHandler { simulation });
}

struct MyWindowHandler {
    simulation: Simulation,
}

impl WindowHandler for MyWindowHandler {

    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {

    }

    fn on_mouse_wheel_scroll(&mut self, _: &mut WindowHelper, distance: MouseScrollDistance) {
        if let MouseScrollDistance::Lines { y, .. } = distance {
            self.simulation.camera.increment_distance(y);
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
        self.simulation.simulate(graphics);
        helper.request_redraw();
    }
}
