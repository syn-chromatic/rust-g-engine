use crate::components::color::RGBA;
use crate::components::shaders::Light;
use crate::components::vectors::Vector3D;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn get_normal(&self) -> Vector3D {
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];

        let edge1 = v2.subtract_vector(&v1);
        let edge2 = v3.subtract_vector(&v1);

        edge1.cross_product(&edge2).normalize()
    }

    pub fn get_centroid(&self) -> Vector3D {
        let vertices: &[Vector3D] = &self.vertices;

        let mut vertices_sum: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let num_vertices: usize = vertices.len();

        for vertex in vertices {
            vertices_sum = vertices_sum.add_vector(vertex);
        }

        vertices_sum.divide(num_vertices as f64)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn get_normal(&self) -> Vector3D {
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];

        let edge1 = v2.subtract_vector(&v1);
        let edge2 = v3.subtract_vector(&v1);

        edge1.cross_product(&edge2).normalize()
    }

    pub fn get_centroid(&self) -> Vector3D {
        let vertices: &[Vector3D] = &self.vertices;

        let mut vertices_sum: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let num_vertices: usize = vertices.len();

        for vertex in vertices {
            vertices_sum = vertices_sum.add_vector(vertex);
        }

        vertices_sum.divide(num_vertices as f64)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Polygon {
    Triangle(Triangle),
    Quad(Quad),
}

impl Polygon {
    pub fn set_shader(&mut self, shader: RGBA) {
        match self {
            Polygon::Triangle(triangle) => {
                let average_shader = triangle.shader.average(&shader);
                triangle.shader = average_shader;
            }
            Polygon::Quad(quad) => {
                let average_shader = quad.shader.average(&shader);
                quad.shader = average_shader;
            }
        }
    }

    pub fn get_normal(&self) -> Vector3D {
        match self {
            Polygon::Triangle(triangle) => triangle.get_normal(),
            Polygon::Quad(quad) => quad.get_normal(),
        }
    }

    pub fn get_centroid(&self) -> Vector3D {
        match self {
            Polygon::Triangle(triangle) => triangle.get_centroid(),
            Polygon::Quad(quad) => quad.get_centroid(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub polygons: Vec<Polygon>,
    pub original_polygons: Vec<Polygon>,
    pub light: Option<Light>,
}

impl Mesh {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        let original_polygons = polygons.clone();
        Self {
            polygons,
            original_polygons,
            light: None,
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.light = Some(light);
    }
}
