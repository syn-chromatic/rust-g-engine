// #![windows_subsystem = "windows"]
mod body;
mod particle;
mod physics;
mod shape;
mod simulation;
mod vector_3d;
mod vertices;

use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use crate::simulation::Simulation;

fn main() {
    let canvas_resolution: (u32, u32) = (1200, 800);
    let center_x: f64 = canvas_resolution.0 as f64 / 2.0;
    let center_y: f64 = canvas_resolution.1 as f64 / 2.0;
    let center_point: (f64, f64) = (center_x, center_y);

    let window: Window = Window::new_centered("Physics System", canvas_resolution).unwrap();
    let mut simulation: Simulation = Simulation::new(center_point);
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
