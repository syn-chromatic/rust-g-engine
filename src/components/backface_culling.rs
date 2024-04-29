use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;

pub struct BackfaceCulling;

impl BackfaceCulling {
    pub fn new() -> BackfaceCulling {
        BackfaceCulling {}
    }

    pub fn cull_backfaces(&self, polygons: &mut Vec<Polygon>, camera_position: &Vector3D) {
        polygons.retain(|polygon| {
            let normal: Vector3D = polygon.get_normal();
            let centroid: Vector3D = polygon.get_centroid();
            let view_vector: Vector3D = centroid.subtract_vector(camera_position);

            let dot_product: f64 = normal.dot_product(&view_vector);

            dot_product < 0.0
        });
    }
}
