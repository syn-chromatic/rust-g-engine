use crate::components::physics::Physics;
use crate::components::polygons::Mesh;
use crate::components::shape::Shape;

pub trait Body {
    fn physics(&self) -> &Physics;
    fn physics_mut(&mut self) -> &mut Physics;
    fn mesh(&self) -> &Mesh;
    fn mesh_cluster(&self) -> &Option<Vec<Mesh>>;
}

#[derive(Clone, Debug)]
pub enum BodyType {
    Shape(Shape),
}

impl Body for BodyType {
    fn physics(&self) -> &Physics {
        match self {
            BodyType::Shape(s) => s.physics(),
        }
    }

    fn physics_mut(&mut self) -> &mut Physics {
        match self {
            BodyType::Shape(s) => s.physics_mut(),
        }
    }

    fn mesh(&self) -> &Mesh {
        match self {
            BodyType::Shape(s) => s.mesh(),
        }
    }

    fn mesh_cluster(&self) -> &Option<Vec<Mesh>> {
        match self {
            BodyType::Shape(s) => s.mesh_cluster(),
        }
    }
}
