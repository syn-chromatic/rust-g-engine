use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use rayon::prelude::ParallelSliceMut;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BVHNode {
    pub polygons: Vec<Polygon>,
    left: Option<Arc<BVHNode>>,
    right: Option<Arc<BVHNode>>,
    aabb: ([f64; 3], [f64; 3]),
    pub max_leaf_size: usize,
}

impl BVHNode {
    pub fn new(polygons: &Vec<Polygon>) -> Self {
        let aabb: ([f64; 3], [f64; 3]) = Self::get_aabb(polygons);
        let polygons: Vec<Polygon> = polygons.clone();
        let max_leaf_size: usize = 32;

        let mut bvh_node: BVHNode = BVHNode {
            polygons,
            left: None,
            right: None,
            aabb,
            max_leaf_size,
        };
        bvh_node.split();
        bvh_node
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
            let v = p.get_centroid().to_vec();
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

        self.left = Some(Arc::new(BVHNode::new(&left_polygons.to_vec())));
        self.right = Some(Arc::new(BVHNode::new(&right_polygons.to_vec())));
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
