use crate::components::vectors::Vector3D;
use std::cmp::Ordering;
use std::f64::EPSILON;

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
