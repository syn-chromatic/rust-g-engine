use std::f64::consts::PI;

use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::polygons::Triangle;
use crate::components::vectors::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    A: f64,
    B: f64,
    C: f64,
    D: f64,
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
        let mut frustum = Frustum {
            width,
            height,
            fov: 100.0,
            near_plane: 0.1,
            far_plane: 20_000.0,
            planes,
        };
        let planes = frustum.make_frustum();
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
        let pA: f64 = -n.x;
        let pB: f64 = -n.y;
        let pC: f64 = -n.z;
        let pD: f64 = -p0.dot_product(&n);
        Plane {
            A: pA,
            B: pB,
            C: pC,
            D: pD,
        }
    }

    fn get_plane_distance(&self, point: Vector3D, plane: &Plane) -> f64 {
        let x = point.x;
        let y = point.y;
        let z = point.z;
        plane.A * x + plane.B * y + plane.C * z + plane.D
    }

    fn get_plane_intersection(&self, a: Vector3D, b: Vector3D, plane: &Plane) -> f64 {
        let ax = a.x;
        let ay = a.y;
        let az = a.z;
        let bx = b.x;
        let by = b.y;
        let bz = b.z;

        let distance = -self.get_plane_distance(a, plane);
        let interpolation = plane.A * (bx - ax) + plane.B * (by - ay) + plane.C * (bz - az);
        if interpolation == 0.0 {
            return 0.0;
        }
        distance / interpolation
    }

    fn is_point_behind_plane(&self, point: Vector3D, plane: &Plane) -> bool {
        let x = point.x;
        let y = point.y;
        let z = point.z;
        let distance = plane.A * x + plane.B * y + plane.C * z + plane.D;
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

    fn get_triangle_faces(&self, output_polygon: &[usize]) -> Vec<(usize, usize, usize)> {
        let mut faces = Vec::new();
        for i in 1..(output_polygon.len() - 1) {
            let face = (output_polygon[0], output_polygon[i], output_polygon[i + 1]);
            faces.push(face);
        }
        faces
    }

    pub fn clip_against_plane(&self, mesh: &mut Mesh, plane: &Plane) {
        let mut output_polygons = Vec::new();

        for polygon in &mesh.polygons {
            match polygon {
                Polygon::Quad(_) => continue,
                Polygon::Triangle(triangle) => {
                    let input_vertices = triangle.vertices;
                    let mut output_vertices = Vec::new();
                    let mut output_faces = Vec::new();

                    let mut vertex_length = output_vertices.len();

                    for i in 0..3 {
                        let a = input_vertices[i];
                        let b = input_vertices[(i + 1) % 3];

                        let t = self.get_plane_intersection(a, b, plane);
                        let c = a.lerp_interpolation(&b, t);

                        let ap_inside = self.is_point_behind_plane(a, plane);
                        let bp_inside = self.is_point_behind_plane(b, plane);

                        if !bp_inside {
                            if ap_inside {
                                output_vertices.push(c);
                                output_faces.push(vertex_length);
                                vertex_length += 1
                            }

                            output_vertices.push(b);
                            output_faces.push(vertex_length);
                            vertex_length += 1
                        } else if !ap_inside {
                            output_vertices.push(c);
                            output_faces.push(vertex_length);
                            vertex_length += 1
                        }
                    }

                    if output_faces.len() > 2 {
                        let faces = self.get_triangle_faces(&output_faces);
                        for face in faces {
                            let new_vertices = [
                                output_vertices[face.0],
                                output_vertices[face.1],
                                output_vertices[face.2],
                            ];
                            let new_polygon = Polygon::Triangle(Triangle::new(
                                new_vertices,
                                face,
                                triangle.shader.clone(),
                                triangle.color.clone(),
                            ));
                            output_polygons.push(new_polygon);
                        }
                    }
                }
            }
        }

        mesh.polygons = output_polygons;
    }

    pub fn frustum_clip(&self, mesh: &mut Mesh) {
        for plane in &self.planes {
            self.clip_against_plane(mesh, plane);
        }
    }
}
