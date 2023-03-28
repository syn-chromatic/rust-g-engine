use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

use crate::color::RGBA;
use crate::polygons::Mesh;
use crate::polygons::Polygon;
use crate::polygons::Quad;
use crate::polygons::Triangle;
use crate::vectors::Vector3D;
use crate::vertices::MeshConverter;

pub struct OBJModelFormat {
    file_path: String,
    scale: f64,
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
}

impl OBJModelFormat {
    pub fn new(file_path: &str, scale: f64) -> Self {
        let file_path: String = file_path.to_string();
        Self {
            file_path,
            scale,
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

    pub fn get_model_triangles(&self) -> Mesh {
        self.get_model_polygons(3)
    }

    pub fn get_model_quads(&self) -> Mesh {
        self.get_model_polygons(4)
    }

    pub fn get_polygons(&self) -> Mesh {
        let mut mesh1 = self.get_model_triangles();
        let mut mesh2 = self.get_model_quads();
        mesh2 = MeshConverter::new(mesh2).quads_to_triangles();

        mesh1.polygons.extend(mesh2.polygons);
        mesh1
    }

    fn get_model_polygons(&self, num_vertices: usize) -> Mesh {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let file = File::open(&self.file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let tokens: Vec<&str> = line.split_whitespace().collect();

            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "v" => {
                    let vertex_tuple: (f64, f64, f64) = (
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        tokens[3].parse().unwrap(),
                    );
                    let mut vertex = Vector3D::new(vertex_tuple.0, vertex_tuple.1, vertex_tuple.2);
                    vertex = vertex.multiply(self.scale);

                    let xv = vertex.x + self.x_offset;
                    let yv = vertex.y + self.y_offset;
                    let zv = vertex.z + self.z_offset;
                    vertex = Vector3D::new(xv, yv, zv);

                    vertices.push(vertex);
                }
                "f" => {
                    let face_indices: Vec<usize> = tokens[1..]
                        .iter()
                        .map(|tok| tok.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                        .collect();

                    if face_indices.len() == num_vertices {
                        faces.push(face_indices);
                    }
                }
                _ => (),
            }
        }

        let polygons = faces
            .into_iter()
            .map(|face| match num_vertices {
                3 => Polygon::Triangle(Triangle::new(
                    [vertices[face[0]], vertices[face[1]], vertices[face[2]]],
                    (face[0], face[1], face[2]),
                    RGBA::from_rgb(1.0, 1.0, 1.0),
                    RGBA::from_rgb(1.0, 1.0, 1.0),
                )),
                4 => Polygon::Quad(Quad::new(
                    [
                        vertices[face[0]],
                        vertices[face[1]],
                        vertices[face[2]],
                        vertices[face[3]],
                    ],
                    (face[0], face[1], face[2], face[3]),
                    RGBA::from_rgb(1.0, 1.0, 1.0),
                    RGBA::from_rgb(1.0, 1.0, 1.0),
                )),
                _ => unreachable!(),
            })
            .collect();
        let mesh = Mesh::new(polygons);
        mesh
    }
}
