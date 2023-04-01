use crate::components::color::RGBA;
use crate::components::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct Triangle {
    pub vertices: [Vector3D; 3],
    pub face: (usize, usize, usize),
    pub shader: RGBA,
    pub color: RGBA,
}

impl Triangle {
    pub fn new(
        vertices: [Vector3D; 3],
        face: (usize, usize, usize),
        shader: RGBA,
        color: RGBA,
    ) -> Self {
        Self {
            vertices,
            face,
            shader,
            color,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Quad {
    pub vertices: [Vector3D; 4],
    pub face: (usize, usize, usize, usize),
    pub shader: RGBA,
    pub color: RGBA,
}

impl Quad {
    pub fn new(
        vertices: [Vector3D; 4],
        face: (usize, usize, usize, usize),
        shader: RGBA,
        color: RGBA,
    ) -> Self {
        Self {
            vertices,
            face,
            shader,
            color,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Polygon {
    Triangle(Triangle),
    Quad(Quad),
}

impl Polygon {
    pub fn get_normal(&self) -> Vector3D {
        match self {
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

    pub fn get_centroid(&self) -> Vector3D {
        let vertices: &[Vector3D] = match self {
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
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub polygons: Vec<Polygon>,
}

impl Mesh {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }
}
