use speedy2d::Graphics2D;

use crate::physics::Physics;

pub trait Body: BodyClone {
    fn set_color(&mut self, r: f32, g: f32, b: f32);
    fn draw_shape(&self, graphics: &mut Graphics2D);
    fn physics(&self) -> &Physics;
    fn mutable_physics(&mut self) -> &mut Physics;
}

pub trait BodyClone {
    fn clone_box(&self) -> Box<dyn Body>;
}

impl<T> BodyClone for T
where
    T: 'static + Body + Clone,
{
    fn clone_box(&self) -> Box<dyn Body> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Body> {
    fn clone(&self) -> Box<dyn Body> {
        self.clone_box()
    }
}
