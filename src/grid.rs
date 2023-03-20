use speedy2d::color::Color;
use speedy2d::Graphics2D;
use std::option::Option;

use crate::body::Body;
use crate::camera::Camera;
use crate::physics::Physics;
use crate::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct GridGround {
    rows: i32,
    columns: i32,
    cell_size: f32,
    physics: Physics,
    color: Color,
}

impl Body for GridGround {
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        self.draw(graphics, camera);
    }

    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
}

impl GridGround {
    pub fn new(rows: i32, columns: i32, cell_size: f32) -> GridGround {
        let shape = vec![[1.0, 1.0, 1.0]];

        GridGround {
            rows,
            columns,
            cell_size,
            physics: Physics::new(shape),
            color: Color::from_rgb(1.0, 1.0, 1.0),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        for row in 0..=self.rows {
            for column in 0..=self.columns {
                self.draw_grid_line(graphics, camera, row, column);
            }
        }
    }

    fn get_position1(&self, row: i32, column: i32) -> Vector3D {
        self.get_cell_position(row, column)
    }

    fn get_position2(&self, row: i32, column: i32) -> Option<Vector3D> {
        if row < self.rows {
            Some(self.get_cell_position(row + 1, column))
        } else {
            None
        }
    }

    fn get_position3(&self, row: i32, column: i32) -> Option<Vector3D> {
        if column < self.columns {
            Some(self.get_cell_position(row, column + 1))
        } else {
            None
        }
    }

    fn draw_grid_line(
        &self,
        graphics: &mut Graphics2D,
        camera: &mut Camera,
        row: i32,
        column: i32,
    ) {
        let position1 = self.get_position1(row, column);
        let position2 = self.get_position2(row, column);
        let position3 = self.get_position3(row, column);

        if let (Some(proj1), Some(proj2)) = (
            camera.get_screen_coordinates(position1),
            position2.and_then(|p| camera.get_screen_coordinates(p)),
        ) {
            let (x1, y1) = (proj1.x as f32, proj1.y as f32);
            let (x2, y2) = (proj2.x as f32, proj2.y as f32);
            graphics.draw_line((x1, y1), (x2, y2), 1.0, self.color);
        }

        if let (Some(proj1), Some(proj3)) = (
            camera.get_screen_coordinates(position1),
            position3.and_then(|p| camera.get_screen_coordinates(p)),
        ) {
            let (x1, y1) = (proj1.x as f32, proj1.y as f32);
            let (x3, y3) = (proj3.x as f32, proj3.y as f32);
            graphics.draw_line((x1, y1), (x3, y3), 1.0, self.color);
        }
    }

    fn get_cell_position(&self, row: i32, column: i32) -> Vector3D {
        let x = ((column as f32) * self.cell_size) as f64;
        let z = ((row as f32) * self.cell_size) as f64;
        Vector3D::new(x, -200.0, z)
    }
}

