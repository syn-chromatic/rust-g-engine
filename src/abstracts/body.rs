use crate::components::camera::Camera;
use crate::components::physics::Physics;
use crate::components::shape::Shape;
use crate::components::graphics::Graphics;


pub trait Body {
    fn draw(&self, graphics: &mut Graphics, camera: &mut Camera, path_trace: bool, bounce_count: usize);
    fn physics(&mut self) -> &mut Physics;
}

#[derive(Clone)]
pub enum BodyType {
    Shape(Shape),
}

impl Body for BodyType {
    fn draw(&self, graphics: &mut Graphics, camera: &mut Camera, path_trace: bool, bounce_count: usize) {
        match self {
            BodyType::Shape(s) => s.draw(graphics, camera, path_trace, bounce_count),
        }
    }

    fn physics(&mut self) -> &mut Physics {
        match self {
            BodyType::Shape(s) => s.physics(),
        }
    }
}
