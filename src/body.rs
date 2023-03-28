use speedy2d::color::Color;
use speedy2d::Graphics2D;

use crate::camera::Camera;
use crate::physics::Physics;
use crate::shape::Shape;

pub trait Body {
    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera);
    fn physics(&mut self) -> &mut Physics;
}

#[derive(Clone)]
pub enum BodyType {
    Shape(Shape),
}

impl Body for BodyType {
    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        match self {
            BodyType::Shape(s) => s.draw(graphics, camera),
        }
    }

    fn physics(&mut self) -> &mut Physics {
        match self {
            BodyType::Shape(s) => s.physics(),
        }
    }
}
