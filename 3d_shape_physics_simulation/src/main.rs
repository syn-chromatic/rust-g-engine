#![windows_subsystem = "windows"]
use std::{thread, time::Duration};

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

mod shape;
mod simulation;
mod vector_3d;
use crate::simulation::Simulation;


fn main() {
    let canvas_width: f32 = 960.0;
    let canvas_height: f32 = 768.0;
    let center_x: f64 = canvas_width as f64 / 2.0;
    let center_y: f64 = canvas_height as f64 / 2.0;

    let window = Window::new_centered(
        "Physics System",
        (canvas_width.round() as u32, canvas_height.round() as u32),
    )
    .unwrap();

    let mut sim_obj: Simulation = simulation::Simulation::new(center_x, center_y);
    sim_obj.create_center_object();

    for _ in 0..200 {
        sim_obj.create_orbiting_object();
    }

    window.run_loop(MyWindowHandler {
        sim_obj: sim_obj,
    });
}

struct MyWindowHandler {
    sim_obj: Simulation,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let background_color = Color::from_rgb(0.15, 0.15, 0.15);
        graphics.clear_screen(background_color);
        self.sim_obj.compute_objects(graphics);
        thread::sleep(Duration::from_millis(16));
        helper.request_redraw();
    }
}
