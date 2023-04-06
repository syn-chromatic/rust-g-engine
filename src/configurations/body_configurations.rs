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
    obj.set_rotation(-90.0, 0.0, 0.0);
    // obj.set_rotation(0.0, -110.0, 0.0);
    // obj.set_offset(800.0, 800.0, 0.0);

    let mesh = obj.get_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere() -> BodyType {
    let mut sphere = Sphere::new(500.0, 50, 50);
    let offset = 1000.0;
    sphere.set_offset(50.0 + offset, 5_000.0 + offset, 10000.0 + offset);

    let mesh = sphere.get_triangle_mesh();
    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}
