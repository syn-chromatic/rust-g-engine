use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use rayon::prelude::ParallelSliceMut;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    polygons: Vec<Polygon>,
    left: Option<Arc<BVHNode>>,
    right: Option<Arc<BVHNode>>,
    aabb: (Vector3D, Vector3D),
}

impl BVHNode {
    pub fn new() -> Self {
        let polygons = vec![];
        let aabb = Self::calculate_aabb(&polygons);
        BVHNode {
            polygons,
            left: None,
            right: None,
            aabb,
        }
    }

    pub fn new_node(polygons: Vec<Polygon>) -> Self {
        let aabb = Self::calculate_aabb(&polygons);
        let mut bvh_node = BVHNode {
            polygons,
            left: None,
            right: None,
            aabb,
        };
        bvh_node.split();
        bvh_node
    }

    pub fn fresh_node(&self, polygons: Vec<Polygon>) -> Self {
        let aabb = Self::calculate_aabb(&polygons);
        let mut bvh_node = BVHNode {
            polygons,
            left: None,
            right: None,
            aabb,
        };
        bvh_node.split();
        bvh_node
    }

    fn calculate_aabb(polygons: &[Polygon]) -> (Vector3D, Vector3D) {
        let mut min_corner = Vector3D::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max_corner = Vector3D::new(f64::MIN, f64::MIN, f64::MIN);

        for polygon in polygons {
            let vertices: &[Vector3D] = match polygon {
                Polygon::Triangle(triangle) => &triangle.vertices,
                Polygon::Quad(quad) => &quad.vertices,
            };

            for vertex in vertices {
                min_corner.x = min_corner.x.min(vertex.x);
                min_corner.y = min_corner.y.min(vertex.y);
                min_corner.z = min_corner.z.min(vertex.z);
                max_corner.x = max_corner.x.max(vertex.x);
                max_corner.y = max_corner.y.max(vertex.y);
                max_corner.z = max_corner.z.max(vertex.z);
            }
        }

        (min_corner, max_corner)
    }

    fn argmax_vector(&self, vec: Vector3D) -> usize {
        let mut max_index = 0;
        let mut max_value = vec.x;

        if vec.y > max_value {
            max_value = vec.y;
            max_index = 1;
        }
        if vec.z > max_value {
            max_value = vec.z;
            max_index = 2;
        }

        max_index
    }

    pub fn extend_polygons(&mut self, new_polygons: Vec<Polygon>) {
        self.polygons.extend(new_polygons);
        self.aabb = Self::calculate_aabb(&self.polygons);
        self.left = None;
        self.right = None;
        self.split();
    }

    pub fn split(&mut self) {
        if self.polygons.len() <= 1 {
            return;
        }

        let (min_corner, max_corner) = self.aabb;
        let axis_lengths = max_corner.subtract_vector(&min_corner);
        let split_axis = self.argmax_vector(axis_lengths);

        let centroid = |p: &Polygon| {
            let v = p.get_centroid().to_vec();
            v[split_axis]
        };

        let mid_point = self.polygons.len() / 2;

        self.polygons.par_sort_unstable_by(|a, b| {
            centroid(a)
                .partial_cmp(&centroid(b))
                .unwrap_or(Ordering::Equal)
        });

        let (left_polygons, right_polygons) = self.polygons.split_at(mid_point);

        self.left = Some(Arc::new(BVHNode::new_node(left_polygons.to_vec())));
        self.right = Some(Arc::new(BVHNode::new_node(right_polygons.to_vec())));
    }

    pub fn ray_intersect_aabb(&self, origin: &Vector3D, direction: &Vector3D) -> bool {
        let epsilon: f64 = 1e-6;
        let (min_corner, max_corner) = self.aabb;
        let inv_direction = Vector3D::new(
            1.0 / (direction.x + epsilon),
            1.0 / (direction.y + epsilon),
            1.0 / (direction.z + epsilon),
        );

        let t1 = (min_corner.x - origin.x) * inv_direction.x;
        let t2 = (max_corner.x - origin.x) * inv_direction.x;
        let t3 = (min_corner.y - origin.y) * inv_direction.y;
        let t4 = (max_corner.y - origin.y) * inv_direction.y;
        let t5 = (min_corner.z - origin.z) * inv_direction.z;
        let t6 = (max_corner.z - origin.z) * inv_direction.z;

        let t_min = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let t_max = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        t_max > t_min.max(0.0)
    }

    pub fn traverse(&self, origin: &Vector3D, direction: &Vector3D) -> Vec<Polygon> {
        if !self.ray_intersect_aabb(origin, direction) {
            return vec![];
        }

        if self.left.is_none() && self.right.is_none() {
            return self.polygons.clone();
        }

        let mut intersecting_polygons = vec![];

        if let Some(left_node) = &self.left {
            intersecting_polygons.extend(left_node.traverse(origin, direction));
        }

        if let Some(right_node) = &self.right {
            intersecting_polygons.extend(right_node.traverse(origin, direction));
        }

        intersecting_polygons
    }
}

// pub struct BVHNode {
//     polygons: Vec<Polygon>,
//     left: Option<Arc<BVHNode>>,
//     right: Option<Arc<BVHNode>>,
//     aabb: (Vector3D, Vector3D),
// }

// impl BVHNode {
//     pub fn new(polygons: Vec<Polygon>) -> Self {
//         let aabb = Self::calculate_aabb(&polygons);
//         let mut node = BVHNode {
//             polygons,
//             left: None,
//             right: None,
//             aabb,
//         };
//         node.split();
//         node
//     }

//     fn calculate_aabb(polygons: &[Polygon]) -> (Vector3D, Vector3D) {
//         let mut min_corner = Vector3D::new(f64::MAX, f64::MAX, f64::MAX);
//         let mut max_corner = Vector3D::new(f64::MIN, f64::MIN, f64::MIN);

//         for polygon in polygons {
//             let vertices: &[Vector3D] = match polygon {
//                 Polygon::Triangle(triangle) => &triangle.vertices,
//                 Polygon::Quad(quad) => &quad.vertices,
//             };

//             for vertex in vertices {
//                 min_corner.x = min_corner.x.min(vertex.x);
//                 min_corner.y = min_corner.y.min(vertex.y);
//                 min_corner.z = min_corner.z.min(vertex.z);
//                 max_corner.x = max_corner.x.max(vertex.x);
//                 max_corner.y = max_corner.y.max(vertex.y);
//                 max_corner.z = max_corner.z.max(vertex.z);
//             }
//         }

//         (min_corner, max_corner)
//     }

//     fn argmax_vector(&self, vec: Vector3D) -> usize {
//         let mut max_index = 0;
//         let mut max_value = vec.x;

//         if vec.y > max_value {
//             max_value = vec.y;
//             max_index = 1;
//         }
//         if vec.z > max_value {
//             max_value = vec.z;
//             max_index = 2;
//         }

//         max_index
//     }

//     pub fn extend_polygons(&mut self, new_polygons: Vec<Polygon>) {
//         self.polygons.extend(new_polygons);
//         self.aabb = Self::calculate_aabb(&self.polygons);
//         self.left = None;
//         self.right = None;
//         self.split();
//     }

//     pub fn split(&mut self) {
//         if self.polygons.len() <= 1 {
//             return;
//         }

//         let (min_corner, max_corner) = self.aabb;
//         let axis_lengths = max_corner.subtract_vector(&min_corner);
//         let split_axis = self.argmax_vector(axis_lengths);

//         let centroid = |p: &Polygon| {
//             let v = p.get_centroid().to_vec();
//             v[split_axis]
//         };

//         let mid_point = self.polygons.len() / 2;

//         self.polygons.par_sort_unstable_by(|a, b| {
//             centroid(a)
//                 .partial_cmp(&centroid(b))
//                 .unwrap_or(Ordering::Equal)
//         });

//         let (left_polygons, right_polygons) = self.polygons.split_at(mid_point);

//         self.left = Some(Arc::new(BVHNode::new(left_polygons.to_vec())));
//         self.right = Some(Arc::new(BVHNode::new(right_polygons.to_vec())));
//     }
//     pub fn ray_intersect_aabb(&self, origin: &Vector3D, direction: &Vector3D) -> bool {
//         let epsilon: f64 = 1e-6;
//         let (min_corner, max_corner) = self.aabb;
//         let inv_direction = Vector3D::new(
//             1.0 / (direction.x + epsilon),
//             1.0 / (direction.y + epsilon),
//             1.0 / (direction.z + epsilon),
//         );

//         let t1 = (min_corner.x - origin.x) * inv_direction.x;
//         let t2 = (max_corner.x - origin.x) * inv_direction.x;
//         let t3 = (min_corner.y - origin.y) * inv_direction.y;
//         let t4 = (max_corner.y - origin.y) * inv_direction.y;
//         let t5 = (min_corner.z - origin.z) * inv_direction.z;
//         let t6 = (max_corner.z - origin.z) * inv_direction.z;

//         let t_min = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
//         let t_max = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

//         t_max > t_min.max(0.0)
//     }

//     pub fn traverse(&self, origin: &Vector3D, direction: &Vector3D) -> Vec<Polygon> {
//         if !self.ray_intersect_aabb(origin, direction) {
//             return vec![];
//         }

//         if self.left.is_none() && self.right.is_none() {
//             return self.polygons.clone();
//         }

//         let mut intersecting_polygons = vec![];

//         if let Some(left_node) = &self.left {
//             intersecting_polygons.extend(left_node.traverse(origin, direction));
//         }

//         if let Some(right_node) = &self.right {
//             intersecting_polygons.extend(right_node.traverse(origin, direction));
//         }

//         intersecting_polygons
//     }
// }

// pub struct BVHNode {
//     polygons: Vec<Polygon>,
//     left: Option<Box<BVHNode>>,
//     right: Option<Box<BVHNode>>,
//     aabb: (Vector3D, Vector3D),
// }

// impl BVHNode {
//     pub fn new(polygons: Vec<Polygon>) -> Self {
//         let aabb = Self::calculate_aabb(&polygons);
//         BVHNode {
//             polygons,
//             left: None,
//             right: None,
//             aabb,
//         }
//     }

//     fn calculate_aabb(polygons: &[Polygon]) -> (Vector3D, Vector3D) {
//         let mut min_corner = Vector3D::new(f64::MAX, f64::MAX, f64::MAX);
//         let mut max_corner = Vector3D::new(f64::MIN, f64::MIN, f64::MIN);

//         for polygon in polygons {
//             let vertices: &[Vector3D] = match polygon {
//                 Polygon::Triangle(triangle) => &triangle.vertices,
//                 Polygon::Quad(quad) => &quad.vertices,
//             };

//             for vertex in vertices {
//                 min_corner.x = min_corner.x.min(vertex.x);
//                 min_corner.y = min_corner.y.min(vertex.y);
//                 min_corner.z = min_corner.z.min(vertex.z);
//                 max_corner.x = max_corner.x.max(vertex.x);
//                 max_corner.y = max_corner.y.max(vertex.y);
//                 max_corner.z = max_corner.z.max(vertex.z);
//             }
//         }

//         (min_corner, max_corner)
//     }

//     fn argmax_vector(&self, vec: Vector3D) -> usize {
//         let mut max_index = 0;
//         let mut max_value = vec.x;

//         if vec.y > max_value {
//             max_value = vec.y;
//             max_index = 1;
//         }
//         if vec.z > max_value {
//             max_value = vec.z;
//             max_index = 2;
//         }

//         max_index
//     }

//     pub fn extend_polygons(&mut self, new_polygons: Vec<Polygon>) {
//         self.polygons.extend(new_polygons);
//         self.aabb = Self::calculate_aabb(&self.polygons);
//         self.left = None;
//         self.right = None;
//         self.split();
//     }

//     pub fn split(&mut self) -> (Option<&BVHNode>, Option<&BVHNode>) {
//         if self.polygons.len() <= 1 {
//             return (None, None);
//         }

//         let (min_corner, max_corner) = self.aabb;
//         let axis_lengths = max_corner.subtract_vector(&min_corner);
//         let split_axis = self.argmax_vector(axis_lengths);

//         let mut sorted_polygons = self.polygons.clone();
//         sorted_polygons.sort_by(|a, b| {
//             a.get_centroid().to_vec()[split_axis]
//                 .partial_cmp(&b.get_centroid().to_vec()[split_axis])
//                 .unwrap_or(Ordering::Equal)
//         });

//         let mid_point = sorted_polygons.len() / 2;

//         let left_polygons = sorted_polygons[0..mid_point].to_vec();
//         let right_polygons = sorted_polygons[mid_point..].to_vec();

//         self.left = Some(Box::new(BVHNode::new(left_polygons)));
//         self.right = Some(Box::new(BVHNode::new(right_polygons)));

//         (
//             self.left.as_ref().map(|x| &**x),
//             self.right.as_ref().map(|x| &**x),
//         )
//     }

//     pub fn ray_intersect_aabb(&self, origin: &Vector3D, direction: &Vector3D) -> bool {
//         let epsilon: f64 = 1e-6;
//         let (min_corner, max_corner) = self.aabb;
//         let inv_direction = Vector3D::new(
//             1.0 / (direction.x + epsilon),
//             1.0 / (direction.y + epsilon),
//             1.0 / (direction.z + epsilon),
//         );

//         let t1 = (min_corner.x - origin.x) * inv_direction.x;
//         let t2 = (max_corner.x - origin.x) * inv_direction.x;
//         let t3 = (min_corner.y - origin.y) * inv_direction.y;
//         let t4 = (max_corner.y - origin.y) * inv_direction.y;
//         let t5 = (min_corner.z - origin.z) * inv_direction.z;
//         let t6 = (max_corner.z - origin.z) * inv_direction.z;

//         let t_min = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
//         let t_max = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

//         t_max > t_min.max(0.0)
//     }

//     pub fn traverse(&self, origin: &Vector3D, direction: &Vector3D) -> Vec<Polygon> {
//         if !self.ray_intersect_aabb(origin, direction) {
//             return vec![];
//         }

//         if self.left.is_none() && self.right.is_none() {
//             return self.polygons.clone();
//         }

//         let mut intersecting_polygons = vec![];

//         if let Some(left_node) = &self.left {
//             intersecting_polygons.extend(left_node.traverse(origin, direction));
//         }

//         if let Some(right_node) = &self.right {
//             intersecting_polygons.extend(right_node.traverse(origin, direction));
//         }

//         intersecting_polygons
//     }
// }
