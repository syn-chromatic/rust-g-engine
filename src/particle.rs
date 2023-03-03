use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
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

    fn draw_shape(&self, graphics: &mut Graphics2D) {
        self.draw_circle(graphics);
    }

    fn physics(&self) -> &Physics {
        &self.physics
    }

    fn mutable_physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
}

impl Particle {
    pub fn new(shape: Vec<[f64; 3]>) -> Particle {
        let physics: Physics = Physics::new(shape.clone());
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        Particle { physics, color }
    }

    fn draw_circle(&self, graphics: &mut Graphics2D) {
        let x: f64 = self.physics.position.x;
        let y: f64 = self.physics.position.y;
        let z: f64 = self.physics.position.z;

        let scale: f64 = self.physics.scale;
        let mut relative_z: f64 = scale + z;
        relative_z = f64::max(0.5, relative_z).min(f64::INFINITY);

        let p: Vector2<f32> = Vector2::new(x, y).into_f32();
        graphics.draw_circle(p, relative_z as f32, self.color);
    }
}
