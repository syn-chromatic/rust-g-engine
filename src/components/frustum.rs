use std::f64::consts::PI;

use crate::components::polygons::Polygon;
use crate::components::polygons::Triangle;
use crate::components::vectors::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

pub struct Frustum {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub near_plane: f64,
    pub far_plane: f64,
    pub planes: Vec<Plane>,
}

impl Frustum {
    pub fn new(width: u32, height: u32) -> Self {
        let planes: Vec<Plane> = vec![];
        let mut frustum: Frustum = Frustum {
            width,
            height,
            fov: 100.0,
            near_plane: 0.1,
            far_plane: 100_000_000.0,
            planes,
        };
        let planes: Vec<Plane> = frustum.make_frustum();
        frustum.planes = planes;
        frustum
    }

    fn make_frustum(&self) -> Vec<Plane> {
        let fov: f64 = self.fov;
        let aspect: f64 = self.width as f64 / self.height as f64;
        let near: f64 = -self.near_plane;
        let far: f64 = -self.far_plane;
        let fov_rad: f64 = (fov / 2.0 * PI / 180.0).tan();

        let y_top: f64 = near.abs() * fov_rad;
        let x_right: f64 = y_top * aspect;

        // Near Plane
        let p0_n: Vector3D = Vector3D::new(0.0, 0.0, near);
        let n_n: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
        let near_plane: Plane = Self::make_plane(p0_n, n_n);

        // Far Plane
        let p0_f: Vector3D = Vector3D::new(0.0, 0.0, far);
        let n_f: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
        let far_plane: Plane = Self::make_plane(p0_f, n_f);

        // Top Plane
        let p0_t: Vector3D = Vector3D::new(0.0, y_top, near);
        let n_t: Vector3D = Vector3D::new(0.0, near / y_top, -1.0).normalize();
        let top_plane: Plane = Self::make_plane(p0_t, n_t);

        // Bottom Plane
        let p0_b: Vector3D = Vector3D::new(0.0, -y_top, near);
        let n_b: Vector3D = Vector3D::new(0.0, -near / y_top, -1.0).normalize();
        let bottom_plane: Plane = Self::make_plane(p0_b, n_b);

        // Left Plane
        let p0_l: Vector3D = Vector3D::new(-x_right, 0.0, near);
        let n_l: Vector3D = Vector3D::new(-near / x_right, 0.0, -1.0).normalize();
        let left_plane: Plane = Self::make_plane(p0_l, n_l);

        // Right Plane
        let p0_r: Vector3D = Vector3D::new(x_right, 0.0, near);
        let n_r: Vector3D = Vector3D::new(near / x_right, 0.0, -1.0).normalize();
        let right_plane: Plane = Self::make_plane(p0_r, n_r);

        let planes: Vec<Plane> = vec![
            near_plane,
            far_plane,
            top_plane,
            bottom_plane,
            left_plane,
            right_plane,
        ];
        planes
    }

    fn make_plane(p0: Vector3D, n: Vector3D) -> Plane {
        let n: Vector3D = n.normalize();
        let a: f64 = -n.x;
        let b: f64 = -n.y;
        let c: f64 = -n.z;
        let d: f64 = -p0.dot_product(&n);
        Plane { a, b, c, d }
    }

    fn get_plane_distance(&self, point: Vector3D, plane: &Plane) -> f64 {
        let x: f64 = point.x;
        let y: f64 = point.y;
        let z: f64 = point.z;
        plane.a * x + plane.b * y + plane.c * z + plane.d
    }

    fn get_plane_intersection(&self, a: Vector3D, b: Vector3D, plane: &Plane) -> f64 {
        let ax: f64 = a.x;
        let ay: f64 = a.y;
        let az: f64 = a.z;
        let bx: f64 = b.x;
        let by: f64 = b.y;
        let bz: f64 = b.z;

        let distance: f64 = -self.get_plane_distance(a, plane);
        let interpolation: f64 = plane.a * (bx - ax) + plane.b * (by - ay) + plane.c * (bz - az);
        if interpolation == 0.0 {
            return 0.0;
        }
        distance / interpolation
    }

    fn is_point_behind_plane(&self, point: Vector3D, plane: &Plane) -> bool {
        let x: f64 = point.x;
        let y: f64 = point.y;
        let z: f64 = point.z;
        let distance: f64 = plane.a * x + plane.b * y + plane.c * z + plane.d;
        distance < 0.0
    }

    fn is_point_in_frustum(&self, point: Vector3D, planes: &Vec<Plane>) -> bool {
        for plane in planes {
            if self.is_point_behind_plane(point, plane) {
                return false;
            }
        }
        true
    }

    pub fn is_polygon_outside_frustum(&self, polygon: &Polygon) -> bool {
        let triangle_vertices: [Vector3D; 3];
        let quad_vertices: [Vector3D; 4];
        let vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => {
                triangle_vertices = triangle.vertices;
                &triangle_vertices
            }
            Polygon::Quad(quad) => {
                quad_vertices = quad.vertices;
                &quad_vertices
            }
        };

        for plane in &self.planes {
            let mut all_points_behind_plane = true;
            for point in vertices {
                if !self.is_point_behind_plane(*point, plane) {
                    all_points_behind_plane = false;
                    break;
                }
            }
            if all_points_behind_plane {
                return true;
            }
        }

        false
    }

    fn get_faces(&self, vertex_length: usize) -> Vec<(usize, usize, usize)> {
        let mut faces: Vec<(usize, usize, usize)> = Vec::new();
        for i in 1..(vertex_length - 1) {
            let face: (usize, usize, usize) = (0, i, i + 1);
            faces.push(face);
        }
        faces
    }

    pub fn clip_polygon_against_plane(
        &self,
        polygon: Polygon,
        plane: &Plane,
    ) -> [Option<Polygon>; 3] {
        let mut output_polygons: [Option<Polygon>; 3] = [None, None, None];
        let mut output_polygons_len: usize = 0;

        match &polygon {
            Polygon::Quad(quad) => {
                let input_vertices: [Vector3D; 4] = quad.vertices;
                let mut output_vertices: [Vector3D; 8] = [Vector3D::default(0.0); 8];
                let mut vertex_length: usize = 0;

                for i in 0..4 {
                    let a: Vector3D = input_vertices[i];
                    let b: Vector3D = input_vertices[(i + 1) % 4];

                    let t: f64 = self.get_plane_intersection(a, b, plane);
                    let c: Vector3D = a.lerp_interpolation(&b, t);

                    let ap_inside: bool = self.is_point_behind_plane(a, plane);
                    let bp_inside: bool = self.is_point_behind_plane(b, plane);

                    if !bp_inside {
                        if ap_inside {
                            output_vertices[vertex_length] = c;
                            vertex_length += 1;
                        }

                        output_vertices[vertex_length] = b;
                        vertex_length += 1;
                    } else if !ap_inside {
                        output_vertices[vertex_length] = c;
                        vertex_length += 1;
                    }
                }

                if vertex_length > 2 {
                    let faces: Vec<(usize, usize, usize)> = self.get_faces(vertex_length);
                    for face in faces.iter() {
                        let new_vertices: [Vector3D; 3] = [
                            output_vertices[face.0],
                            output_vertices[face.1],
                            output_vertices[face.2],
                        ];
                        let new_polygon: Polygon = Polygon::Triangle(Triangle::new(
                            new_vertices,
                            *face,
                            quad.shader,
                            quad.color,
                        ));
                        output_polygons[output_polygons_len] = Some(new_polygon);
                        output_polygons_len += 1;
                    }
                }
            }
            Polygon::Triangle(triangle) => {
                let input_vertices: [Vector3D; 3] = triangle.vertices;
                let mut output_vertices: [Vector3D; 8] = [Vector3D::default(0.0); 8];
                let mut output_faces: [usize; 8] = [0usize; 8];
                let mut vertex_length: usize = 0;

                for i in 0..3 {
                    let a: Vector3D = input_vertices[i];
                    let b: Vector3D = input_vertices[(i + 1) % 3];

                    let t: f64 = self.get_plane_intersection(a, b, plane);
                    let c: Vector3D = a.lerp_interpolation(&b, t);

                    let ap_inside: bool = self.is_point_behind_plane(a, plane);
                    let bp_inside: bool = self.is_point_behind_plane(b, plane);

                    if !bp_inside {
                        if ap_inside {
                            output_vertices[vertex_length] = c;
                            output_faces[vertex_length] = vertex_length;
                            vertex_length += 1;
                        }

                        output_vertices[vertex_length] = b;
                        output_faces[vertex_length] = vertex_length;
                        vertex_length += 1;
                    } else if !ap_inside {
                        output_vertices[vertex_length] = c;
                        output_faces[vertex_length] = vertex_length;
                        vertex_length += 1;
                    }
                }

                if vertex_length > 2 {
                    let faces: Vec<(usize, usize, usize)> = self.get_faces(vertex_length);
                    for face in faces.iter() {
                        let new_vertices: [Vector3D; 3] = [
                            output_vertices[face.0],
                            output_vertices[face.1],
                            output_vertices[face.2],
                        ];
                        let new_polygon: Polygon = Polygon::Triangle(Triangle::new(
                            new_vertices,
                            *face,
                            triangle.shader,
                            triangle.color,
                        ));
                        output_polygons[output_polygons_len] = Some(new_polygon);
                        output_polygons_len += 1;
                    }
                }
            }
        }
        output_polygons
    }

    pub fn clip_polygon_against_frustum(&self, polygon: Polygon) -> Vec<Polygon> {
        let mut clipped_polygons: Vec<Polygon> = vec![polygon];

        for plane in &self.planes {
            let mut new_polygons: Vec<Polygon> = Vec::new();
            for poly in clipped_polygons {
                let clipped_result: [Option<Polygon>; 3] =
                    self.clip_polygon_against_plane(poly, plane);
                for clipped_poly in clipped_result {
                    if let Some(polygon) = clipped_poly {
                        new_polygons.push(polygon);
                    }
                }
            }
            clipped_polygons = new_polygons;
        }

        clipped_polygons
    }
}
