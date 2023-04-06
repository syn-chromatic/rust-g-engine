use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use std::cmp::Ordering::Equal;

pub struct ZBufferSort {}

impl ZBufferSort {
    pub fn new() -> Self {
        Self {}
    }

    fn get_centroid_distance(&self, polygon: &Polygon, camera_position: &Vector3D) -> f64 {
        let centroid: Vector3D = polygon.get_centroid();
        let distance: f64 = camera_position.get_distance(&centroid);
        distance
    }

    pub fn get_sorted_polygons(&self, mut mesh: Mesh, camera_position: Vector3D) -> Mesh {
        let polygons = &mut mesh.polygons;

        polygons.sort_unstable_by(|a, b| {
            let dist_a = self.get_centroid_distance(a, &camera_position);
            let dist_b = self.get_centroid_distance(b, &camera_position);

            dist_b.partial_cmp(&dist_a).unwrap_or(Equal)
        });
        mesh
    }
}
