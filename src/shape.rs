use std::u128::MAX;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
use crate::physics::Physics;
use crate::camera::Camera;

#[derive(Clone, Debug)]
pub struct Shape {
    pub physics: Physics,
    color: Color,
    thickness: f64,
}

impl Body for Shape {
    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Color::from_rgb(r, g, b);
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &Camera) {
        self.draw_shape(graphics, camera);
    }

    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
}

impl Shape {
    pub fn new(shape: Vec<[f64; 3]>) -> Shape {
        let physics: Physics = Physics::new(shape);
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        let thickness: f64 = 3.0;
        Shape {
            physics,
            color,
            thickness,
        }
    }

    fn perspective_projection(&self, xyz_point: [f64; 3]) -> [f64; 3] {
        let distance: f64 = 5.0;
        let zp: f64 = 1.0 / (distance - xyz_point[2]);
        let xp: f64 = xyz_point[0] * zp;
        let yp: f64 = xyz_point[1] * zp;
        return [xp, yp, zp];
    }

    fn draw_edge(&self, a: [f64; 3], b: [f64; 3], rgb: (f32, f32, f32), graphics: &mut Graphics2D) {
        let z: f64 = self.get_relative_z();

        let x1: f64 = a[0] * z + self.physics.position.x;
        let y1: f64 = a[1] * z + self.physics.position.y;
        let x2: f64 = b[0] * z + self.physics.position.x;
        let y2: f64 = b[1] * z + self.physics.position.y;

        let z_alpha: f32 = self.get_z_alpha(z);
        let color: Color = Color::from_rgba(rgb.0, rgb.1, rgb.2, z_alpha);

        let p1: Vector2<f32> = Vector2::new(x1, y1).into_f32();
        let p2: Vector2<f32> = Vector2::new(x2, y2).into_f32();
        let thickness: f32 = self.thickness as f32;
        graphics.draw_line(p1, p2, thickness, color);
    }

    fn draw_edge_perspective(
        &self,
        a: [f64; 3],
        b: [f64; 3],
        color: (f32, f32, f32),
        graphics: &mut Graphics2D,
    ) {
        let a: [f64; 3] = self.perspective_projection(a);
        let b: [f64; 3] = self.perspective_projection(b);
        self.draw_edge(a, b, color, graphics)
    }

    fn draw_shape(&self, graphics: &mut Graphics2D, camera: &Camera) {
        let shape: &Vec<[f64; 3]> = &self.physics.shape;
        let shape_length: usize = shape.len();
        let shading: Vec<f32> = self.get_static_shading_sequence(shape_length);

        let rgb: (f32, f32, f32) = self.get_rgb_values(self.color);

        for idx in 0..shape_length {
            let nxt_idx: usize = idx + 1;
            let color: (f32, f32, f32) = self.get_shaded_rgb(rgb, shading[idx]);
            if nxt_idx < shape_length {
                let p1: [f64; 3] = shape[idx];
                let p2: [f64; 3] = shape[nxt_idx];

                self.draw_edge(p1, p2, color, graphics);
                continue;
            }

            let p1: [f64; 3] = shape[idx];
            let p2: [f64; 3] = shape[0];
            self.draw_edge(p1, p2, color, graphics);
        }
    }

    fn get_static_shading_sequence(&self, shape_length: usize) -> Vec<f32> {
        let mut shading: Vec<f32> = Vec::new();
        for i in (0..shape_length.pow(2)).step_by(shape_length as usize) {
            let value = i as f32 / shape_length.pow(2) as f32;
            shading.push(value);
        }
        shading
    }

    fn get_z_alpha(&self, z: f64) -> f32 {
        let max_z: f32 = 300.0;
        let min_z: f32 = max_z / 2.0;
        if z < min_z as f64 {
            return 1.0;
        }
        let alpha_normalized = (z as f32 - min_z) / (max_z - min_z);
        let alpha_clamped = alpha_normalized.clamp(0.0, 1.0);
        let alpha = 1.0 - alpha_clamped;
        alpha
    }

    fn get_rgb_values(&self, color: Color) -> (f32, f32, f32) {
        let r: f32 = self.color.r();
        let g: f32 = self.color.g();
        let b: f32 = self.color.b();
        (r, g, b)
    }

    fn get_relative_z(&self) -> f64 {
        let z: f64 = self.physics.position.z;
        let scale: f64 = self.physics.scale;
        let mut relative_z: f64 = scale + z;
        relative_z = f64::max(0.1, relative_z).min(f64::INFINITY);
        relative_z
    }

    fn get_shaded_rgb(&self, rgb: (f32, f32, f32), shade_value: f32) -> (f32, f32, f32) {
        let color: (f32, f32, f32) = (
            rgb.0 * shade_value,
            rgb.1 * shade_value,
            rgb.2 * shade_value,
        );
        color
    }
}
