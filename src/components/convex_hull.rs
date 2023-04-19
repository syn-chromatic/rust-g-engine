use crate::components::vectors::Vector3D;
use std::cmp::Ordering;

use std::f64::EPSILON;

pub struct QuickHull {
    vertices: Vec<Vector3D>,
}

impl QuickHull {
    pub fn new(vertices: Vec<Vector3D>) -> QuickHull {
        QuickHull { vertices }
    }

    fn get_left_set_a(
        &self,
        vertices: &Vec<Vector3D>,
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
        &self,
        vertices: &Vec<Vector3D>,
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

    fn recursive_hull(
        &self,
        vertices: &Vec<Vector3D>,
        p1: &Vector3D,
        p2: &Vector3D,
        hull: &mut Vec<Vector3D>,
    ) {
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

        let left_set_a: Vec<Vector3D> = self.get_left_set_a(vertices, farthest_vertex, p1);
        self.recursive_hull(&left_set_a, p1, farthest_vertex, hull);
        let left_set_b: Vec<Vector3D> = self.get_left_set_b(vertices, farthest_vertex, p2);
        self.recursive_hull(&left_set_b, farthest_vertex, p2, hull);
    }

    pub fn get_hull(&self) -> Vec<Vector3D> {
        let mut hull: Vec<Vector3D> = Vec::new();
        let vertices = &self.vertices;
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
        self.recursive_hull(&left_set, min_vertex, max_vertex, &mut hull);
        hull.push(*max_vertex);
        self.recursive_hull(&right_set, max_vertex, min_vertex, &mut hull);
        hull
    }
}

pub struct ChansHull {
    vertices: Vec<Vector3D>,
    m: usize,
}

impl ChansHull {
    pub fn new(vertices: Vec<Vector3D>, m: usize) -> ChansHull {
        ChansHull { vertices, m }
    }

    fn get_leftmost_vertex(&self, vertices: &Vec<Vector3D>) -> Vector3D {
        let leftmost_vertex: Vector3D = *vertices
            .iter()
            .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
            .unwrap();
        leftmost_vertex
    }

    fn gift_wrapping(&self, vertices: &Vec<Vector3D>) -> Vec<Vector3D> {
        let mut hull: Vec<Vector3D> = Vec::new();

        let leftmost_vertex: Vector3D = self.get_leftmost_vertex(vertices);

        let mut p: Vector3D = leftmost_vertex;
        let mut q: Vector3D;

        loop {
            hull.push(p);
            q = *vertices.iter().find(|&&v| v != p).unwrap_or(&p);

            for r in vertices {
                if r == &p {
                    continue;
                }
                let orientation: f64 = (q.subtract_vector(&p))
                    .cross_product(&(r.subtract_vector(&p)))
                    .z;

                if orientation > 0.0 {
                    q = *r;
                } else if orientation == 0.0 {
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

    fn get_anchor(&self, vertices: &Vec<Vector3D>) -> Vector3D {
        let anchor = vertices
            .iter()
            .min_by(|a, b| {
                a.y.partial_cmp(&b.y)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
            })
            .cloned()
            .unwrap();
        anchor
    }

    fn sort_vertices(&self, vertices: &mut Vec<Vector3D>, anchor: Vector3D) {
        vertices.sort_unstable_by(|a, b| {
            let angle_a = (a.y - anchor.y).atan2(a.x - anchor.x);
            let angle_b = (b.y - anchor.y).atan2(b.x - anchor.x);
            let angle_cmp = angle_a.partial_cmp(&angle_b).unwrap_or(Ordering::Equal);
            if angle_cmp == Ordering::Equal {
                a.get_distance(&anchor)
                    .partial_cmp(&b.get_distance(&anchor))
                    .unwrap_or(Ordering::Equal)
            } else {
                angle_cmp
            }
        });
    }

    fn graham_scan(&self, vertices: &mut Vec<Vector3D>) -> Vec<Vector3D> {
        let anchor = self.get_anchor(vertices);
        self.sort_vertices(vertices, anchor);

        let mut hull: Vec<Vector3D> = Vec::new();
        for &vertex in vertices.iter() {
            while hull.len() >= 2 {
                let a: Vector3D = hull[hull.len() - 2];
                let b: Vector3D = hull[hull.len() - 1];
                let ab: Vector3D = b.subtract_vector(&a);
                let ac: Vector3D = vertex.subtract_vector(&a);
                // let normal: Vector3D = ab.cross_product(&ac);
                let normal_z: f64 = ab.cross_product(&ac).z;
                if normal_z >= 0.0 {
                    break;
                }
                hull.pop();
            }
            hull.push(vertex);
        }
        hull
    }

    fn merge_hulls(&self, hull1: &Vec<Vector3D>, hull2: &Vec<Vector3D>) -> Vec<Vector3D> {
        let mut merged_hull: Vec<Vector3D> = hull1.clone();
        merged_hull.extend(hull2);
        self.graham_scan(&mut merged_hull)
    }

    pub fn get_hull(&self) -> Vec<Vector3D> {
        let n: usize = self.vertices.len();
        let mut hulls: Vec<Vec<Vector3D>> = Vec::new();

        for i in (0..n).step_by(self.m) {
            let sub_hull_vertices: Vec<Vector3D> =
                self.vertices[i..usize::min(i + self.m, n)].to_vec();
            let sub_hull: Vec<Vector3D> = self.gift_wrapping(&sub_hull_vertices);
            hulls.push(sub_hull);
        }

        while hulls.len() > 1 {
            let mut new_hulls: Vec<Vec<Vector3D>> = Vec::new();
            for hulls_chunk in hulls.chunks(2) {
                match hulls_chunk.len() {
                    1 => new_hulls.push(hulls_chunk[0].clone()),
                    2 => new_hulls.push(self.merge_hulls(&hulls_chunk[0], &hulls_chunk[1])),
                    _ => (),
                }
            }
            hulls = new_hulls;
        }

        hulls.into_iter().next().unwrap_or_else(Vec::new)
    }
}

// pub struct QuickHull {
//     vertices: Vec<Vector3D>,
// }

// impl QuickHull {
//     pub fn new(vertices: Vec<Vector3D>) -> QuickHull {
//         QuickHull { vertices }
//     }

//     fn area_of_triangle(&self, a: &Vector3D, b: &Vector3D, c: &Vector3D) -> f64 {
//         let ab: Vector3D = b.subtract_vector(a);
//         let ac: Vector3D = c.subtract_vector(a);
//         let cross_product: Vector3D = ab.cross_product(&ac);
//         let area: f64 = cross_product.get_length() * 0.5;
//         area
//     }

//     fn tetrahedron_volume(&self, a: &Vector3D, b: &Vector3D, c: &Vector3D, d: &Vector3D) -> f64 {
//         let ab: Vector3D = b.subtract_vector(a);
//         let ac: Vector3D = c.subtract_vector(a);
//         let ad: Vector3D = d.subtract_vector(a);
//         let cross_product: Vector3D = ab.cross_product(&ac);
//         let volume: f64 = cross_product.dot_product(&ad).abs() / 6.0;
//         volume
//     }

//     fn get_extreme_vertices(&self) -> (Vector3D, Vector3D) {
//         let min_x: Vector3D = self
//             .vertices
//             .iter()
//             .cloned()
//             .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
//             .unwrap();
//         let max_x: Vector3D = self
//             .vertices
//             .iter()
//             .cloned()
//             .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal))
//             .unwrap();
//         (min_x, max_x)
//     }

//     fn initial_tetrahedron(&self) -> (Vector3D, Vector3D, Vector3D, Vector3D) {
//         let (min_x, max_x): (Vector3D, Vector3D) = self.get_extreme_vertices();
//         let mut max_distance: f64 = 0.0;
//         let mut initial_vertex_a: Vector3D = min_x;
//         let initial_vertex_b: Vector3D = max_x;

//         for vertex in self.vertices.iter() {
//             let distance =
//                 vertex.get_distance(&initial_vertex_a) + vertex.get_distance(&initial_vertex_b);
//             if distance > max_distance {
//                 max_distance = distance;
//                 initial_vertex_a = *vertex;
//             }
//         }

//         let mut max_area: f64 = 0.0;
//         let mut initial_vertex_c: Vector3D = min_x;
//         for vertex in self.vertices.iter() {
//             let area: f64 = self.area_of_triangle(&initial_vertex_a, &initial_vertex_b, vertex);
//             if area > max_area {
//                 max_area = area;
//                 initial_vertex_c = *vertex;
//             }
//         }

//         let mut max_volume: f64 = 0.0;
//         let mut initial_vertex_d: Vector3D = min_x;
//         for vertex in self.vertices.iter() {
//             let volume: f64 = self.tetrahedron_volume(
//                 &initial_vertex_a,
//                 &initial_vertex_b,
//                 &initial_vertex_c,
//                 vertex,
//             );
//             if volume > max_volume {
//                 max_volume = volume;
//                 initial_vertex_d = *vertex;
//             }
//         }

//         (
//             initial_vertex_a,
//             initial_vertex_b,
//             initial_vertex_c,
//             initial_vertex_d,
//         )
//     }

//     fn find_furthest_point_on_mesh(
//         &self,
//         plane_normal: &Vector3D,
//         plane_point: &Vector3D,
//     ) -> Vector3D {
//         let mut max_distance: f64 = 0.0;
//         let mut furthest_point: Vector3D = plane_point.clone();
//         for vertex in self.vertices.iter() {
//             let distance: f64 = plane_normal.dot_product(&(vertex.subtract_vector(plane_point)));
//             if distance > max_distance {
//                 max_distance = distance;
//                 furthest_point = *vertex;
//             }
//         }
//         furthest_point
//     }
//     fn quick_hull_recursive(
//         &self,
//         vertices: &HashSet<Vector3D>,
//         plane_normal: &Vector3D,
//         plane_point: &Vector3D,
//     ) -> Vec<Vector3D> {
//         if vertices.is_empty() {
//             return vec![];
//         }

//         let furthest_point: Vector3D = self.find_furthest_point_on_mesh(plane_normal, plane_point);
//         let mut new_vertices: HashSet<Vector3D> = vertices.clone();
//         new_vertices.remove(&furthest_point);

//         let mut new_hull = vec![furthest_point];
//         for i in 0..new_hull.len() {
//             let a: Vector3D = new_hull[i];
//             for j in (i + 1)..new_hull.len() {
//                 let b: Vector3D = new_hull[j];
//                 let ab: Vector3D = b.subtract_vector(&a);
//                 let mut max_distance: f64 = 0.0;
//                 let mut furthest_vertex: Vector3D = a;
//                 for vertex in &new_vertices {
//                     let distance: f64 =
//                         ab.cross_product(&(vertex.subtract_vector(&a))).get_length();
//                     if distance > max_distance {
//                         max_distance = distance;
//                         furthest_vertex = *vertex;
//                     }
//                 }
//                 if furthest_vertex != a {
//                     let face_normal: Vector3D = ab
//                         .cross_product(&(furthest_vertex.subtract_vector(&a)))
//                         .normalize();
//                     let face_vertices: HashSet<Vector3D> = new_vertices
//                         .iter()
//                         .cloned()
//                         .filter(|vertex| {
//                             face_normal.dot_product(&(vertex.subtract_vector(&a))) >= 0.0
//                         })
//                         .collect::<HashSet<_>>();
//                     let sub_hull: Vec<Vector3D> =
//                         self.quick_hull_recursive(&face_vertices, &face_normal, &a);
//                     new_hull.extend(sub_hull);
//                     new_vertices = new_vertices.difference(&face_vertices).cloned().collect();
//                 }
//             }
//         }

//         new_hull
//     }

//     pub fn get_hull(&self) -> Vec<Vector3D> {
//         let unique_vertices: HashSet<Vector3D> = self.vertices.iter().cloned().collect();
//         let n: usize = unique_vertices.len();
//         if n <= 4 {
//             return unique_vertices.into_iter().collect();
//         }

//         let (a, b, c, d) = self.initial_tetrahedron();
//         let mut vertices: HashSet<Vector3D> = unique_vertices.clone();
//         vertices.remove(&a);
//         vertices.remove(&b);
//         vertices.remove(&c);
//         vertices.remove(&d);

//         let mut hull = vec![a, b, c, d];
//         for i in 0..hull.len() {
//             let a: Vector3D = hull[i];
//             for j in (i + 1)..hull.len() {
//                 let b: Vector3D = hull[j];
//                 let ab: Vector3D = b.subtract_vector(&a);
//                 let mut max_distance: f64 = 0.0;
//                 let mut furthest_vertex = a;
//                 for vertex in &vertices {
//                     let distance: f64 =
//                         ab.cross_product(&(vertex.subtract_vector(&a))).get_length();
//                     if distance > max_distance {
//                         max_distance = distance;
//                         furthest_vertex = *vertex;
//                     }
//                 }
//                 if furthest_vertex != a {
//                     let face_normal = ab
//                         .cross_product(&(furthest_vertex.subtract_vector(&a)))
//                         .normalize();
//                     let face_vertices = vertices
//                         .iter()
//                         .cloned()
//                         .filter(|vertex| {
//                             face_normal.dot_product(&(vertex.subtract_vector(&a))) >= 0.0
//                         })
//                         .collect::<HashSet<_>>();
//                     let sub_hull = self.quick_hull_recursive(&face_vertices, &face_normal, &a);
//                     hull.extend(sub_hull);
//                     vertices = vertices.difference(&face_vertices).cloned().collect();
//                 }
//             }
//         }

//         hull
//     }
// }
