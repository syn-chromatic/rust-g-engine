use crate::abstracts::body::Body;
use crate::components::physics::Physics;
use crate::components::polygons::Mesh;

#[derive(Clone, Debug)]
pub struct Shape {
    physics: Physics,
}

impl Body for Shape {
    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
    fn mesh(&self) -> &Mesh {
        &self.physics.mesh
    }
}

impl Shape {
    pub fn new(mesh: Mesh) -> Shape {
        let physics: Physics = Physics::new(mesh);
        Shape { physics }
    }
}
