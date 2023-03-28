use crate::body::BodyType;
use crate::model::OBJModelFormat;
use crate::shape::Shape;
use crate::vertices::GridHorizontal;

pub fn get_grid() -> BodyType {
    let mut grid = GridHorizontal::new(15, 15, 100.0);
    grid.set_offset(-1000.0, -100.0, -1000.0);
    let mesh = grid.get_triangle_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_obj() -> BodyType {
    let mut obj = OBJModelFormat::new("./cottage.obj", 0.2);
    obj.set_offset(0.0, -100.0, 0.0);

    let mesh = obj.get_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}
