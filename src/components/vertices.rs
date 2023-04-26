use crate::components::color::RGBA;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::polygons::Quad;
use crate::components::polygons::Triangle;
use crate::components::vectors::Vector3D;
use std::f64::consts::PI;

pub struct Sphere {
    radius: f64,
    num_latitude: usize,
    num_longitude: usize,
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
    color: RGBA,
    shader: RGBA,
}

impl Sphere {
    pub fn new(radius: f64, num_latitude: usize, num_longitude: usize) -> Sphere {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let z_offset = 0.0;
        let color = RGBA::from_rgb(1.0, 1.0, 1.0);
        let shader = RGBA::from_rgb(0.0, 0.0, 0.0);

        Sphere {
            radius,
            num_latitude,
            num_longitude,
            x_offset,
            y_offset,
            z_offset,
            color,
            shader,
        }
    }

    pub fn set_offset(&mut self, x: f64, y: f64, z: f64) {
        self.x_offset = x;
        self.y_offset = y;
        self.z_offset = z;
    }

    pub fn set_color(&mut self, color: RGBA) {
        self.color = color;
    }

    pub fn set_shader(&mut self, shader: RGBA) {
        self.shader = shader;
    }

    fn get_vertices(&self) -> Vec<Vector3D> {
        let mut vertices: Vec<Vector3D> = vec![];

        for i in 0..(self.num_latitude + 1) {
            let theta = i as f64 * PI / self.num_latitude as f64;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for j in 0..(self.num_longitude + 1) {
                let phi = j as f64 * 2.0 * PI / self.num_longitude as f64;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = (self.radius * sin_theta * cos_phi) + self.x_offset;
                let y = (self.radius * sin_theta * sin_phi) + self.y_offset;
                let z = (self.radius * cos_theta) + self.z_offset;

                let vertex = Vector3D::new(x, y, z);
                vertices.push(vertex);
            }
        }
        vertices
    }

    fn get_triangle_faces(&self) -> Vec<(usize, usize, usize)> {
        let mut faces: Vec<(usize, usize, usize)> = vec![];

        for i in 0..self.num_latitude {
            for j in 0..self.num_longitude {
                let first: usize = i * (self.num_longitude + 1) + j;
                let second: usize = first + self.num_longitude + 1;

                let face1: (usize, usize, usize) = (first, second, first + 1);
                let face2: (usize, usize, usize) = (second, second + 1, first + 1);
                faces.extend([face1, face2]);
            }
        }
        faces
    }

    fn get_quad_faces(&self) -> Vec<(usize, usize, usize, usize)> {
        let mut faces: Vec<(usize, usize, usize, usize)> = vec![];

        for i in 0..self.num_latitude {
            for j in 0..self.num_longitude {
                let first: usize = i * (self.num_longitude + 1) + j;
                let second: usize = first + self.num_longitude + 1;

                let face: (usize, usize, usize, usize) = (first, second, second + 1, first + 1);
                faces.push(face);
            }
        }
        faces
    }

    pub fn get_triangle_mesh(&self) -> Mesh {
        let vertices: Vec<Vector3D> = self.get_vertices();
        let faces: Vec<(usize, usize, usize)> = self.get_triangle_faces();
        let mut triangle_polygons: Vec<Polygon> = vec![];
        let mut vertices_count: usize = 0;
        let mut faces_count: usize = faces.len();

        for face in faces {
            let triangle_vertices: [Vector3D; 3] =
                [vertices[face.0], vertices[face.1], vertices[face.2]];
            let triangle: Triangle =
                Triangle::new(triangle_vertices, face, self.shader, self.color);
            let polygon: Polygon = Polygon::Triangle(triangle);
            triangle_polygons.push(polygon);
            vertices_count += 3;
        }
        println!(
            "{} {:?}, {}{:?}",
            "Sphere Vertices:", vertices_count, "Faces:", faces_count
        );
        let mesh = Mesh::new(triangle_polygons);
        mesh
    }

    pub fn get_quad_mesh(&self) -> Mesh {
        let vertices: Vec<Vector3D> = self.get_vertices();
        let faces: Vec<(usize, usize, usize, usize)> = self.get_quad_faces();
        let mut quad_polygons: Vec<Polygon> = vec![];

        for face in faces {
            let quad_vertices: [Vector3D; 4] = [
                vertices[face.0],
                vertices[face.1],
                vertices[face.2],
                vertices[face.3],
            ];
            let triangle: Quad = Quad::new(quad_vertices, face, self.shader, self.color);
            let polygon: Polygon = Polygon::Quad(triangle);
            quad_polygons.push(polygon);
        }
        let mesh = Mesh::new(quad_polygons);
        mesh
    }
}

pub struct Cuboid {
    width: f64,
    height: f64,
    depth: f64,
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
    color: RGBA,
    shader: RGBA,
}

impl Cuboid {
    pub fn new(width: f64, height: f64, depth: f64) -> Cuboid {
        let x_offset: f64 = 0.0;
        let y_offset: f64 = 0.0;
        let z_offset: f64 = 0.0;
        let color: RGBA = RGBA::from_rgb(1.0, 1.0, 1.0);
        let shader: RGBA = RGBA::from_rgb(0.0, 0.0, 0.0);

        Cuboid {
            width,
            height,
            depth,
            x_offset,
            y_offset,
            z_offset,
            color,
            shader,
        }
    }

    pub fn set_offset(&mut self, x: f64, y: f64, z: f64) {
        self.x_offset = x;
        self.y_offset = y;
        self.z_offset = z;
    }

    pub fn set_color(&mut self, color: RGBA) {
        self.color = color;
    }

    pub fn set_shader(&mut self, shader: RGBA) {
        self.shader = shader;
    }

    fn get_vertices(&self) -> Vec<Vector3D> {
        let mut vertices: Vec<Vector3D> = vec![];

        for x_factor in &[0.0, 1.0] {
            for y_factor in &[0.0, 1.0] {
                for z_factor in &[0.0, 1.0] {
                    let x: f64 = self.width * x_factor + self.x_offset;
                    let y: f64 = self.height * y_factor + self.y_offset;
                    let z: f64 = self.depth * z_factor + self.z_offset;

                    let vertex = Vector3D::new(x, y, z);
                    vertices.push(vertex);
                }
            }
        }

        vertices
    }

    fn get_quad_faces(&self) -> Vec<[usize; 4]> {
        vec![
            [0, 1, 3, 2],
            [4, 6, 7, 5],
            [0, 4, 5, 1],
            [2, 3, 7, 6],
            [0, 2, 6, 4],
            [1, 5, 7, 3],
        ]
    }

    pub fn get_triangle_mesh(&self) -> Mesh {
        let vertices: Vec<Vector3D> = self.get_vertices();
        let quad_faces: Vec<[usize; 4]> = self.get_quad_faces();
        let mut triangle_polygons: Vec<Polygon> = vec![];
        let mut vertices_count: usize = 0;
        let mut faces_count: usize = 0;

        for face in quad_faces {
            let quad_vertices: [Vector3D; 4] = [
                vertices[face[0]],
                vertices[face[1]],
                vertices[face[2]],
                vertices[face[3]],
            ];
            let triangle1: Triangle = Triangle::new(
                [quad_vertices[0], quad_vertices[1], quad_vertices[2]],
                (face[0], face[1], face[2]),
                self.shader,
                self.color,
            );
            let triangle2: Triangle = Triangle::new(
                [quad_vertices[0], quad_vertices[2], quad_vertices[3]],
                (face[0], face[2], face[3]),
                self.shader,
                self.color,
            );
            let polygon1: Polygon = Polygon::Triangle(triangle1);
            let polygon2: Polygon = Polygon::Triangle(triangle2);
            triangle_polygons.extend([polygon1, polygon2]);
            vertices_count += 4;
            faces_count += 2;
        }
        println!(
            "{} {:?}, {}{:?}",
            "Cuboid Vertices:", vertices_count, "Faces:", faces_count
        );
        let mesh = Mesh::new(triangle_polygons);
        mesh
    }
}

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
                RGBA::from_rgb(0.0, 0.0, 0.0),
                RGBA::from_rgb(1.0, 1.0, 1.0),
            );
            triangle_polygons.push(Polygon::Triangle(triangle));
        }
        Mesh::new(triangle_polygons)
    }

    pub fn get_quad_polygons(&self) -> Mesh {
        let vertices = self.get_vertices();
        let faces = self.get_quad_faces();
        let mut quad_polygons = Vec::new();

        for face in faces {
            let quad = Quad::new(
                [
                    vertices[face.0],
                    vertices[face.1],
                    vertices[face.2],
                    vertices[face.3],
                ],
                face,
                RGBA::from_rgb(0.0, 0.0, 0.0),
                RGBA::from_rgb(1.0, 1.0, 1.0),
            );
            quad_polygons.push(Polygon::Quad(quad));
        }
        Mesh::new(quad_polygons)
    }
}
