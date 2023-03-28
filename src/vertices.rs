use rand::thread_rng;
use rand::Rng;

use crate::color::RGBA;
use crate::polygons::Mesh;
use crate::polygons::Polygon;
use crate::polygons::Quad;
use crate::polygons::Triangle;
use crate::vectors::Vector3D;
use std::f64::consts::PI;

// pub struct Sphere {
//     radius: f64,
//     num_latitude: u32,
//     num_longitude: u32,
//     x_offset: f64,
//     y_offset: f64,
//     z_offset: f64,
// }

// impl Sphere {
//     pub fn new(radius: f64, num_latitude: u32, num_longitude: u32) -> Sphere {
//         let x_offset = 0.0;
//         let y_offset = 0.0;
//         let z_offset = 0.0;

//         Sphere {
//             radius,
//             num_latitude,
//             num_longitude,
//             x_offset,
//             y_offset,
//             z_offset,
//         }
//     }

//     pub fn set_offset(&mut self, x: f64, y: f64, z: f64) {
//         self.x_offset = x;
//         self.y_offset = y;
//         self.z_offset = z;
//     }

//     pub fn get_vertices(&self) {

//     }
// }

pub struct MeshConverter {
    mesh: Mesh,
}

impl MeshConverter {
    pub fn new(mesh: Mesh) -> Self {
        MeshConverter { mesh }
    }

    pub fn quads_to_triangles(&self) -> Mesh {
        let mut new_polygons: Vec<Polygon> = vec![];

        for polygon in &self.mesh.polygons {
            match polygon {
                Polygon::Triangle(triangle) => {
                    new_polygons.push(Polygon::Triangle(triangle.clone()));
                }
                Polygon::Quad(quad) => {
                    let vertices = quad.vertices;
                    let face = quad.face;
                    let shader = &quad.shader;
                    let color = &quad.color;

                    let triangle1_vertices = [vertices[0], vertices[1], vertices[2]];
                    let triangle1_face = (face.0, face.1, face.2);
                    let triangle1 = Triangle::new(
                        triangle1_vertices,
                        triangle1_face,
                        shader.clone(),
                        color.clone(),
                    );

                    let triangle2_vertices = [vertices[0], vertices[2], vertices[3]];
                    let triangle2_face = (face.0, face.2, face.3);
                    let triangle2 = Triangle::new(
                        triangle2_vertices,
                        triangle2_face,
                        shader.clone(),
                        color.clone(),
                    );

                    new_polygons.push(Polygon::Triangle(triangle1));
                    new_polygons.push(Polygon::Triangle(triangle2));
                }
            }
        }
        Mesh::new(new_polygons)
    }
}

pub struct GridHorizontal {
    rows: usize,
    cols: usize,
    size: f64,
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
}

impl GridHorizontal {
    pub fn new(rows: usize, cols: usize, size: f64) -> Self {
        Self {
            rows,
            cols,
            size,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
        }
    }

    pub fn set_offset(&mut self, x: f64, y: f64, z: f64) {
        self.x_offset = x;
        self.y_offset = y;
        self.z_offset = z;
    }

    pub fn get_vertices(&self) -> Vec<Vector3D> {
        let mut vertices = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let xv = (row as f64 * self.size) + self.x_offset;
                let yv = self.y_offset;
                let zv = (col as f64 * self.size) + self.z_offset;
                let vertex = Vector3D::new(xv, yv, zv);
                vertices.push(vertex);
            }
        }
        vertices
    }

    pub fn get_triangle_faces(&self) -> Vec<(usize, usize, usize)> {
        let mut faces = Vec::new();

        for row in 0..(self.rows - 1) {
            for col in 0..(self.cols - 1) {
                let face1 = (
                    row * self.cols + col,
                    row * self.cols + col + 1,
                    (row + 1) * self.cols + col,
                );

                let face2 = (
                    row * self.cols + col + 1,
                    (row + 1) * self.cols + col + 1,
                    (row + 1) * self.cols + col,
                );

                faces.push(face1);
                faces.push(face2);
            }
        }
        faces
    }

    pub fn get_quad_faces(&self) -> Vec<(usize, usize, usize, usize)> {
        let mut faces = Vec::new();

        for row in 0..(self.rows - 1) {
            for col in 0..(self.cols - 1) {
                let face = (
                    row * self.cols + col,
                    row * self.cols + col + 1,
                    (row + 1) * self.cols + col + 1,
                    (row + 1) * self.cols + col,
                );
                faces.push(face);
            }
        }
        faces
    }

    pub fn get_triangle_polygons(&self) -> Mesh {
        let vertices = self.get_vertices();
        let faces = self.get_triangle_faces();
        let mut triangle_polygons = Vec::new();

        for face in faces {
            let triangle = Triangle::new(
                [vertices[face.0], vertices[face.1], vertices[face.2]],
                face,
                RGBA::from_rgb(1.0, 1.0, 1.0),
                RGBA::from_rgb(1.0, 1.0, 1.0),
            );
            triangle_polygons.push(Polygon::Triangle(triangle));
        }
        Mesh::new(triangle_polygons)
    }
}
