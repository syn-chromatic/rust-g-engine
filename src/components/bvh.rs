use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct BVHNode {
    pub polygons: Vec<Polygon>,
    pub vertices: Vec<Vector3D>,
    pub face_normals: Vec<Vector3D>,
    aabb: ([f64; 3], [f64; 3]),
}

impl BVHNode {
    pub fn new(polygons: &[Polygon], vertices: &[Vector3D]) -> Self {
        let aabb: ([f64; 3], [f64; 3]) = Self::get_aabb(polygons);
        let polygons: Vec<Polygon> = polygons.to_vec();
        let vertices: Vec<Vector3D> = vertices.to_vec();
        let face_normals: Vec<Vector3D> = polygons.iter().map(|p| p.get_normal()).collect();

        BVHNode {
            polygons,
            vertices,
            face_normals,
            aabb,
        }
    }

    pub fn translate_bvh(&mut self, translation: &Vector3D) {
        for polygon in &mut self.polygons {
            polygon.translate(&translation);
        }

        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.add_vector(&translation);
        }

        let left_aabb_v3d: Vector3D = Vector3D::from_array(self.aabb.0);
        let left_aabb_v3d: Vector3D = left_aabb_v3d.add_vector(&translation);

        let right_aabb_v3d: Vector3D = Vector3D::from_array(self.aabb.1);
        let right_aabb_v3d: Vector3D = right_aabb_v3d.add_vector(&translation);

        let left_aabb: [f64; 3] = left_aabb_v3d.to_array();
        let right_aabb: [f64; 3] = right_aabb_v3d.to_array();

        self.aabb = (left_aabb, right_aabb);
    }

    pub fn rotate_bvh(&mut self, axis: &Vector3D, centroid: &Vector3D, angle: f64) {
        let translation_to_origin = centroid.multiply(-1.0);

        for polygon in &mut self.polygons {
            polygon.translate(&translation_to_origin);
            polygon.rotate(&axis, angle);
            polygon.translate(&centroid);
        }

        for vertex in self.vertices.iter_mut() {
            *vertex = vertex.add_vector(&translation_to_origin);
            *vertex = vertex.rotate_around_axis(axis, angle);
            *vertex = vertex.add_vector(&centroid);
        }

        let mut min: Vector3D = self.vertices[0];
        let mut max: Vector3D = self.vertices[0];
        for vertex in self.vertices.iter() {
            min = min.component_min(vertex);
            max = max.component_max(vertex);
        }
        self.aabb = (min.to_array(), max.to_array());
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

    pub fn aabb_intersects(&self, other: &BVHNode) -> bool {
        let (min_a, max_a): ([f64; 3], [f64; 3]) = self.aabb;
        let (min_b, max_b): ([f64; 3], [f64; 3]) = other.aabb;

        for i in 0..3 {
            if min_a[i] > max_b[i] || min_b[i] > max_a[i] {
                return false;
            }
        }
        true
    }

    // fn sat_intersection(&self, other: &BVHNode) -> Option<Vector3D> {
    //     let mut mtv: Option<Vector3D> = None;
    //     let mut min_overlap: f64 = f64::MAX;
    //     let epsilon: f64 = f64::EPSILON;

    //     for face_list in [&self.face_normals, &other.face_normals] {
    //         for face_normal in face_list {
    //             let mut axis: Vector3D = *face_normal;
    //             if axis.get_length() < epsilon {
    //                 continue;
    //             }
    //             let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
    //             let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

    //             if max_a < min_b || max_b < min_a {
    //                 return None;
    //             }

    //             let overlap: f64 = (max_a - min_b).min(max_b - min_a);
    //             if overlap < min_overlap && overlap > epsilon {
    //                 min_overlap = overlap;
    //                 let center_a: Vector3D = self.get_center();
    //                 let center_b: Vector3D = other.get_center();
    //                 let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
    //                 if direction.dot_product(&axis) < 0.0 {
    //                     axis = axis.multiply(-1.0);
    //                 }
    //                 mtv = Some(axis.multiply(min_overlap));
    //             }
    //         }
    //     }

    //     if mtv.is_none() {
    //         return None;
    //     }

    //     let edges_a: Vec<Vector3D> = self.get_edges();
    //     let edges_b: Vec<Vector3D> = other.get_edges();

    //     for edge_list in [&edges_a, &edges_b] {
    //         for edge in edge_list {
    //             let edge: Vector3D = *edge;
    //             let mtv_unwrap: Vector3D = mtv.unwrap();
    //             let mut axis: Vector3D = edge.cross_product(&mtv_unwrap).normalize();

    //             if axis.get_length() < epsilon {
    //                 continue;
    //             }

    //             let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
    //             let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

    //             if max_a < min_b || max_b < min_a {
    //                 return None;
    //             }

    //             let overlap: f64 = (max_a - min_b).min(max_b - min_a);
    //             if overlap <= min_overlap && overlap > epsilon {
    //                 min_overlap = overlap;
    //                 let center_a: Vector3D = self.get_center();
    //                 let center_b: Vector3D = other.get_center();
    //                 let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
    //                 if direction.dot_product(&axis) < 0.0 {
    //                     axis = axis.multiply(-1.0);
    //                 }
    //                 mtv = Some(axis.multiply(min_overlap));
    //             }
    //         }
    //     }

    //     mtv
    // }

    // fn sat_intersection(&self, other: &BVHNode) -> Option<(Vector3D, Vector3D)> {
    //     let mut mtv: Option<Vector3D> = None;
    //     let mut contact_point: Option<Vector3D> = None;
    //     let mut min_overlap: f64 = f64::MAX;
    //     let epsilon: f64 = f64::EPSILON;

    //     for face_list in [&self.face_normals, &other.face_normals] {
    //         for face_normal in face_list {
    //             let mut axis: Vector3D = *face_normal;
    //             if axis.get_length() < epsilon {
    //                 continue;
    //             }
    //             let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
    //             let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

    //             if max_a < min_b || max_b < min_a {
    //                 return None;
    //             }

    //             let overlap: f64 = (max_a - min_b).min(max_b - min_a);
    //             if overlap < min_overlap && overlap > epsilon {
    //                 min_overlap = overlap;
    //                 let center_a: Vector3D = self.get_center();
    //                 let center_b: Vector3D = other.get_center();
    //                 let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
    //                 if direction.dot_product(&axis) < 0.0 {
    //                     axis = axis.multiply(-1.0);
    //                 }
    //                 mtv = Some(axis.multiply(min_overlap));

    //                 let mut avg_contact: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    //                 let mut count: i32 = 0;
    //                 for vertex in &self.vertices {
    //                     let dist: f64 = vertex.dot_product(&axis);
    //                     if dist >= min_a && dist <= max_b {
    //                         avg_contact = avg_contact.add_vector(vertex);
    //                         count += 1;
    //                     }
    //                 }
    //                 for vertex in &other.vertices {
    //                     let dist: f64 = vertex.dot_product(&axis);
    //                     if dist <= max_a && dist >= min_b {
    //                         avg_contact = avg_contact.add_vector(vertex);
    //                         count += 1;
    //                     }
    //                 }
    //                 if count > 0 {
    //                     avg_contact = avg_contact.multiply(1.0 / count as f64);
    //                     contact_point = Some(avg_contact);
    //                 }
    //             }
    //         }
    //     }

    //     if mtv.is_none() || contact_point.is_none() {
    //         return None;
    //     }

    //     let edges_a: Vec<Vector3D> = self.get_edges();
    //     let edges_b: Vec<Vector3D> = other.get_edges();

    //     for edge_list in [&edges_a, &edges_b] {
    //         for edge in edge_list {
    //             let edge: Vector3D = *edge;
    //             let mtv_unwrap: Vector3D = mtv.unwrap();
    //             let mut axis: Vector3D = edge.cross_product(&mtv_unwrap).normalize();

    //             if axis.get_length() < epsilon {
    //                 continue;
    //             }

    //             let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
    //             let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

    //             if max_a < min_b || max_b < min_a {
    //                 return None;
    //             }

    //             let overlap: f64 = (max_a - min_b).min(max_b - min_a);
    //             if overlap <= min_overlap && overlap > epsilon {
    //                 min_overlap = overlap;
    //                 let center_a: Vector3D = self.get_center();
    //                 let center_b: Vector3D = other.get_center();
    //                 let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
    //                 if direction.dot_product(&axis) < 0.0 {
    //                     axis = axis.multiply(-1.0);
    //                 }
    //                 mtv = Some(axis.multiply(min_overlap));
    //             }
    //         }
    //     }

    //     Some((mtv.unwrap(), contact_point.unwrap()))
    // }

    fn sat_intersection(&self, other: &BVHNode) -> Option<(Vector3D, Vector3D)> {
        let mut mtv: Option<Vector3D> = None;
        let mut contact_point: Option<Vector3D> = None;
        let mut min_overlap: f64 = f64::MAX;
        let epsilon: f64 = f64::EPSILON;

        let face_normals: Vec<Vector3D> = self.polygons.iter().map(|p| p.get_normal()).collect();
        let other_face_normals: Vec<Vector3D> =
            other.polygons.iter().map(|p| p.get_normal()).collect();

        for face_list in [&face_normals, &other_face_normals] {
            for face_normal in face_list {
                let mut axis: Vector3D = *face_normal;
                if axis.get_length() <= epsilon {
                    continue;
                }
                let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
                let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

                if max_a < min_b || max_b < min_a {
                    return None;
                }

                let overlap: f64 = (max_a - min_b).min(max_b - min_a);
                if overlap < min_overlap {
                    min_overlap = overlap;
                    let center_a: Vector3D = self.get_center();
                    let center_b: Vector3D = other.get_center();
                    let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
                    if direction.dot_product(&axis) <= 0.0 {
                        axis = axis.multiply(-1.0);
                    }
                    mtv = Some(axis.multiply(min_overlap));

                    let mut avg_contact: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
                    let mut count: i32 = 0;
                    for vertex in &self.vertices {
                        let dist: f64 = vertex.dot_product(&axis);
                        if dist >= min_a && dist <= max_b {
                            avg_contact = avg_contact.add_vector(vertex);
                            count += 1;
                        }
                    }
                    for vertex in &other.vertices {
                        let dist: f64 = vertex.dot_product(&axis);
                        if dist <= max_a && dist >= min_b {
                            avg_contact = avg_contact.add_vector(vertex);
                            count += 1;
                        }
                    }
                    if count > 0 {
                        avg_contact = avg_contact.multiply(1.0 / count as f64);
                        contact_point = Some(avg_contact);
                    }
                }
            }
        }

        if mtv.is_none() || contact_point.is_none() {
            return None;
        }

        let edges_a: Vec<Vector3D> = self.get_edges();
        let edges_b: Vec<Vector3D> = other.get_edges();

        for edge_list in [&edges_a, &edges_b] {
            for edge in edge_list {
                let edge: Vector3D = *edge;
                let mtv_unwrap: Vector3D = mtv.unwrap();
                let mut axis: Vector3D = edge.cross_product(&mtv_unwrap).normalize();

                if axis.get_length() <= epsilon {
                    continue;
                }

                let (min_a, max_a): (f64, f64) = self.project_onto_axis(&axis);
                let (min_b, max_b): (f64, f64) = other.project_onto_axis(&axis);

                if max_a < min_b || max_b < min_a {
                    return None;
                }

                let overlap: f64 = (max_a - min_b).min(max_b - min_a);
                if overlap < min_overlap {
                    min_overlap = overlap;
                    let center_a: Vector3D = self.get_center();
                    let center_b: Vector3D = other.get_center();
                    let direction: Vector3D = center_b.subtract_vector(&center_a).normalize();
                    if direction.dot_product(&axis) <= 0.0 {
                        axis = axis.multiply(-1.0);
                    }
                    mtv = Some(axis.multiply(min_overlap));
                }
            }
        }

        let mut closest_vertices: Option<(Vector3D, Vector3D)> = None;
        let mut min_distance: f64 = f64::MAX;

        for vertex_a in &self.vertices {
            for vertex_b in &other.vertices {
                let distance: f64 = vertex_a.get_distance(vertex_b);
                if distance < min_distance {
                    min_distance = distance;
                    closest_vertices = Some((*vertex_a, *vertex_b));
                }
            }
        }

        if let Some((vertex_a, vertex_b)) = closest_vertices {
            contact_point = Some(vertex_a.add_vector(&vertex_b).multiply(0.5));
        }

        Some((mtv.unwrap(), contact_point.unwrap()))
    }

    pub fn is_intersecting(&self, other: &BVHNode) -> Option<(Vector3D, Vector3D)> {
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
        let mut intersecting_polygons: Vec<Polygon> = vec![];

        if self.ray_intersect_aabb(origin, direction) {
            intersecting_polygons.extend(self.polygons.clone());
        }

        intersecting_polygons
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
