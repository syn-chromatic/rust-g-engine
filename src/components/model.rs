use std::fs::read;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

use crate::components::color::RGBA;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::polygons::Quad;
use crate::components::polygons::Triangle;
use crate::components::vectors::Vector3D;
use crate::components::vertices::MeshConverter;

pub struct OBJModelFormat {
    file_bytes: Vec<u8>,
    scale: f64,
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
    x_rotation: f64,
    y_rotation: f64,
    z_rotation: f64,
}

impl OBJModelFormat {
    pub fn new(file_path: &str, scale: f64) -> Self {
        let file_bytes = read(file_path);
        if file_bytes.is_err() {
            println!("Filepath not found: {:?}", file_path);
            std::process::exit(1);
        }
        let file_bytes = file_bytes.unwrap();

        Self {
            file_bytes,
            scale,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
        }
    }

    pub fn set_offset(&mut self, x: f64, y: f64, z: f64) {
        self.x_offset = x;
        self.y_offset = y;
        self.z_offset = z;
    }

    pub fn set_rotation(&mut self, x: f64, y: f64, z: f64) {
        self.x_rotation = x;
        self.y_rotation = y;
        self.z_rotation = z;
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

    fn rotate_x(&self, vertex: &Vector3D) -> Vector3D {
        let cos_theta = self.x_rotation.to_radians().cos();
        let sin_theta = self.x_rotation.to_radians().sin();
        let x = vertex.x;
        let y = vertex.y * cos_theta - vertex.z * sin_theta;
        let z = vertex.y * sin_theta + vertex.z * cos_theta;

        Vector3D::new(x, y, z)
    }

    fn rotate_y(&self, vertex: &Vector3D) -> Vector3D {
        let cos_theta = self.y_rotation.to_radians().cos();
        let sin_theta = self.y_rotation.to_radians().sin();
        let x = vertex.x * cos_theta + vertex.z * sin_theta;
        let y = vertex.y;
        let z = -vertex.x * sin_theta + vertex.z * cos_theta;

        Vector3D::new(x, y, z)
    }

    fn rotate_z(&self, vertex: &Vector3D) -> Vector3D {
        let cos_theta = self.z_rotation.to_radians().cos();
        let sin_theta = self.z_rotation.to_radians().sin();
        let x = vertex.x * cos_theta - vertex.y * sin_theta;
        let y = vertex.x * sin_theta + vertex.y * cos_theta;
        let z = vertex.z;

        Vector3D::new(x, y, z)
    }

    fn apply_rotation(&self, vertex: Vector3D) -> Vector3D {
        let vertex = self.rotate_x(&vertex);
        let vertex = self.rotate_y(&vertex);
        let vertex = self.rotate_z(&vertex);
        vertex
    }

    fn get_vertex(&self, tokens: Vec<&str>) -> Vector3D {
        let vertex_tuple: (f64, f64, f64) = (
            tokens[1].parse().unwrap(),
            tokens[2].parse().unwrap(),
            tokens[3].parse().unwrap(),
        );
        let mut vertex = Vector3D::new(vertex_tuple.0, vertex_tuple.1, vertex_tuple.2);
        vertex = vertex.multiply(self.scale);
        vertex = self.apply_rotation(vertex);

        let xv = vertex.x + self.x_offset;
        let yv = vertex.y + self.y_offset;
        let zv = vertex.z + self.z_offset;
        vertex = Vector3D::new(xv, yv, zv);
        vertex
    }

    fn get_face(&self, tokens: Vec<&str>) -> Vec<usize> {
        let face_indices: Vec<usize> = tokens[1..]
            .iter()
            .map(|tok| tok.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
            .collect();
        face_indices
    }

    fn get_model_polygons(&self, num_vertices: usize) -> Mesh {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let cursor = Cursor::new(self.file_bytes.clone());
        let reader = BufReader::new(cursor);

        for line in reader.lines() {
            let line = line.unwrap();
            let tokens: Vec<&str> = line.split_whitespace().collect();

            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "v" => {
                    let vertex = self.get_vertex(tokens);
                    vertices.push(vertex);
                }
                "f" => {
                    let face_indices = self.get_face(tokens);

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
                    RGBA::from_rgb(0.0, 0.0, 0.0),
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
                    RGBA::from_rgb(0.0, 0.0, 0.0),
                    RGBA::from_rgb(1.0, 1.0, 1.0),
                )),
                _ => unreachable!(),
            })
            .collect();
        let mesh = Mesh::new(polygons);
        mesh
    }
}
