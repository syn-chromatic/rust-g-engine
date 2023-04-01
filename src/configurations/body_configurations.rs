use crate::abstracts::body::BodyType;
use crate::components::model::OBJModelFormat;
use crate::components::shape::Shape;
use crate::components::vertices::GridHorizontal;
use crate::components::vertices::Sphere;

pub fn get_grid() -> BodyType {
    let mut grid = GridHorizontal::new(50, 50, 200.0);
    grid.set_offset(-1000.0, -100.0, -1000.0);
    let mesh = grid.get_triangle_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_obj(file_path: &str) -> BodyType {
    let mut obj = OBJModelFormat::new(file_path, 1.0);
    obj.set_offset(2000.0, -100.0, 4000.0);
    // obj.set_rotation(-90.0, 0.0, 0.0);

    let mesh = obj.get_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere() -> BodyType {
    let mut sphere = Sphere::new(50.0, 20, 20);
    sphere.set_offset(4900.0, 20100.0, 7500.0);

    let mesh = sphere.get_triangle_mesh();
    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}
