use crate::components::color::RGBA;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct MeshDecompose {
    tolerance: f64,
}

impl MeshDecompose {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    pub fn decompose(&self, mesh: &Mesh) -> Vec<Mesh> {
        let mut result: Vec<Mesh> = Vec::new();

        if self.is_convex(&mesh) {
            result.push(mesh.clone());
            return result;
        }

        let (left, right): (Option<Mesh>, Option<Mesh>) = self.split_mesh(mesh);

        if let Some(left_mesh) = left {
            result.append(&mut self.decompose(&left_mesh));
        }

        if let Some(right_mesh) = right {
            result.append(&mut self.decompose(&right_mesh));
        }

        result
    }

    fn is_convex(&self, mesh: &Mesh) -> bool {
        for poly in &mesh.polygons {
            let normal: Vector3D = poly.get_normal();
            for neighbor in &mesh.polygons {
                let neighbor_normal: Vector3D = neighbor.get_normal();
                if normal.get_angle_between(&neighbor_normal) > self.tolerance {
                    return false;
                }
            }
        }
        true
    }

    fn randomize_mesh_color(&self, mut mesh: Mesh) -> Mesh {
        let random_color: RGBA = RGBA::from_random();
        for polygon in mesh.polygons.iter_mut() {
            polygon.set_color(&random_color);
        }
        mesh
    }

    fn split_mesh(&self, mesh: &Mesh) -> (Option<Mesh>, Option<Mesh>) {
        let (min, max): ([f64; 3], [f64; 3]) = mesh.get_bounding_box();
        let mut axis: usize = 0;
        let mut max_extent: f64 = max[0] - min[0];
        for i in 1..3 {
            let extent: f64 = max[i] - min[i];
            if extent > max_extent {
                max_extent = extent;
                axis = i;
            }
        }

        let mid: f64 = (min[axis] + max[axis]) / 2.0;
        let mut left_polygons: Vec<Polygon> = Vec::new();
        let mut right_polygons: Vec<Polygon> = Vec::new();

        for poly in &mesh.polygons {
            let centroid: [f64; 3] = poly.get_centroid().to_array();
            if centroid[axis] < mid {
                left_polygons.push(poly.clone());
            } else {
                right_polygons.push(poly.clone());
            }
        }

        let left_mesh: Option<Mesh> = if !left_polygons.is_empty() {
            let mesh: Mesh = Mesh::new(left_polygons);
            let mesh: Mesh = self.randomize_mesh_color(mesh);
            Some(mesh)
        } else {
            None
        };

        let right_mesh: Option<Mesh> = if !right_polygons.is_empty() {
            let mesh: Mesh = Mesh::new(right_polygons);
            let mesh: Mesh = self.randomize_mesh_color(mesh);
            Some(mesh)
        } else {
            None
        };

        (left_mesh, right_mesh)
    }
}
