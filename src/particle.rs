use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
use crate::camera::Camera;
use crate::physics::Physics;
use crate::vector_3d::Vector3D;

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

    fn get_rgb_values(&self) -> (f32, f32, f32) {
        let r: f32 = self.color.r();
        let g: f32 = self.color.g();
        let b: f32 = self.color.b();
        (r, g, b)
    }

    fn get_particle_position(&self) -> Vector3D {
        self.physics.position
    }

    fn get_particle_scale(&self) -> f64 {
        self.physics.scale
    }

    fn draw_circle(&self, graphics: &mut Graphics2D, camera: &Camera) {
        let position: Vector3D = self.get_particle_position();
        let scale: f64 = self.get_particle_scale();

        let projected: Vector3D = camera.perspective_projection(position);
        let radius: f64 = camera.interpolate_radius(projected, scale);

        let rgb: (f32, f32, f32) = self.get_rgb_values();
        let alpha: f32 = self.get_scale_alpha(radius);
        let color: Color = Color::from_rgba(rgb.0, rgb.1, rgb.2, alpha);

        let x: f64 = projected.x;
        let y: f64 = projected.y;
        let p: Vector2<f32> = Vector2::new(x, y).into_f32();
        graphics.draw_circle(p, radius as f32, color);
    }
}
