use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use std::cmp::Ordering::Equal;

pub struct ZBufferSort;

impl ZBufferSort {
    pub fn new() -> Self {
        Self {}
    }

    fn get_centroid_distance(&self, polygon: &Polygon, camera_position: &Vector3D) -> f64 {
        let centroid: Vector3D = polygon.get_centroid();
        let distance: f64 = camera_position.get_distance(&centroid);
        distance
    }

    pub fn sort_polygons(&self, polygons: &mut Vec<Polygon>, camera_position: Vector3D) {
        polygons.sort_unstable_by(|a, b| {
            let dist_a = self.get_centroid_distance(a, &camera_position);
            let dist_b = self.get_centroid_distance(b, &camera_position);

            dist_b.partial_cmp(&dist_a).unwrap_or(Equal)
        });
    }
}

pub struct PainterBufferSort;

impl PainterBufferSort {
    pub fn new() -> Self {
        Self {}
    }

    fn get_max_vertex_distance(&self, vertices: &[Vector3D], camera_position: &Vector3D) -> f64 {
        vertices
            .iter()
            .map(|vertex| camera_position.get_distance(vertex))
            .fold(f64::MIN, f64::max)
    }

    pub fn sort_polygons(&self, polygons: &mut Vec<Polygon>, camera_position: Vector3D) {
        polygons.sort_unstable_by(|a, b| {
            let dist_a = self.get_max_vertex_distance(&a.get_vertices(), &camera_position);
            let dist_b = self.get_max_vertex_distance(&b.get_vertices(), &camera_position);

            dist_b.partial_cmp(&dist_a).unwrap_or(Equal)
        });
    }
}

pub struct BSPNode {
    pub polygon: Option<Polygon>,
    pub front: Option<Box<BSPNode>>,
    pub back: Option<Box<BSPNode>>,
}

impl BSPNode {
    pub fn new(polygon: Option<Polygon>) -> Self {
        Self {
            polygon,
            front: None,
            back: None,
        }
    }

    pub fn insert(&mut self, polygon: Polygon) {
        match &self.polygon {
            None => self.polygon = Some(polygon),
            Some(current_polygon) => {
                let (front, back) = current_polygon.split(polygon);

                if let Some(front_polygon) = front {
                    if let Some(front_child) = &mut self.front {
                        front_child.insert(Polygon::Triangle(front_polygon));
                    } else {
                        self.front = Some(Box::new(BSPNode::new(Some(Polygon::Triangle(
                            front_polygon,
                        )))));
                    }
                }

                if let Some(back_polygon) = back {
                    if let Some(back_child) = &mut self.back {
                        back_child.insert(Polygon::Triangle(back_polygon));
                    } else {
                        self.back = Some(Box::new(BSPNode::new(Some(Polygon::Triangle(
                            back_polygon,
                        )))));
                    }
                }
            }
        }
    }

    pub fn traverse(&self, camera_position: &Vector3D) -> Vec<Polygon> {
        let mut polygons = Vec::new();

        let current_polygon = match &self.polygon {
            Some(p) => p,
            None => return polygons,
        };

        let dot_product = current_polygon.plane().0.dot_product(camera_position);

        if dot_product < 0.0 {
            if let Some(front_child) = &self.front {
                polygons.extend(front_child.traverse(camera_position));
            }
            polygons.push(current_polygon.clone());
            if let Some(back_child) = &self.back {
                polygons.extend(back_child.traverse(camera_position));
            }
        } else {
            if let Some(back_child) = &self.back {
                polygons.extend(back_child.traverse(camera_position));
            }
            polygons.push(current_polygon.clone());
            if let Some(front_child) = &self.front {
                polygons.extend(front_child.traverse(camera_position));
            }
        }

        polygons
    }
}
