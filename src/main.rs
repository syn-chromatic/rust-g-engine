#![windows_subsystem = "windows"]
mod abstracts;
mod components;
mod configurations;
mod debug;

use speedy2d::dimen::Vec2;
use speedy2d::window::KeyScancode;

use speedy2d::dimen::UVec2;
use speedy2d::window::MouseButton;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::WindowFullscreenMode;
use speedy2d::window::WindowHandler;
use speedy2d::window::WindowHelper;
use speedy2d::Graphics2D;
use speedy2d::Window;

use crate::components::camera::Camera;
use crate::components::draw_call::DrawCall;
use crate::components::graphics::Graphics;
use crate::components::simulation::Simulation;

fn main() {
    let width: u32 = 1760;
    let height: u32 = 960;
    let resolution: (u32, u32) = (width, height);

    let window: Window = Window::new_centered("G-Engine", resolution).unwrap();
    let mut camera: Camera = Camera::new(width, height);
    camera.calibrate();

    let mut simulation: Simulation = Simulation::new(camera, resolution);
    simulation.setup_objects();

    let graphics: Graphics = Graphics::new(width, height);
    let draw_call = DrawCall::new(graphics, simulation);
    window.run_loop(draw_call);
}

impl WindowHandler for DrawCall {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        _info: speedy2d::window::WindowStartupInfo,
    ) {
        self.graphics.set_cursor_grab(true);
        helper.set_cursor_visible(false);
        helper.set_fullscreen_mode(WindowFullscreenMode::Windowed);
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size: UVec2) {
        let width = size.x;
        let height = size.y;
        self.graphics.set_screensize(width, height);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics_2d: &mut Graphics2D) {
        let fps = self.frame_timing.get_frames_per_second();
        let graphics = &mut self.graphics;
        graphics.execute_helper_functions(helper);
        graphics.execute_buffer(graphics_2d);
        graphics.clear_screen();
        self.simulation.simulate(graphics, fps);
        let objects = self.simulation.objects.clone();
        self.draw(objects);
        self.frame_timing.tick();
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vec2) {
        let dx: f64 = position.x as f64;
        let dy: f64 = position.y as f64;
        let cursor_grab = self.graphics.get_cursor_grab();
        if !cursor_grab.is_grabbed || cursor_grab.first_pass {
            return;
        }
        let camera: &mut Camera = &mut self.simulation.camera;
        camera.handle_mouse_movement(dx, dy);
    }

    fn on_mouse_button_down(
        &mut self,
        _helper: &mut WindowHelper,
        button: speedy2d::window::MouseButton,
    ) {
        if let MouseButton::Left = button {
            self.graphics.set_cursor_grab(true);
            self.simulation.shoot();
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(VirtualKeyCode::Period) = virtual_key_code {
            self.simulation.increment_timestep(1);
        }

        if let Some(VirtualKeyCode::Comma) = virtual_key_code {
            self.simulation.increment_timestep(-1);
        }

        let step_val = 50000.0;
        let camera = &mut self.simulation.camera;

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

        if let Some(VirtualKeyCode::Up) = virtual_key_code {
            camera.increment_position_y(step_val);
        }

        if let Some(VirtualKeyCode::Down) = virtual_key_code {
            camera.increment_position_y(-step_val);
        }

        if let Some(VirtualKeyCode::Y) = virtual_key_code {
            camera.toggle_y_lock();
        }

        if let Some(VirtualKeyCode::RWin) = virtual_key_code {
            self.graphics.set_cursor_grab(false);
        }

        if let Some(VirtualKeyCode::LWin) = virtual_key_code {
            self.graphics.set_cursor_grab(false);
        }
    }
}
