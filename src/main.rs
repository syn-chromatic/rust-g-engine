// #![windows_subsystem = "windows"]
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

use std::collections::HashSet;

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
    let draw_call: DrawCall = DrawCall::new(graphics, simulation);
    let handler: Handler = Handler::new(draw_call);

    window.run_loop(handler);
}

pub struct Handler {
    draw_call: DrawCall,
    keycodes: HashSet<VirtualKeyCode>,
}

impl Handler {
    pub fn new(draw_call: DrawCall) -> Handler {
        let keys: HashSet<VirtualKeyCode> = HashSet::new();

        Self {
            draw_call,
            keycodes: keys,
        }
    }

    pub fn add_keycode(&mut self, key: VirtualKeyCode) {
        self.keycodes.insert(key);
    }

    pub fn remove_keycode(&mut self, key: &VirtualKeyCode) {
        self.keycodes.remove(key);
    }

    pub fn check_keycode(keys: &HashSet<VirtualKeyCode>, key: VirtualKeyCode) -> bool {
        keys.contains(&key)
    }

    pub fn address_keycodes(&mut self) {
        let keys: &HashSet<VirtualKeyCode> = &self.keycodes;

        if Self::check_keycode(keys, VirtualKeyCode::Period) {
            self.draw_call.simulation.increment_timestep(1);
        }

        if Self::check_keycode(keys, VirtualKeyCode::Comma) {
            self.draw_call.simulation.increment_timestep(-1);
        }

        let step_val: f64 = 50_000.0;
        let camera: &mut Camera = &mut self.draw_call.simulation.camera;

        if Self::check_keycode(keys, VirtualKeyCode::W) {
            camera.increment_position_z(step_val);
        }

        if Self::check_keycode(keys, VirtualKeyCode::A) {
            camera.increment_position_x(-step_val);
        }

        if Self::check_keycode(keys, VirtualKeyCode::S) {
            camera.increment_position_z(-step_val);
        }

        if Self::check_keycode(keys, VirtualKeyCode::D) {
            camera.increment_position_x(step_val);
        }

        if Self::check_keycode(keys, VirtualKeyCode::Up) {
            camera.increment_position_y(step_val);
        }

        if Self::check_keycode(keys, VirtualKeyCode::Down) {
            camera.increment_position_y(-step_val);
        }

        if Self::check_keycode(keys, VirtualKeyCode::Y) {
            camera.toggle_y_lock();
        }

        if Self::check_keycode(keys, VirtualKeyCode::RWin)
            || Self::check_keycode(keys, VirtualKeyCode::LWin)
        {
            self.draw_call.graphics.set_cursor_grab(false);
        }

        if Self::check_keycode(keys, VirtualKeyCode::RBracket) {
            self.draw_call.simulation.toggle_draw_polygons();
        }
    }
}

impl WindowHandler for Handler {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        _info: speedy2d::window::WindowStartupInfo,
    ) {
        self.draw_call.graphics.set_cursor_grab(true);
        helper.set_cursor_visible(false);
        helper.set_fullscreen_mode(WindowFullscreenMode::Windowed);
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size: UVec2) {
        let width: u32 = size.x;
        let height: u32 = size.y;
        self.draw_call.graphics.set_screensize(width, height);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics_2d: &mut Graphics2D) {
        self.address_keycodes();
        let fps: f64 = self.draw_call.frame_timing.get_frames_per_second();
        let graphics: &mut Graphics = &mut self.draw_call.graphics;
        graphics.execute_helper_functions(helper);
        graphics.execute_buffer(graphics_2d);
        graphics.clear_screen();
        self.draw_call.simulation.simulate(graphics, fps);
        self.draw_call.draw();
        self.draw_call.frame_timing.tick();
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vec2) {
        let dx: f64 = position.x as f64;
        let dy: f64 = position.y as f64;
        let cursor_grab = self.draw_call.graphics.get_cursor_grab();
        if !cursor_grab.is_grabbed || cursor_grab.first_pass {
            return;
        }
        let camera: &mut Camera = &mut self.draw_call.simulation.camera;
        camera.handle_mouse_movement(dx, dy);
    }

    fn on_mouse_button_down(
        &mut self,
        _helper: &mut WindowHelper,
        button: speedy2d::window::MouseButton,
    ) {
        if let MouseButton::Left = button {
            let cursor_grab = self.draw_call.graphics.get_cursor_grab();
            if !cursor_grab.is_grabbed {
                self.draw_call.graphics.set_cursor_grab(true);
                return;
            }
            self.draw_call.simulation.shoot();
        }
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(keycode) = virtual_key_code {
            self.add_keycode(keycode);
        }
    }

    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(keycode) = virtual_key_code {
            self.remove_keycode(&keycode);
        }
    }
}
