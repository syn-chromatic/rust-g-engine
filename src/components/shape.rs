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
    fn mesh_cluster(&self) -> &Vec<Mesh> {
        &self.physics.mesh_cluster
    }
}

impl Shape {
    pub fn new(meshes: Vec<Mesh>) -> Shape {
        let physics: Physics = Physics::new(meshes);
        Shape { physics }
    }
}
