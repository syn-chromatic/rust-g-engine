use crate::components::polygons::Mesh;
use crate::components::vectors::Vector3D;

pub struct BackfaceCulling;

impl BackfaceCulling {
    pub fn new() -> BackfaceCulling {
        BackfaceCulling {}
    }

    pub fn cull_backfaces(&self, mut mesh: Mesh, camera_position: &Vector3D) -> Mesh {
        mesh.polygons.retain(|polygon| {
            let normal: Vector3D = polygon.get_normal();
            let centroid: Vector3D = polygon.get_centroid();
            let view_vector: Vector3D = centroid.subtract_vector(camera_position);

            let dot_product: f64 = normal.dot_product(&view_vector);

            dot_product < 0.0
        });

        mesh
    }
}
