// #![windows_subsystem = "windows"]
mod body;
mod camera;
mod debug;
mod particle;
mod physics;
mod shape;
mod simulation;
mod vector_3d;
mod vertices;

use speedy2d::dimen::Vector2;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;
use speedy2d::window::MouseScrollDistance;
use speedy2d::window::MouseScrollDistance::Lines;

use crate::camera::Camera;
use crate::simulation::Simulation;

fn main() {
    let width: u32 = 1200;
    let height: u32 = 800;
    let canvas_resolution: (u32, u32) = (width, height);
    let center_x: f64 = canvas_resolution.0 as f64 / 2.0;
    let center_y: f64 = canvas_resolution.1 as f64 / 2.0;
    let center_point: (f64, f64) = (center_x, center_y);

    let window: Window = Window::new_centered("Physics System", canvas_resolution).unwrap();
    let camera = Camera::new(width, height);

    let mut simulation: Simulation = Simulation::new(camera, center_point);
    // simulation.setup_collision_configuration();
    simulation.setup_gravity_configuration();
    window.run_loop(MyWindowHandler { simulation });
}

struct MyWindowHandler {
    simulation: Simulation,
}

impl WindowHandler for MyWindowHandler {
    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<()>,
        distance: MouseScrollDistance,
    ) {

        if let MouseScrollDistance::Lines { x, y, z } = distance {
            self.simulation.camera.increment_distance(y);
        }
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        let dx = position.x as f64;
        let dy = position.y as f64;
        self.simulation.camera.handle_mouse_movement(dx, dy);
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        // let position = Vector2::new(100.0, 100.0);
        // _helper.set_position_pixels(position);
        // _helper.set_size_scaled_pixels(position);s

        // let size = Vector2::new(100, 100);
        // _helper.set_size_pixels(size);

        // if let Some(VirtualKeyCode::Up) = virtual_key_code {
        //     self.simulation.camera.increase_near_plane(0.1);
        // }

        // if let Some(VirtualKeyCode::Down) = virtual_key_code {
        //     self.simulation.camera.decrease_near_plane(0.1);
        // }

        // if let Some(VirtualKeyCode::W) = virtual_key_code {
        //     self.simulation.camera.increase_distance(1.0);
        // }

        // if let Some(VirtualKeyCode::S) = virtual_key_code {
        //     self.simulation.camera.decrease_distance(1.0);
        // }

        // if let Some(VirtualKeyCode::D) = virtual_key_code {
        //     self.simulation.move_right();
        // }

        // if let Some(VirtualKeyCode::A) = virtual_key_code {
        //     self.simulation.move_left();
        // }
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.simulation.simulate(helper, graphics);
    }
}
