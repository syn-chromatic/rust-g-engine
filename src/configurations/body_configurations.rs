use crate::body::BodyType;
use crate::model::OBJModelFormat;
use crate::shape::Shape;
use crate::vertices::GridHorizontal;
use crate::vertices::Sphere;

pub fn get_grid() -> BodyType {
    let mut grid = GridHorizontal::new(50, 50, 100.0);
    grid.set_offset(-1000.0, -100.0, -1000.0);
    let mesh = grid.get_triangle_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_obj() -> BodyType {
    let mut obj = OBJModelFormat::new("./tank_2.obj", 20.0);
    obj.set_offset(0.0, -100.0, 0.0);

    let mesh = obj.get_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere() -> BodyType {
    let mut sphere = Sphere::new(50.0, 10, 10);
    sphere.set_offset(50.0, 1200.0, 2000.0);

    let mesh = sphere.get_triangle_mesh();
    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}
