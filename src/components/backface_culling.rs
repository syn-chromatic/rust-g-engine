use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;

pub struct BackfaceCulling;

impl BackfaceCulling {
    pub fn get_normal(&self, polygon: &Polygon) -> Vector3D {
        match polygon {
            Polygon::Triangle(triangle) => {
                let v0 = triangle.vertices[0];
                let v1 = triangle.vertices[1];
                let v2 = triangle.vertices[2];

                let edge1 = v1.subtract_vector(&v0);
                let edge2 = v2.subtract_vector(&v0);

                edge1.cross_product(&edge2).normalize()
            }
            Polygon::Quad(quad) => {
                let v0 = quad.vertices[0];
                let v1 = quad.vertices[1];
                let v2 = quad.vertices[2];

                let edge1 = v1.subtract_vector(&v0);
                let edge2 = v2.subtract_vector(&v0);

                edge1.cross_product(&edge2).normalize()
            }
        }
    }

    pub fn get_centroid(&self, polygon: &Polygon) -> Vector3D {
        let vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };

        let mut vertices_sum: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let num_vertices: usize = vertices.len();

        for vertex in vertices {
            vertices_sum = vertices_sum.add_vector(vertex);
        }

        vertices_sum.divide(num_vertices as f64)
    }

    pub fn cull_backfaces(&self, camera_position: &Vector3D, polygons: &[Polygon]) -> Vec<Polygon> {
        let mut culled_polygons = Vec::new();

        for polygon in polygons {
            let normal = self.get_normal(polygon);
            let centroid = polygon.get_centroid();
            let view_vector = centroid.subtract_vector(camera_position);

            let dot_product = normal.dot_product(&view_vector);

            if dot_product < 0.0 {
                culled_polygons.push(polygon.clone());
            }
        }

        culled_polygons
    }
}
