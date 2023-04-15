use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;

use itertools::Itertools;
use itertools::Unique;
use rayon::prelude::ParallelSliceMut;
use std::cmp::Ordering;
use std::sync::Arc;

const EPSILON: f64 = 1e-6;
fn gift_wrapping(vertices: &[Vector3D]) -> Vec<Vector3D> {
    let mut hull: Vec<Vector3D> = Vec::new();

    let leftmost_vertex: Vector3D = *vertices
        .iter()
        .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
        .unwrap();

    let mut p: Vector3D = leftmost_vertex;
    let mut q: Vector3D;

    loop {
        hull.push(p);
        q = *vertices.iter().find(|&&v| v != p).unwrap_or(&p); // Find the first vertex that is not p

        for r in vertices {
            if r == &p {
                continue;
            }
            let orientation: f64 = (q.subtract_vector(&p))
                .cross_product(&(r.subtract_vector(&p)))
                .z;

            if orientation > EPSILON {
                q = *r;
            } else if orientation.abs() <= EPSILON {
                let dist_pq = q.subtract_vector(&p).get_length();
                let dist_pr = r.subtract_vector(&p).get_length();

                if dist_pr > dist_pq {
                    q = *r;
                }
            }
        }
        p = q;

        if p == leftmost_vertex {
            break;
        }
    }

    hull
}

fn graham_scan(mut vertices: Vec<Vector3D>) -> Vec<Vector3D> {
    vertices.sort_unstable_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal))
    });

    let mut hull: Vec<Vector3D> = Vec::new();
    for &vertex in vertices.iter().chain(vertices.iter().rev().skip(1)) {
        while hull.len() >= 2
            && (hull[hull.len() - 1].subtract_vector(&hull[hull.len() - 2]))
                .cross_product(&(vertex.subtract_vector(&hull[hull.len() - 2])))
                .z
                < EPSILON
        {
            hull.pop();
        }
        hull.push(vertex);
    }

    hull.pop();
    hull
}
fn remove_duplicates(points: &mut Vec<Vector3D>) {
    points.sort_unstable_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal))
    });
    points.dedup_by(|a, b| {
        (a.x - b.x).abs() < f64::EPSILON
            && (a.y - b.y).abs() < f64::EPSILON
            && (a.z - b.z).abs() < f64::EPSILON
    });
}

fn merge_hulls(hull1: &Vec<Vector3D>, hull2: &Vec<Vector3D>) -> Vec<Vector3D> {
    let mut merged_hull: Vec<Vector3D> = hull1.clone();
    merged_hull.extend(hull2.clone());
    graham_scan(merged_hull)
}

pub fn chans_algorithm(vertices: &[Vector3D], m: usize) -> Vec<Vector3D> {
    let n: usize = vertices.len();
    let mut hulls: Vec<Vec<Vector3D>> = Vec::new();
    let step_size: usize = usize::min(m, n);

    for i in (0..n).step_by(step_size) {
        let sub_hull_vertices: &[Vector3D] = &vertices[i..usize::min(i + step_size, n)];
        let sub_hull: Vec<Vector3D> = gift_wrapping(sub_hull_vertices);
        hulls.push(sub_hull);
    }

    while hulls.len() > 1 {
        let mut new_hulls: Vec<Vec<Vector3D>> = Vec::new();
        for hulls_chunk in hulls.chunks(2) {
            match hulls_chunk.len() {
                1 => new_hulls.push(hulls_chunk[0].clone()),
                2 => new_hulls.push(merge_hulls(&hulls_chunk[0], &hulls_chunk[1])),
                _ => (),
            }
        }
        hulls = new_hulls;
    }

    hulls.into_iter().next().unwrap_or_else(Vec::new)
}

fn get_left_set_a(
    vertices: &[Vector3D],
    farthest_vertex: &Vector3D,
    p1: &Vector3D,
) -> Vec<Vector3D> {
    let left_set_a: Vec<Vector3D> = vertices
        .iter()
        .filter(|v| {
            ((farthest_vertex.subtract_vector(p1)).cross_product(&(v.subtract_vector(p1)))).z
                > EPSILON
        })
        .copied()
        .collect();
    left_set_a
}

fn get_left_set_b(
    vertices: &[Vector3D],
    farthest_vertex: &Vector3D,
    p2: &Vector3D,
) -> Vec<Vector3D> {
    let left_set_b: Vec<Vector3D> = vertices
        .iter()
        .filter(|v| {
            ((p2.subtract_vector(farthest_vertex))
                .cross_product(&(v.subtract_vector(farthest_vertex))))
            .z > EPSILON
        })
        .copied()
        .collect();
    left_set_b
}

fn recursive_hull(vertices: &[Vector3D], p1: &Vector3D, p2: &Vector3D, hull: &mut Vec<Vector3D>) {
    if vertices.is_empty() {
        return;
    }

    let mut farthest_vertex: &Vector3D = &vertices[0];
    let mut max_distance: f64 = 0.0;

    for vertex in vertices {
        let base: Vector3D = p2.subtract_vector(p1);
        let cross_product: Vector3D = base.cross_product(&(vertex.subtract_vector(p1)));
        let distance: f64 = cross_product.get_length() / base.get_length();
        if distance > max_distance {
            farthest_vertex = vertex;
            max_distance = distance;
        }
    }
    hull.push(*farthest_vertex);

    let left_set_a: Vec<Vector3D> = get_left_set_a(vertices, farthest_vertex, p1);
    recursive_hull(&left_set_a, p1, farthest_vertex, hull);
    let left_set_b: Vec<Vector3D> = get_left_set_b(vertices, farthest_vertex, p2);
    recursive_hull(&left_set_b, farthest_vertex, p2, hull);
}

pub fn quick_hull(vertices: &[Vector3D]) -> Vec<Vector3D> {
    let mut hull: Vec<Vector3D> = Vec::new();
    let min_vertex = vertices
        .iter()
        .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
        .unwrap();

    let max_vertex = vertices
        .iter()
        .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
        .unwrap();

    let left_set: Vec<Vector3D> = vertices
        .iter()
        .filter(|v| {
            ((max_vertex.subtract_vector(min_vertex))
                .cross_product(&(v.subtract_vector(min_vertex))))
            .z > EPSILON
        })
        .copied()
        .collect();

    let right_set: Vec<Vector3D> = vertices
        .iter()
        .filter(|v| {
            ((min_vertex.subtract_vector(max_vertex))
                .cross_product(&(v.subtract_vector(max_vertex))))
            .z > EPSILON
        })
        .copied()
        .collect();

    hull.push(*min_vertex);
    recursive_hull(&left_set, min_vertex, max_vertex, &mut hull);
    hull.push(*max_vertex);
    recursive_hull(&right_set, max_vertex, min_vertex, &mut hull);
    hull
}

#[derive(Clone, Debug)]
pub struct BVHNode {
    pub polygons: Vec<Polygon>,
    left: Option<Arc<BVHNode>>,
    right: Option<Arc<BVHNode>>,
    aabb: ([f64; 3], [f64; 3]),
    pub max_leaf_size: usize,
    pub convex_hull: Vec<Vector3D>,
}

impl BVHNode {
    pub fn new(polygons: &Vec<Polygon>, convex_hull: &Vec<Vector3D>) -> Self {
        let aabb: ([f64; 3], [f64; 3]) = Self::get_aabb(polygons);
        let polygons: Vec<Polygon> = polygons.clone();
        let max_leaf_size: usize = 32;

        let convex_hull: Vec<Vector3D> = convex_hull.clone();

        let mut bvh_node: BVHNode = BVHNode {
            polygons,
            left: None,
            right: None,
            aabb,
            max_leaf_size,
            convex_hull,
        };
        bvh_node.split();
        bvh_node
    }
    pub fn project_onto_axis(&self, axis: &Vector3D) -> (f64, f64) {
        let mut min: f64 = f64::MAX;
        let mut max: f64 = f64::MIN;

        for vertex in self.convex_hull.iter() {
            let dot: f64 = vertex.dot_product(axis);
            min = min.min(dot);
            max = max.max(dot);
        }

        (min, max)
    }

    fn are_projections_overlapping(&self, other: &BVHNode, axis: &Vector3D) -> bool {
        let normalized_axis = axis.normalize();
        let (min_a, max_a): (f64, f64) = self.project_onto_axis(&normalized_axis);
        let (min_b, max_b): (f64, f64) = other.project_onto_axis(&normalized_axis);
        let epsilon: f64 = 1e-6;

        !(max_a < min_b - epsilon || max_b < min_a - epsilon)
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

    // fn sat_intersection(&self, other: &BVHNode) -> bool {
    //     let edges_a: Vec<Vector3D> = self.get_edges();
    //     let edges_b: Vec<Vector3D> = other.get_edges();

    //     for edge_a in &edges_a {
    //         let axis_a: Vector3D = edge_a
    //             .cross_product(&Vector3D::new(0.0, 0.0, 1.0))
    //             .normalize();
    //         if !self.are_projections_overlapping(other, &axis_a) {
    //             return false;
    //         }
    //     }

    //     for edge_b in &edges_b {
    //         let axis_b: Vector3D = edge_b
    //             .cross_product(&Vector3D::new(0.0, 0.0, 1.0))
    //             .normalize();
    //         if !self.are_projections_overlapping(other, &axis_b) {
    //             return false;
    //         }
    //     }

    //     for edge_a in &edges_a {
    //         for edge_b in &edges_b {
    //             let axis_c: Vector3D = edge_a.cross_product(edge_b).normalize();
    //             if axis_c.get_length() > 1e-6 && !self.are_projections_overlapping(other, &axis_c) {
    //                 return false;
    //             }
    //         }
    //     }

    //     true
    // }

    // pub fn is_intersecting(&self, other: &BVHNode) -> bool {
    //     // let (min_a, max_a): ([f64; 3], [f64; 3]) = self.aabb;
    //     // let (min_b, max_b): ([f64; 3], [f64; 3]) = other.aabb;

    //     // for i in 0..3 {
    //     //     if min_a[i] > max_b[i] || min_b[i] > max_a[i] {
    //     //         return false;
    //     //     }
    //     // }

    //     let intersected = self.sat_intersection(other);
    //     // if intersected {
    //     //     let distance = self.get_distance(other);
    //     //     println!("{}, {}", intersected, distance);

    //     // }
    //     intersected
    // }

    // fn sat_intersection(&self, other: &BVHNode) -> Option<Vector3D> {
    //     let mut mtv: Option<Vector3D> = None;
    //     let mut min_overlap: f64 = f64::MAX;

    //     let edges_a: Vec<Vector3D> = self.get_edges();
    //     let edges_b: Vec<Vector3D> = other.get_edges();

    //     // Check edge axes for both objects
    //     for edge_list in [&edges_a, &edges_b] {
    //         for edge in edge_list {
    //             let axis: Vector3D = edge
    //                 .cross_product(&Vector3D::new(0.0, 0.0, 1.0))
    //                 .normalize();
    //             let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
    //             let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

    //             // If there is no overlap, objects are not intersecting
    //             if max_a < min_b || max_b < min_a {
    //                 return None;
    //             }

    //             // Calculate overlap along the current axis
    //             let overlap = (max_a - min_b).min(max_b - min_a);

    //             // Update minimum translation vector (MTV) if the current overlap is smaller
    //             if overlap < min_overlap {
    //                 min_overlap = overlap;
    //                 mtv = Some(axis.multiply(overlap));
    //             }
    //         }
    //     }

    //     // Check all pairs of edge cross products for both objects
    //     for edge_a in &edges_a {
    //         for edge_b in &edges_b {
    //             let axis: Vector3D = edge_a.cross_product(edge_b).normalize();

    //             if axis.get_length() > 1e-6 {
    //                 let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
    //                 let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

    //                 // If there is no overlap, objects are not intersecting
    //                 if max_a < min_b || max_b < min_a {
    //                     return None;
    //                 }

    //                 // Calculate overlap along the current axis
    //                 let overlap = (max_a - min_b).min(max_b - min_a);

    //                 // Update minimum translation vector (MTV) if the current overlap is smaller
    //                 if overlap < min_overlap {
    //                     min_overlap = overlap;
    //                     mtv = Some(axis.multiply(overlap));
    //                 }
    //             }
    //         }
    //     }
    //     mtv
    // }

    fn sat_intersection(&self, other: &BVHNode) -> Option<Vector3D> {
        let mut mtv: Option<Vector3D> = None;
        let mut min_overlap: f64 = f64::MAX;
        let epsilon: f64 = 1e-6;

        let edges_a: Vec<Vector3D> = self.get_edges();
        let edges_b: Vec<Vector3D> = other.get_edges();

        for edge_list in [&edges_a, &edges_b] {
            for edge in edge_list {
                let axis: Vector3D = edge
                    .cross_product(&Vector3D::new(0.0, 0.0, 1.0))
                    .normalize();
                let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
                let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

                if max_a < min_b || max_b < min_a {
                    return None;
                }

                let overlap = (max_a - min_b).min(max_b - min_a);
                if overlap < min_overlap && overlap > epsilon {
                    min_overlap = overlap;
                    mtv = Some(axis.multiply(overlap));
                }
            }
        }

        for edge_a in &edges_a {
            for edge_b in &edges_b {
                let axis: Vector3D = edge_a.cross_product(edge_b).normalize();

                if axis.get_length() > epsilon {
                    let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
                    let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

                    if max_a < min_b || max_b < min_a {
                        return None;
                    }

                    let overlap = (max_a - min_b).min(max_b - min_a);
                    if overlap < min_overlap && overlap > epsilon {
                        min_overlap = overlap;
                        mtv = Some(axis.multiply(overlap));
                    }
                }
            }
        }

        mtv
    }

    pub fn is_intersecting(&self, other: &BVHNode) -> Option<Vector3D> {
        // let (min_a, max_a): ([f64; 3], [f64; 3]) = self.aabb;
        // let (min_b, max_b): ([f64; 3], [f64; 3]) = other.aabb;

        // for i in 0..3 {
        //     if min_a[i] > max_b[i] || min_b[i] > max_a[i] {
        //         return None;
        //     }
        // }

        self.sat_intersection(other)
    }

    pub fn get_vertices(&self) -> Vec<Vector3D> {
        let mut vertices: Vec<Vector3D> = Vec::new();

        for polygon in &self.polygons {
            let polygon_vertices: &[Vector3D] = match polygon {
                Polygon::Triangle(triangle) => &triangle.vertices,
                Polygon::Quad(quad) => &quad.vertices,
            };

            vertices.extend_from_slice(polygon_vertices);
        }

        vertices
    }

    pub fn get_edges(&self) -> Vec<Vector3D> {
        let mut edges: Vec<Vector3D> = Vec::new();

        for polygon in &self.polygons {
            let polygon_vertices: &[Vector3D] = match polygon {
                Polygon::Triangle(triangle) => &triangle.vertices,
                Polygon::Quad(quad) => &quad.vertices,
            };

            let num_vertices: usize = polygon_vertices.len();
            for i in 0..num_vertices {
                let edge: Vector3D =
                    polygon_vertices[(i + 1) % num_vertices].subtract_vector(&polygon_vertices[i]);
                edges.push(edge);
            }
        }

        edges
    }

    pub fn get_distance_bounding_boxes(&self, other: &BVHNode) -> f64 {
        let (min_a, max_a): ([f64; 3], [f64; 3]) = self.aabb;
        let (min_b, max_b): ([f64; 3], [f64; 3]) = other.aabb;

        let centroid_a: [f64; 3] = [
            (min_a[0] + max_a[0]) / 2.0,
            (min_a[1] + max_a[1]) / 2.0,
            (min_a[2] + max_a[2]) / 2.0,
        ];

        let centroid_b: [f64; 3] = [
            (min_b[0] + max_b[0]) / 2.0,
            (min_b[1] + max_b[1]) / 2.0,
            (min_b[2] + max_b[2]) / 2.0,
        ];

        let distance: f64 = (centroid_a[0] - centroid_b[0]).powi(2)
            + (centroid_a[1] - centroid_b[1]).powi(2)
            + (centroid_a[2] - centroid_b[2]).powi(2);

        let mut intersection: bool = true;
        let mut overlap: f64 = f64::INFINITY;

        for i in 0..3 {
            if max_a[i] < min_b[i] || max_b[i] < min_a[i] {
                intersection = false;
                break;
            } else {
                let min_overlap = (max_a[i].min(max_b[i])) - (min_a[i].max(min_b[i]));
                overlap = overlap.min(min_overlap);
            }
        }

        if intersection {
            return -overlap;
        }

        distance.sqrt()
    }

    pub fn get_distance(&self, other: &BVHNode) -> f64 {
        self.get_distance_bounding_boxes(other)
    }

    pub fn split(&mut self) {
        if self.polygons.len() <= self.max_leaf_size {
            return;
        }

        let (min_corner, max_corner): ([f64; 3], [f64; 3]) = self.aabb;
        let axis_lengths: [f64; 3] = [
            max_corner[0] - min_corner[0],
            max_corner[1] - min_corner[1],
            max_corner[2] - min_corner[2],
        ];
        let split_axis: usize = self.argmax_vector(axis_lengths);

        let centroid = |p: &Polygon| {
            let v = p.get_centroid().to_array();
            v[split_axis]
        };

        let mid_point: usize = self.polygons.len() / 2;

        self.polygons.par_sort_unstable_by(|a, b| {
            centroid(a)
                .partial_cmp(&centroid(b))
                .unwrap_or(Ordering::Equal)
        });

        let (left_polygons, right_polygons): (&[Polygon], &[Polygon]) =
            self.polygons.split_at(mid_point);

        self.left = Some(Arc::new(BVHNode::new(
            &left_polygons.to_vec(),
            &self.convex_hull,
        )));
        self.right = Some(Arc::new(BVHNode::new(
            &right_polygons.to_vec(),
            &self.convex_hull,
        )));
    }

    pub fn ray_intersect_aabb(&self, origin: &Vector3D, direction: &Vector3D) -> bool {
        let epsilon: f64 = 1e-6;
        let (min_corner, max_corner): ([f64; 3], [f64; 3]) = self.aabb;
        let inv_direction: [f64; 3] = [
            1.0 / (direction.x + epsilon),
            1.0 / (direction.y + epsilon),
            1.0 / (direction.z + epsilon),
        ];

        let t1: f64 = (min_corner[0] - origin.x) * inv_direction[0];
        let t2: f64 = (max_corner[0] - origin.x) * inv_direction[0];
        let t3: f64 = (min_corner[1] - origin.y) * inv_direction[1];
        let t4: f64 = (max_corner[1] - origin.y) * inv_direction[1];
        let t5: f64 = (min_corner[2] - origin.z) * inv_direction[2];
        let t6: f64 = (max_corner[2] - origin.z) * inv_direction[2];

        let t_min: f64 = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let t_max: f64 = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        t_max > t_min.max(0.0)
    }

    pub fn traverse(&self, origin: &Vector3D, direction: &Vector3D) -> Vec<Polygon> {
        if !self.ray_intersect_aabb(origin, direction) {
            return vec![];
        }

        let mut intersecting_polygons: Vec<Polygon> = vec![];

        if let Some(left_node) = &self.left {
            intersecting_polygons.extend(left_node.traverse(origin, direction));
        }

        if let Some(right_node) = &self.right {
            intersecting_polygons.extend(right_node.traverse(origin, direction));
        }

        if self.left.is_none() || self.right.is_none() {
            intersecting_polygons.extend(self.polygons.clone());
        }

        intersecting_polygons
    }

    fn get_aabb(polygons: &[Polygon]) -> ([f64; 3], [f64; 3]) {
        let mut min_corner: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max_corner: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];

        for polygon in polygons {
            let vertices: &[Vector3D] = match polygon {
                Polygon::Triangle(triangle) => &triangle.vertices,
                Polygon::Quad(quad) => &quad.vertices,
            };

            for vertex in vertices {
                min_corner[0] = min_corner[0].min(vertex.x);
                min_corner[1] = min_corner[1].min(vertex.y);
                min_corner[2] = min_corner[2].min(vertex.z);
                max_corner[0] = max_corner[0].max(vertex.x);
                max_corner[1] = max_corner[1].max(vertex.y);
                max_corner[2] = max_corner[2].max(vertex.z);
            }
        }

        (min_corner, max_corner)
    }

    fn argmax_vector(&self, vec: [f64; 3]) -> usize {
        let mut max_index = 0;
        let mut max_value = vec[0];

        if vec[1] > max_value {
            max_value = vec[1];
            max_index = 1;
        }
        if vec[2] > max_value {
            max_value = vec[2];
            max_index = 2;
        }

        max_index
    }
}
