use speedy2d::color::Color;
use speedy2d::Graphics2D;

use crate::camera::Camera;
use crate::grid::GridGround;
use crate::particle::Particle;
use crate::physics::Physics;
use crate::shape::Shape;

pub trait Body {
    fn set_color(&mut self, color: Color);
    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera);
    fn physics(&mut self) -> &mut Physics;
}

#[derive(Clone)]
pub enum BodyType {
    Shape(Shape),
    Particle(Particle),
    Grid(GridGround),
}

impl Body for BodyType {
    fn set_color(&mut self, color: Color) {
        match self {
            BodyType::Shape(s) => s.set_color(color),
            BodyType::Particle(s) => s.set_color(color),
            BodyType::Grid(s) => s.set_color(color),

        }
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        match self {
            BodyType::Shape(s) => s.draw(graphics, camera),
            BodyType::Particle(s) => s.draw(graphics, camera),
            BodyType::Grid(s) => s.draw(graphics, camera),
        }
    }

    fn physics(&mut self) -> &mut Physics {
        match self {
            BodyType::Shape(s) => s.physics(),
            BodyType::Particle(s) => s.physics(),
            BodyType::Grid(s) => s.physics(),
        }
    }
}
