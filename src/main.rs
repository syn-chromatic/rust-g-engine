#![windows_subsystem = "windows"]
use speedy2d::dimen::Vector2;
use speedy2d::window::MouseScrollDistance;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;
use std::io::{self, Write};
mod body;
mod particle;
mod physics;
mod shape;
mod simulation;
mod vector_3d;
use crate::simulation::Simulation;
use speedy2d::window::VirtualKeyCode;

fn main() {
    let canvas_width: f32 = 960.0;
    let canvas_height: f32 = 768.0;
    let center_x: f64 = canvas_width as f64 / 2.0;
    let center_y: f64 = canvas_height as f64 / 2.0;

    let window: Window = Window::new_centered(
        "Physics System",
        (canvas_width.round() as u32, canvas_height.round() as u32),
    )
    .unwrap();

    let mut simulation: Simulation = Simulation::new(center_x, center_y);
    simulation.setup_objects();
    window.run_loop(MyWindowHandler { simulation });
}

struct MyWindowHandler {
    simulation: Simulation,
}

impl WindowHandler for MyWindowHandler {
    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {

        let position = Vector2::new(100.0, 100.0);
        // _helper.set_position_pixels(position);
        _helper.set_size_scaled_pixels(position);

        let size = Vector2::new(100, 100);
        // _helper.set_size_pixels(size);

        if let Some(VirtualKeyCode::Up) = virtual_key_code {
            self.simulation.scale_up();
        }

        if let Some(VirtualKeyCode::Down) = virtual_key_code {
            self.simulation.scale_down();
        }

        if let Some(VirtualKeyCode::W) = virtual_key_code {
            self.simulation.move_up();
        }

        if let Some(VirtualKeyCode::S) = virtual_key_code {
            self.simulation.move_down();
        }

        if let Some(VirtualKeyCode::D) = virtual_key_code {
            self.simulation.move_right();
        }

        if let Some(VirtualKeyCode::A) = virtual_key_code {
            self.simulation.move_left();
        }
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.simulation.simulate(helper, graphics);
    }
}
