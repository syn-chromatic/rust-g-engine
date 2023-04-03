use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use std::cmp::Ordering::Equal;

pub struct ZBufferSort {}

impl ZBufferSort {
    pub fn new() -> Self {
        Self {}
    }

    fn get_centroid_distance(&self, polygon: &Polygon, camera_position: Vector3D) -> f64 {
        let centroid: Vector3D = polygon.get_centroid();
        let distance: f64 = camera_position.get_distance(&centroid);
        distance
    }

    fn get_polygon_max_z(&self, polygon: &Polygon, camera_position: Vector3D) -> f64 {
        let vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };

        let mut max_z: f64 = f64::MIN;

        for vertex in vertices {
            let distance: f64 = camera_position.get_distance(vertex);

            if distance > max_z {
                max_z = distance;
            }
        }
        max_z
    }

    fn merge_sort(&self, distances: &mut Vec<(f64, usize)>, left: usize, right: usize) {
        if left < right {
            let mid: usize = left + (right - left) / 2;
            self.merge_sort(distances, left, mid);
            self.merge_sort(distances, mid + 1, right);
            self.merge(distances, left, mid, right);
        }
    }

    fn merge(&self, distances: &mut Vec<(f64, usize)>, left: usize, mid: usize, right: usize) {
        let n1: usize = mid - left + 1;
        let n2: usize = right - mid;

        let left_distances: Vec<(f64, usize)> = distances[left..(left + n1)].to_vec();
        let right_distances: Vec<(f64, usize)> = distances[(mid + 1)..(mid + 1 + n2)].to_vec();

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = left;

        while i < n1 && j < n2 {
            if left_distances[i].0 > right_distances[j].0 {
                distances[k] = left_distances[i];
                i += 1;
            } else {
                distances[k] = right_distances[j];
                j += 1;
            }
            k += 1;
        }

        while i < n1 {
            distances[k] = left_distances[i];
            i += 1;
            k += 1;
        }

        while j < n2 {
            distances[k] = right_distances[j];
            j += 1;
            k += 1;
        }
    }

    pub fn get_sorted_polygons(&self, mut mesh: Mesh, camera_position: Vector3D) -> Mesh {
        let polygons = mesh.polygons;
        let mut distances: Vec<(f64, usize)> = Vec::new();

        for i in 0..polygons.len() {
            let distance: f64 = self.get_centroid_distance(&polygons[i], camera_position);
            distances.push((distance, i));
        }

        // self.merge_sort(&mut distances, 0, len - 1);
        distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal).reverse());

        let mut sorted_polygons = Vec::new();

        for (_, index) in distances {
            sorted_polygons.push(polygons[index].clone());
        }

        mesh.polygons = sorted_polygons;
        mesh
    }
}
