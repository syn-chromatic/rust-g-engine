use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
use crate::camera::Camera;
use crate::physics::Physics;

#[derive(Clone, Debug)]
pub struct Particle {
    pub physics: Physics,
    color: Color,
}

impl Body for Particle {
    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Color::from_rgb(r, g, b);
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &Camera) {
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

    fn get_rgb_values(&self, color: Color) -> (f32, f32, f32) {
        let r: f32 = self.color.r();
        let g: f32 = self.color.g();
        let b: f32 = self.color.b();
        (r, g, b)
    }

    fn get_relative_z(&self) -> f64 {
        let z: f64 = self.physics.position.z;
        let scale: f64 = self.physics.scale;
        let mut relative_z: f64 = scale + z;
        relative_z = f64::max(0.5, relative_z).min(f64::INFINITY);
        relative_z
    }

    fn draw_circle(&self, graphics: &mut Graphics2D, camera: &Camera) {
        let position = camera.perspective_projection(self.physics.position);
        let x = position.x;
        let y = position.y;
        let radius = camera.interpolate_radius(self.physics.position, self.physics.scale);

        let rgb: (f32, f32, f32) = self.get_rgb_values(self.color);
        let alpha: f32 = self.get_scale_alpha(radius);
        let color: Color = Color::from_rgba(rgb.0, rgb.1, rgb.2, alpha);

        let p: Vector2<f32> = Vector2::new(x, y).into_f32();
        graphics.draw_circle(p, radius as f32, color);
    }
}
