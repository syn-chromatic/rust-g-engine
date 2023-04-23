use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BVHNode {
    pub polygons: Vec<Polygon>,
    pub vertices: Vec<Vector3D>,
    pub face_normals: Vec<Vector3D>,
    left: Option<Arc<BVHNode>>,
    right: Option<Arc<BVHNode>>,
    aabb: ([f64; 3], [f64; 3]),
    pub max_leaf_size: usize,
}

impl BVHNode {
    pub fn new(polygons: &[Polygon], vertices: &[Vector3D]) -> Self {
        let aabb: ([f64; 3], [f64; 3]) = Self::get_aabb(polygons);
        let polygons: Vec<Polygon> = polygons.to_vec();
        let face_normals: Vec<Vector3D> = polygons.iter().map(|p| p.get_normal()).collect();

        let max_leaf_size: usize = vertices.len();
        let vertices: Vec<Vector3D> = vertices.to_vec();

        let mut bvh_node: BVHNode = BVHNode {
            polygons,
            vertices,
            face_normals,
            left: None,
            right: None,
            aabb,
            max_leaf_size,
        };
        bvh_node.split();
        bvh_node
    }

    pub fn project_onto_axis(&self, axis: &Vector3D) -> (f64, f64) {
        let mut min: f64 = f64::MAX;
        let mut max: f64 = f64::MIN;

        let axis: &Vector3D = &axis.normalize();
        for vertex in &self.vertices {
            let dot: f64 = vertex.dot_product(axis);
            min = min.min(dot);
            max = max.max(dot);
        }

        (min, max)
    }

    fn aabb_intersects(&self, other: &BVHNode) -> bool {
        let (min_a, max_a): ([f64; 3], [f64; 3]) = self.aabb;
        let (min_b, max_b): ([f64; 3], [f64; 3]) = other.aabb;

        for i in 0..3 {
            if min_a[i] > max_b[i] || min_b[i] > max_a[i] {
                return false;
            }
        }
        true
    }

    fn sat_intersection(&self, other: &BVHNode) -> Option<Vector3D> {
        let mut mtv: Option<Vector3D> = None;
        let mut min_overlap: f64 = f64::MAX;
        let epsilon: f64 = f64::EPSILON;

        for face_list in [&self.face_normals, &other.face_normals] {
            for face_normal in face_list {
                let mut axis: Vector3D = *face_normal;
                if axis.get_length() < epsilon {
                    continue;
                }
                let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
                let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

                if max_a < min_b || max_b < min_a {
                    return None;
                }

                let overlap: f64 = (max_a - min_b).min(max_b - min_a);
                if overlap < min_overlap && overlap > epsilon {
                    min_overlap = overlap;
                    let center_a: Vector3D = self.get_center();
                    let center_b: Vector3D = other.get_center();
                    let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
                    if direction.dot_product(&axis) < 0.0 {
                        axis = axis.multiply(-1.0);
                    }
                    mtv = Some(axis.multiply(min_overlap));
                }
            }
        }

        if mtv.is_none() {
            return None;
        }

        let edges_a: Vec<Vector3D> = self.get_edges();
        let edges_b: Vec<Vector3D> = other.get_edges();

        for edge_list in [&edges_a, &edges_b] {
            for edge in edge_list {
                let edge: Vector3D = *edge;
                let mtv_unwrap: Vector3D = mtv.unwrap();
                let mut axis: Vector3D = edge.cross_product(&mtv_unwrap).normalize();

                if axis.get_length() < epsilon {
                    continue;
                }

                let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
                let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

                if max_a < min_b || max_b < min_a {
                    return None;
                }

                let overlap: f64 = (max_a - min_b).min(max_b - min_a);
                if overlap <= min_overlap && overlap > epsilon {
                    min_overlap = overlap;
                    let center_a: Vector3D = self.get_center();
                    let center_b: Vector3D = other.get_center();
                    let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
                    if direction.dot_product(&axis) < 0.0 {
                        axis = axis.multiply(-1.0);
                    }
                    mtv = Some(axis.multiply(min_overlap));
                }
            }
        }

        mtv
    }

    pub fn is_intersecting(&mut self, other: &mut BVHNode) -> Option<Vector3D> {
        self.sat_intersection(other)
    }

    pub fn get_center(&self) -> Vector3D {
        let (min_corner, max_corner) = self.aabb;
        Vector3D::new(
            (min_corner[0] + max_corner[0]) / 2.0,
            (min_corner[1] + max_corner[1]) / 2.0,
            (min_corner[2] + max_corner[2]) / 2.0,
        )
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
            self.face_normals = self.polygons.iter().map(|p| p.get_normal()).collect();
            return;
        }

        let mut best_cost: f64 = f64::MAX;
        let mut best_axis: usize = 0;
        let mut best_split_position: usize = 0;

        for axis in 0..3 {
            self.polygons.sort_unstable_by(|a, b| {
                let centroid_a: [f64; 3] = a.get_centroid().to_array();
                let centroid_b: [f64; 3] = b.get_centroid().to_array();
                centroid_a[axis]
                    .partial_cmp(&centroid_b[axis])
                    .unwrap_or(Ordering::Equal)
            });

            let mut left_aabb: ([f64; 3], [f64; 3]) = BVHNode::empty_aabb();
            let mut right_aabb: ([f64; 3], [f64; 3]) = BVHNode::get_aabb(&self.polygons);

            for i in 0..(self.polygons.len() - 1) {
                let polygon = &self.polygons[i];
                left_aabb = BVHNode::expand_aabb(left_aabb, Self::get_polygon_aabb(polygon));
                right_aabb = BVHNode::shrink_aabb(right_aabb, Self::get_polygon_aabb(polygon));

                let cost = BVHNode::surface_area(left_aabb) * (i + 1) as f64
                    + BVHNode::surface_area(right_aabb) * (self.polygons.len() - i - 1) as f64;

                if cost < best_cost {
                    best_cost = cost;
                    best_axis = axis;
                    best_split_position = i;
                }
            }
        }

        self.polygons.sort_unstable_by(|a, b| {
            let centroid_a = a.get_centroid().to_array();
            let centroid_b = b.get_centroid().to_array();
            centroid_a[best_axis]
                .partial_cmp(&centroid_b[best_axis])
                .unwrap_or(Ordering::Equal)
        });

        let (left_polygons, right_polygons) = self.polygons.split_at(best_split_position + 1);

        self.left = Some(Arc::new(BVHNode::new(
            &left_polygons.to_vec(),
            &self.vertices,
        )));
        self.right = Some(Arc::new(BVHNode::new(
            &right_polygons.to_vec(),
            &self.vertices,
        )));
    }

    fn get_polygon_aabb(polygon: &Polygon) -> ([f64; 3], [f64; 3]) {
        let vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };

        let mut min_corner: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max_corner: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];

        for vertex in vertices {
            min_corner[0] = min_corner[0].min(vertex.x);
            min_corner[1] = min_corner[1].min(vertex.y);
            min_corner[2] = min_corner[2].min(vertex.z);
            max_corner[0] = max_corner[0].max(vertex.x);
            max_corner[1] = max_corner[1].max(vertex.y);
            max_corner[2] = max_corner[2].max(vertex.z);
        }

        (min_corner, max_corner)
    }

    fn get_aabb(polygons: &[Polygon]) -> ([f64; 3], [f64; 3]) {
        let mut min_corner: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max_corner: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];

        for polygon in polygons {
            let polygon_aabb = Self::get_polygon_aabb(polygon);
            min_corner = [
                min_corner[0].min(polygon_aabb.0[0]),
                min_corner[1].min(polygon_aabb.0[1]),
                min_corner[2].min(polygon_aabb.0[2]),
            ];
            max_corner = [
                max_corner[0].max(polygon_aabb.1[0]),
                max_corner[1].max(polygon_aabb.1[1]),
                max_corner[2].max(polygon_aabb.1[2]),
            ];
        }
        (min_corner, max_corner)
    }

    fn empty_aabb() -> ([f64; 3], [f64; 3]) {
        (
            [f64::MAX, f64::MAX, f64::MAX],
            [f64::MIN, f64::MIN, f64::MIN],
        )
    }

    fn expand_aabb(
        aabb: ([f64; 3], [f64; 3]),
        new_aabb: ([f64; 3], [f64; 3]),
    ) -> ([f64; 3], [f64; 3]) {
        let (mut min_corner, mut max_corner): ([f64; 3], [f64; 3]) = aabb;
        let (new_min_corner, new_max_corner): ([f64; 3], [f64; 3]) = new_aabb;

        for i in 0..3 {
            min_corner[i] = min_corner[i].min(new_min_corner[i]);
            max_corner[i] = max_corner[i].max(new_max_corner[i]);
        }

        (min_corner, max_corner)
    }

    fn shrink_aabb(
        aabb: ([f64; 3], [f64; 3]),
        removed_aabb: ([f64; 3], [f64; 3]),
    ) -> ([f64; 3], [f64; 3]) {
        let (mut min_corner, mut max_corner): ([f64; 3], [f64; 3]) = aabb;
        let (removed_min_corner, removed_max_corner): ([f64; 3], [f64; 3]) = removed_aabb;

        for i in 0..3 {
            if min_corner[i] == removed_min_corner[i] {
                min_corner[i] = f64::MAX;
            }
            if max_corner[i] == removed_max_corner[i] {
                max_corner[i] = f64::MIN;
            }
        }

        (min_corner, max_corner)
    }

    fn surface_area(aabb: ([f64; 3], [f64; 3])) -> f64 {
        let (min_corner, max_corner): ([f64; 3], [f64; 3]) = aabb;
        let width: f64 = max_corner[0] - min_corner[0];
        let height: f64 = max_corner[1] - min_corner[1];
        let depth: f64 = max_corner[2] - min_corner[2];
        2.0 * (width * height + width * depth + height * depth)
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

    pub fn traverse_and_collide_sat(&self, other: &Self) -> Option<Vector3D> {
        let mut stack_a: Vec<&BVHNode> = Vec::new();
        let mut stack_b: Vec<&BVHNode> = Vec::new();
        stack_a.push(self);
        stack_b.push(other);

        while let (Some(node_a), Some(node_b)) = (stack_a.pop(), stack_b.pop()) {
            if !node_a.aabb_intersects(node_b) {
                continue;
            }

            match (&node_a.left, &node_a.right, &node_b.left, &node_b.right) {
                (None, None, None, None) => {
                    if let Some(mtv) = node_a.sat_intersection(node_b) {
                        return Some(mtv);
                    }
                }
                (Some(left_a), Some(right_a), None, None) => {
                    stack_a.push(left_a);
                    stack_a.push(right_a);
                    stack_b.push(node_b);
                    stack_b.push(node_b);
                }
                (None, None, Some(left_b), Some(right_b)) => {
                    stack_a.push(node_a);
                    stack_a.push(node_a);
                    stack_b.push(left_b);
                    stack_b.push(right_b);
                }
                _ => {
                    if let Some(left_a) = &node_a.left {
                        stack_a.push(left_a);
                    }
                    if let Some(right_a) = &node_a.right {
                        stack_a.push(right_a);
                    }
                    if let Some(left_b) = &node_b.left {
                        stack_b.push(left_b);
                    }
                    if let Some(right_b) = &node_b.right {
                        stack_b.push(right_b);
                    }
                }
            }
        }

        None
    }

    pub fn traverse_and_collide(&self, other: &Self) -> bool {
        let mut stack_a: Vec<&BVHNode> = Vec::new();
        let mut stack_b: Vec<&BVHNode> = Vec::new();
        stack_a.push(self);
        stack_b.push(other);

        while let (Some(node_a), Some(node_b)) = (stack_a.pop(), stack_b.pop()) {
            if !node_a.aabb_intersects(node_b) {
                continue;
            }

            match (&node_a.left, &node_a.right, &node_b.left, &node_b.right) {
                (None, None, None, None) => {
                    return true;
                }
                (Some(left_a), Some(right_a), None, None) => {
                    stack_a.push(left_a);
                    stack_a.push(right_a);
                    stack_b.push(node_b);
                    stack_b.push(node_b);
                }
                (None, None, Some(left_b), Some(right_b)) => {
                    stack_a.push(node_a);
                    stack_a.push(node_a);
                    stack_b.push(left_b);
                    stack_b.push(right_b);
                }
                _ => {
                    if let Some(left_a) = &node_a.left {
                        stack_a.push(left_a);
                    }
                    if let Some(right_a) = &node_a.right {
                        stack_a.push(right_a);
                    }
                    if let Some(left_b) = &node_b.left {
                        stack_b.push(left_b);
                    }
                    if let Some(right_b) = &node_b.right {
                        stack_b.push(right_b);
                    }
                }
            }
        }

        false
    }

    pub fn get_aabb_points(&self) -> Vec<Vector3D> {
        let (min, max): &([f64; 3], [f64; 3]) = &self.aabb;
        let min: Vector3D = Vector3D::new(min[0], min[1], min[2]);
        let max: Vector3D = Vector3D::new(max[0], max[1], max[2]);

        vec![
            min,
            Vector3D::new(min.x, min.y, max.z),
            Vector3D::new(min.x, max.y, min.z),
            Vector3D::new(min.x, max.y, max.z),
            Vector3D::new(max.x, min.y, min.z),
            Vector3D::new(max.x, min.y, max.z),
            Vector3D::new(max.x, max.y, min.z),
            max,
        ]
    }
}
