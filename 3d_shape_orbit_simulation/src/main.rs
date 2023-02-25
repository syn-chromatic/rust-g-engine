#![windows_subsystem = "windows"]
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

mod physics;
mod shape;
mod simulation;
mod vector_3d;

use crate::simulation::Simulation;

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
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.simulation.simulate(helper, graphics);
    }
}
