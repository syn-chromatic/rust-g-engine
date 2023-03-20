use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
use crate::camera::Camera;
use crate::physics::Physics;
use crate::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct Particle {
    physics: Physics,
    color: Color,
}

impl Body for Particle {
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        self.draw_circle(graphics, camera);
    }

    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
}

impl Particle {
    pub fn new(shape: Vec<[f64; 3]>) -> Particle {
        let physics: Physics = Physics::new(shape.clone());
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        Particle { physics, color }
    }

    fn draw_circle(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        let position = self.get_particle_position();
        let scale = self.get_particle_scale();

        let point1 = Vector3D::new(position.x - scale, position.y, position.z);
        let point2 = Vector3D::new(position.x + scale, position.y, position.z);

        let proj1 = camera.get_screen_coordinates(point1);
        let proj2 = camera.get_screen_coordinates(point2);

        if proj1.is_none() || proj2.is_none() {
            return;
        }

        let proj1 = proj1.unwrap();
        let proj2 = proj2.unwrap();

        let p_scale = proj1.subtract_vector(proj2).get_length() / 2.0;
        let mid_point = proj1.get_midpoint(proj2);
        let point = (mid_point.x as f32, mid_point.y as f32);

        let alpha = self.get_scale_alpha(p_scale);
        let rgb = self.get_rgb_values();
        let color = Color::from_rgba(rgb.0, rgb.1, rgb.2, alpha);
        let p_scale = p_scale as f32;

        graphics.draw_circle(point, p_scale, self.color)
    }

    fn get_scale_alpha(&self, scale: f64) -> f32 {
        let max_scale: f32 = 300.0;
        let min_scale: f32 = max_scale / 2.0;
        if scale < min_scale as f64 {
            return 1.0;
        }
        let alpha_normalized = (scale as f32 - min_scale) / (max_scale - min_scale);
        let alpha_clamped = alpha_normalized.clamp(0.0, 1.0);
        let alpha = 1.0 - alpha_clamped;
        alpha
    }

    fn get_rgb_values(&self) -> (f32, f32, f32) {
        let r: f32 = self.color.r();
        let g: f32 = self.color.g();
        let b: f32 = self.color.b();
        (r, g, b)
    }

    fn get_shaded_rgb(&self, rgb: (f32, f32, f32), shade_value: f32) -> (f32, f32, f32) {
        let rgb: (f32, f32, f32) = (
            rgb.0 * shade_value,
            rgb.1 * shade_value,
            rgb.2 * shade_value,
        );
        rgb
    }

    fn get_particle_position(&self) -> Vector3D {
        self.physics.position
    }

    fn get_particle_scale(&self) -> f64 {
        self.physics.scale
    }
}
