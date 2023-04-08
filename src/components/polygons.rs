use crate::components::color::RGBA;
use crate::components::shaders::Light;
use crate::components::vectors::Vector3D;

#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum Polygon {
    Triangle(Triangle),
    Quad(Quad),
}

impl Polygon {
    pub fn get_bounding_box(&self) -> ([f64; 3], [f64; 3]) {
        let infinity: f64 = f64::INFINITY;
        let neg_infinity: f64 = f64::NEG_INFINITY;
        let mut min: [f64; 3] = Vector3D::default(infinity).to_vec();
        let mut max: [f64; 3] = Vector3D::default(neg_infinity).to_vec();

        let centroid = match self {
            Polygon::Triangle(triangle) => triangle.get_centroid().to_vec(),
            Polygon::Quad(quad) => quad.get_centroid().to_vec(),
        };

        for i in 0..3 {
            if centroid[i] < min[i] {
                min[i] = centroid[i];
            }
            if centroid[i] > max[i] {
                max[i] = centroid[i];
            }
        }

        let min: [f64; 3] = [min[0], min[1], min[2]];
        let max: [f64; 3] = [max[0], max[1], max[2]];
        (min, max)
    }

    pub fn get_color(&self) -> RGBA {
        match self {
            Polygon::Triangle(triangle) => triangle.color,
            Polygon::Quad(quad) => quad.color,
        }
    }

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

#[derive(Clone, Debug)]
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

    pub fn get_distance_bounding_boxes(
        &self,
        a: &([f64; 3], [f64; 3]),
        b: &([f64; 3], [f64; 3]),
    ) -> f64 {
        let (min_a, max_a) = a;
        let (min_b, max_b) = b;

        let mut intersection: bool = true;
        let mut distance: f64 = 0.0;

        for i in 0..3 {
            if max_a[i] < min_b[i] {
                intersection = false;
                distance += (min_b[i] - max_a[i]).powi(2);
            } else if max_b[i] < min_a[i] {
                intersection = false;
                distance += (min_a[i] - max_b[i]).powi(2);
            }
        }

        if intersection {
            let mut min_overlap = f64::INFINITY;
            for i in 0..3 {
                let overlap = (max_a[i].min(max_b[i])) - (min_a[i].max(min_b[i]));
                min_overlap = min_overlap.min(overlap);
            }
            return -min_overlap;
        }

        distance.sqrt()
    }

    pub fn get_intersect_distance_bounding_boxes(
        &self,
        a: &([f64; 3], [f64; 3]),
        b: &([f64; 3], [f64; 3]),
    ) -> Option<f64> {
        let (min_a, max_a) = a;
        let (min_b, max_b) = b;

        let mut intersection: bool = true;
        let mut min_overlap: f64 = f64::INFINITY;

        for i in 0..3 {
            if max_a[i] < min_b[i] || max_b[i] < min_a[i] {
                intersection = false;
                break;
            } else {
                let overlap = (max_a[i].min(max_b[i])) - (min_a[i].max(min_b[i]));
                min_overlap = min_overlap.min(overlap);
            }
        }

        if intersection {
            Some(min_overlap)
        } else {
            None
        }
    }

    pub fn get_bounding_box(&self) -> ([f64; 3], [f64; 3]) {
        let infinity: f64 = f64::INFINITY;
        let neg_infinity: f64 = f64::NEG_INFINITY;
        let mut min: [f64; 3] = Vector3D::default(infinity).to_vec();
        let mut max: [f64; 3] = Vector3D::default(neg_infinity).to_vec();

        for poly in &self.polygons {
            let centroid = poly.get_centroid().to_vec();
            for i in 0..3 {
                if centroid[i] < min[i] {
                    min[i] = centroid[i];
                }
                if centroid[i] > max[i] {
                    max[i] = centroid[i];
                }
            }
        }
        let min: [f64; 3] = [min[0], min[1], min[2]];
        let max: [f64; 3] = [max[0], max[1], max[2]];
        (min, max)
    }

    pub fn get_distance(&self, other: &Mesh) -> f64 {
        let self_bounding_box: ([f64; 3], [f64; 3]) = self.get_bounding_box();
        let other_bounding_box: ([f64; 3], [f64; 3]) = other.get_bounding_box();

        self.get_distance_bounding_boxes(&self_bounding_box, &other_bounding_box)
    }

    pub fn get_intersect_distance(&self, other: &Mesh) -> Option<f64> {
        let self_bounding_box: ([f64; 3], [f64; 3]) = self.get_bounding_box();
        let other_bounding_box: ([f64; 3], [f64; 3]) = other.get_bounding_box();

        self.get_intersect_distance_bounding_boxes(&self_bounding_box, &other_bounding_box)
    }
}