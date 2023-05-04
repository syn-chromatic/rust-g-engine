use crate::components::bvh::BVHNode;
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

    pub fn plane(&self) -> (Vector3D, f64) {
        let normal = self.get_normal();
        let d = -normal.dot_product(&self.vertices[0]);

        (normal, d)
    }

    pub fn classify_vertex(&self, vertex: &Vector3D) -> f64 {
        let (plane_normal, d) = self.plane();
        plane_normal.dot_product(vertex) + d
    }

    pub fn split(&self, other: Triangle) -> (Option<Triangle>, Option<Triangle>) {
        let mut front_vertices = Vec::new();
        let mut back_vertices = Vec::new();

        for vertex in &other.vertices {
            let distance = self.classify_vertex(vertex);

            if distance <= 0.0 {
                front_vertices.push(*vertex);
            } else {
                back_vertices.push(*vertex);
            }
        }

        let front_triangle = if front_vertices.len() == 3 {
            Some(Triangle::new(
                [front_vertices[0], front_vertices[1], front_vertices[2]],
                other.face,
                other.shader,
                other.color,
            ))
        } else {
            None
        };

        let back_triangle = if back_vertices.len() == 3 {
            Some(Triangle::new(
                [back_vertices[0], back_vertices[1], back_vertices[2]],
                other.face,
                other.shader,
                other.color,
            ))
        } else {
            None
        };

        (front_triangle, back_triangle)
    }

    pub fn rotate_around_point(&mut self, axis: &Vector3D, angle: f64, point: &Vector3D) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex
                .subtract_vector(point)
                .rotate_around_axis(axis, angle)
                .add_vector(point);
        }
    }

    pub fn rotate(&mut self, axis: &Vector3D, angle: f64) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.rotate_around_axis(axis, angle);
        }
    }

    pub fn translate(&mut self, translation: &Vector3D) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.add_vector(translation);
        }
    }

    pub fn get_normal(&self) -> Vector3D {
        let v1: Vector3D = self.vertices[0];
        let v2: Vector3D = self.vertices[1];
        let v3: Vector3D = self.vertices[2];

        let edge1: Vector3D = v2.subtract_vector(&v1);
        let edge2: Vector3D = v3.subtract_vector(&v1);

        edge1.cross_product(&edge2).normalize()
    }

    pub fn get_area(&self) -> f64 {
        let v1: Vector3D = self.vertices[0];
        let v2: Vector3D = self.vertices[1];
        let v3: Vector3D = self.vertices[2];

        let edge1: Vector3D = v2.subtract_vector(&v1);
        let edge2: Vector3D = v3.subtract_vector(&v1);

        edge1.cross_product(&edge2).get_length() / 2.0
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


    pub fn plane(&self) -> (Vector3D, f64) {
        let normal = self.get_normal();
        let d = -normal.dot_product(&self.vertices[0]);

        (normal, d)
    }


    pub fn classify_vertex(&self, vertex: &Vector3D) -> f64 {
        let (plane_normal, d) = self.plane();
        plane_normal.dot_product(vertex) + d
    }

    pub fn split(&self, other: Quad) -> (Option<Quad>, Option<Quad>) {
        let mut front_vertices = Vec::new();
        let mut back_vertices = Vec::new();

        for vertex in &other.vertices {
            let distance = self.classify_vertex(vertex);

            if distance >= 0.0 {
                front_vertices.push(*vertex);
            } else {
                back_vertices.push(*vertex);
            }
        }

        let front_quad = if front_vertices.len() == 4 {
            Some(Quad::new(
                [front_vertices[0], front_vertices[1], front_vertices[2], front_vertices[3]],
                other.face,
                other.shader,
                other.color,
            ))
        } else {
            None
        };

        let back_quad = if back_vertices.len() == 4 {
            Some(Quad::new(
                [back_vertices[0], back_vertices[1], back_vertices[2], back_vertices[3]],
                other.face,
                other.shader,
                other.color,
            ))
        } else {
            None
        };

        (front_quad, back_quad)
    }


    pub fn rotate_around_point(&mut self, axis: &Vector3D, angle: f64, point: &Vector3D) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex
                .subtract_vector(point)
                .rotate_around_axis(axis, angle)
                .add_vector(point);
        }
    }

    pub fn rotate(&mut self, axis: &Vector3D, angle: f64) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.rotate_around_axis(axis, angle);
        }
    }

    pub fn translate(&mut self, translation: &Vector3D) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.add_vector(translation);
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

    pub fn get_area(&self) -> f64 {
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];
        let v4 = self.vertices[3];

        let edge1 = v2.subtract_vector(&v1);
        let edge2 = v3.subtract_vector(&v1);

        let area1 = edge1.cross_product(&edge2).get_length() / 2.0;

        let edge3 = v3.subtract_vector(&v1);
        let edge4 = v4.subtract_vector(&v1);

        let area2 = edge3.cross_product(&edge4).get_length() / 2.0;

        area1 + area2
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
    pub fn get_vertices(&self) -> &[Vector3D] {
        let vertices: &[Vector3D] = match self {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };
        vertices
    }

    pub fn get_area(&self) -> f64 {
        match self {
            Polygon::Triangle(triangle) => triangle.get_area(),
            Polygon::Quad(quad) => quad.get_area(),
        }
    }

    pub fn plane(&self) -> (Vector3D, f64) {
        match self {
            Polygon::Triangle(triangle) => triangle.plane(),
            Polygon::Quad(quad) => quad.plane(),
        }
    }

    pub fn split(&self, other: Polygon) -> (Option<Triangle>, Option<Triangle>) {
        match self {
            Polygon::Triangle(triangle) => {
                match other {
                    Polygon::Triangle(other_triangle) => triangle.split(other_triangle),
                    Polygon::Quad(other_Quad) => (None, None),
                }
            },
            Polygon::Quad(quad) => {
                match other {
                    Polygon::Quad(other_quad) => (None, None),
                    Polygon::Triangle(other_triangle) => (None, None),
                }
            }
        }

        // match self {
        //     Polygon::Triangle(triangle) => triangle.split(),
        //     Polygon::Quad(quad) => quad.split(),
        // }
    }

    pub fn set_color(&mut self, color: &RGBA) {
        match self {
            Polygon::Triangle(triangle) => triangle.color = color.clone(),
            Polygon::Quad(quad) => quad.color = color.clone(),
        }
    }

    pub fn rotate_around_point(&mut self, axis: &Vector3D, angle: f64, point: &Vector3D) {
        match self {
            Polygon::Triangle(triangle) => triangle.rotate_around_point(axis, angle, point),
            Polygon::Quad(quad) => quad.rotate_around_point(axis, angle, point),
        }
    }

    pub fn rotate(&mut self, axis: &Vector3D, angle: f64) {
        match self {
            Polygon::Triangle(triangle) => triangle.rotate(axis, angle),
            Polygon::Quad(quad) => quad.rotate(axis, angle),
        }
    }

    pub fn translate(&mut self, translation: &Vector3D) {
        match self {
            Polygon::Triangle(triangle) => triangle.translate(translation),
            Polygon::Quad(quad) => quad.translate(translation),
        }
    }

    pub fn get_bounding_box(&self) -> ([f64; 3], [f64; 3]) {
        let infinity: f64 = f64::INFINITY;
        let neg_infinity: f64 = f64::NEG_INFINITY;
        let mut min: [f64; 3] = Vector3D::default(infinity).to_array();
        let mut max: [f64; 3] = Vector3D::default(neg_infinity).to_array();

        let centroid = match self {
            Polygon::Triangle(triangle) => triangle.get_centroid().to_array(),
            Polygon::Quad(quad) => quad.get_centroid().to_array(),
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

fn get_vertices_from_polygons(polygons: &[Polygon]) -> Vec<Vector3D> {
    let mut vertices: Vec<Vector3D> = Vec::new();

    for polygon in polygons {
        let polygon_vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };

        vertices.extend_from_slice(polygon_vertices);
    }

    vertices
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub polygons: Vec<Polygon>,
    pub bvh_node: BVHNode,
    pub light: Option<Light>,
}

impl Mesh {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        let vertices: Vec<Vector3D> = get_vertices_from_polygons(&polygons);
        let bvh_node: BVHNode = BVHNode::new(&polygons, &vertices);
        Self {
            polygons,
            bvh_node,
            light: None,
        }
    }

    // pub fn get_total_volume(&self) -> f64 {
    //     let mut total_volume: f64 = 0.0;
    //     let center_of_mass: Vector3D = self.get_center_of_mass();

    //     for polygon in &self.polygons {
    //         let vertices: &[Vector3D] = polygon.get_vertices();

    //         if vertices.len() > 3 {
    //             continue;
    //         }

    //         for i in 1..vertices.len() - 1 {
    //             let a: Vector3D = vertices[0].subtract_vector(&center_of_mass);
    //             let b: Vector3D = vertices[i].subtract_vector(&center_of_mass);
    //             let c: Vector3D = vertices[i + 1].subtract_vector(&center_of_mass);

    //             let volume: f64 = (a.dot_product(&b.cross_product(&c))).abs() / 6.0;
    //             total_volume += volume;
    //         }
    //     }

    //     total_volume
    // }

    pub fn get_total_volume(&self) -> f64 {
        let mut total_volume: f64 = 0.0;

        for polygon in &self.polygons {
            let vertices: &[Vector3D] = polygon.get_vertices();

            if vertices.len() != 3 {
                continue;
            }

            let a: Vector3D = vertices[0];
            let b: Vector3D = vertices[1];
            let c: Vector3D = vertices[2];

            let volume: f64 = (a.dot_product(&(b.cross_product(&c)))) / 6.0;
            total_volume += volume;
        }

        total_volume
    }

    // pub fn get_center_of_mass(&self) -> Vector3D {
    //     let mut centroid_sum = Vector3D::new(0.0, 0.0, 0.0);
    //     let num_polygons = self.polygons.len();

    //     for polygon in &self.polygons {
    //         centroid_sum = centroid_sum.add_vector(&polygon.get_centroid());
    //     }

    //     centroid_sum.divide(num_polygons as f64)
    // }

    pub fn get_center_of_mass(&self) -> Vector3D {
        let mut weighted_centroid_sum = Vector3D::new(0.0, 0.0, 0.0);
        let mut total_volume: f64 = 0.0;

        for polygon in &self.polygons {
            let vertices: &[Vector3D] = polygon.get_vertices();

            if vertices.len() != 3 {
                continue;
            }

            let a: Vector3D = vertices[0];
            let b: Vector3D = vertices[1];
            let c: Vector3D = vertices[2];

            let volume: f64 = (a.dot_product(&(b.cross_product(&c)))) / 6.0;
            let centroid: Vector3D = a.add_vector(&b).add_vector(&c).divide(4.0);

            weighted_centroid_sum = weighted_centroid_sum.add_vector(&centroid.multiply(volume));
            total_volume += volume;
        }

        weighted_centroid_sum.divide(total_volume)
    }

    pub fn set_uniform_color(&mut self, color: RGBA) {
        for polygon in self.polygons.iter_mut() {
            polygon.set_color(&color);
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.light = Some(light);
    }

    pub fn translate_polygons(&mut self, translation: &Vector3D) {
        for polygon in &mut self.polygons {
            polygon.translate(translation);
        }
        self.bvh_node.translate_bvh(translation);
    }

    pub fn get_mesh_centroid(&self) -> Vector3D {
        let mut centroid_sum = Vector3D::new(0.0, 0.0, 0.0);
        let num_polygons = self.polygons.len();
        for polygon in &self.polygons {
            centroid_sum = centroid_sum.add_vector(&polygon.get_centroid());
        }
        let centroid: Vector3D = centroid_sum.divide(num_polygons as f64);
        centroid
    }

    pub fn rotate_polygons_around_axis(
        &mut self,
        axis: &Vector3D,
        centroid: &Vector3D,
        angle: f64,
    ) {
        let translation_to_origin = centroid.multiply(-1.0);
        self.translate_polygons(&translation_to_origin);

        for polygon in &mut self.polygons {
            polygon.rotate(&axis, angle);
        }

        self.translate_polygons(&centroid);

        self.bvh_node.rotate_bvh(axis, centroid, angle);
    }

    // pub fn rotate_polygons_around_axis(&mut self, axis: &Vector3D, angle: f64) {

    //     let mut centroid_sum = Vector3D::new(0.0, 0.0, 0.0);
    //     let num_polygons = self.polygons.len();
    //     for polygon in &self.polygons {
    //         centroid_sum = centroid_sum.add_vector(&polygon.get_centroid());
    //     }
    //     let centroid = centroid_sum.divide(num_polygons as f64);

    //     for polygon in &mut self.polygons {
    //         polygon.rotate_around_point(&axis, angle, &centroid);
    //     }

    //     let vertices = get_vertices_from_polygons(&self.polygons);
    //     self.bvh_node = BVHNode::new(&self.polygons, &vertices);
    // }

    pub fn get_distance_bvh(&self, other: &Mesh) -> f64 {
        self.bvh_node.get_distance(&other.bvh_node)
    }

    pub fn is_intersecting_bvh(&self, other: &Mesh) -> Option<(Vector3D, Vector3D)> {
        self.bvh_node.is_intersecting(&other.bvh_node)
    }

    pub fn get_distance(&self, other: &Mesh) -> f64 {
        let self_bounding_box: ([f64; 3], [f64; 3]) = self.get_bounding_box();
        let other_bounding_box: ([f64; 3], [f64; 3]) = other.get_bounding_box();

        self.get_distance_bounding_boxes(&self_bounding_box, &other_bounding_box)
    }

    pub fn get_distance_bounding_boxes(
        &self,
        a: &([f64; 3], [f64; 3]),
        b: &([f64; 3], [f64; 3]),
    ) -> f64 {
        let (min_a, max_a): &([f64; 3], [f64; 3]) = a;
        let (min_b, max_b): &([f64; 3], [f64; 3]) = b;

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
        let (min_a, max_a): &([f64; 3], [f64; 3]) = a;
        let (min_b, max_b): &([f64; 3], [f64; 3]) = b;

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
        let mut min: [f64; 3] = Vector3D::default(infinity).to_array();
        let mut max: [f64; 3] = Vector3D::default(neg_infinity).to_array();

        for poly in &self.polygons {
            let centroid: [f64; 3] = poly.get_centroid().to_array();
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

    pub fn get_intersect_distance(&self, other: &Mesh) -> Option<f64> {
        let self_bounding_box: ([f64; 3], [f64; 3]) = self.get_bounding_box();
        let other_bounding_box: ([f64; 3], [f64; 3]) = other.get_bounding_box();

        self.get_intersect_distance_bounding_boxes(&self_bounding_box, &other_bounding_box)
    }
}
