use crate::components::physics::Physics;
use crate::components::polygons::Mesh;
use crate::components::shape::Shape;

pub trait Body {
    fn physics(&mut self) -> &mut Physics;
    fn mesh_cluster(&self) -> &Vec<Mesh>;
}

#[derive(Clone, Debug)]
pub enum BodyType {
    Shape(Shape),
}

impl Body for BodyType {
    fn physics(&mut self) -> &mut Physics {
        match self {
            BodyType::Shape(s) => s.physics(),
        }
    }
    fn mesh_cluster(&self) -> &Vec<Mesh> {
        match self {
            BodyType::Shape(s) => s.mesh_cluster(),
        }
    }
}
