use crate::abstracts::body::Body;
use crate::components::physics::Physics;
use crate::components::polygons::Mesh;

#[derive(Clone, Debug)]
pub struct Shape {
    physics: Physics,
}

impl Body for Shape {
    fn physics(&self) -> &Physics {
        &self.physics
    }

    fn physics_mut(&mut self) -> &mut Physics {
        &mut self.physics
    }

    fn mesh(&self) -> &Mesh {
        &self.physics.mesh
    }

    fn mesh_cluster(&self) -> &Option<Vec<Mesh>> {
        &self.physics.mesh_cluster
    }
}

impl Shape {
    pub fn new(mesh: Mesh, mesh_cluster: Option<Vec<Mesh>>) -> Shape {
        let physics: Physics = Physics::new(mesh, mesh_cluster);
        Shape { physics }
    }
}
