use crate::color::RGBA;
use crate::vectors::Vector3D;

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

#[derive(Clone, Debug)]
pub struct Mesh {
    pub polygons: Vec<Polygon>,
}

impl Mesh {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }
}
