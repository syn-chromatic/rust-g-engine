use crate::components::polygons::Mesh;
use crate::components::vectors::Vector3D;

pub struct BackfaceCulling;

impl BackfaceCulling {
    pub fn new() -> BackfaceCulling {
        BackfaceCulling {}
    }

    pub fn cull_backfaces(&self, mut mesh: Mesh, camera_position: &Vector3D) -> Mesh {
        let polygons = mesh.polygons;
        let mut culled_polygons = Vec::new();

        for polygon in polygons {
            let normal = polygon.get_normal();
            let centroid = polygon.get_centroid();
            let view_vector = centroid.subtract_vector(camera_position);

            let dot_product = normal.dot_product(&view_vector);

            if dot_product < 0.0 {
                culled_polygons.push(polygon.clone());
            }
        }

        mesh.polygons = culled_polygons;
        mesh
    }
}
