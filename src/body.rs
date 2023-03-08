use speedy2d::Graphics2D;

use crate::particle::Particle;
use crate::physics::Physics;
use crate::shape::Shape;
use crate::camera::Camera;

pub trait Body {
    fn set_color(&mut self, r: f32, g: f32, b: f32);
    fn draw(&self, graphics: &mut Graphics2D, camera: &Camera);
    fn physics(&mut self) -> &mut Physics;
}

#[derive(Clone)]
pub enum BodyType {
    Shape(Shape),
    Particle(Particle),
}

impl Body for BodyType {
    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        match self {
            BodyType::Shape(s) => s.set_color(r, g, b),
            BodyType::Particle(s) => s.set_color(r, g, b),
        }
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &Camera) {
        match self {
            BodyType::Shape(s) => s.draw(graphics, camera),
            BodyType::Particle(s) => s.draw(graphics, camera),
        }
    }

    fn physics(&mut self) -> &mut Physics {
        match self {
            BodyType::Shape(s) => s.physics(),
            BodyType::Particle(s) => s.physics(),
        }
    }
}
