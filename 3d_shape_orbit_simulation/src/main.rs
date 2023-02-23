#![windows_subsystem = "windows"]
use std::{thread};
use std::time::{Duration, Instant};

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

    let window: Window = Window::new_centered(
        "Physics System",
        (canvas_width.round() as u32, canvas_height.round() as u32),
    )
    .unwrap();

    let mut sim_obj: Simulation = simulation::Simulation::new(center_x, center_y);
    sim_obj.setup_objects();

    window.run_loop(MyWindowHandler { sim_obj });
}

struct MyWindowHandler {
    sim_obj: Simulation,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let frame_st = Instant::now();

        let background_color = Color::from_rgb(0.15, 0.15, 0.15);
        graphics.clear_screen(background_color);
        self.sim_obj.compute_objects(graphics);
        thread::sleep(Duration::from_millis(16));

        let frame_time = Instant::now().duration_since(frame_st).as_secs_f32();
        self.sim_obj.write_fps(frame_time, graphics);
        helper.request_redraw();
    }
}
