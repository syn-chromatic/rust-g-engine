use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
use crate::camera::Camera;
use crate::physics::Physics;
use crate::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct Shape {
    physics: Physics,
    color: Color,
    thickness: f64,
}

impl Body for Shape {
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
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

    fn draw_shape(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
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

                self.draw_edge(p1, p2, color, graphics, camera);
                continue;
            }

            let p1: [f64; 3] = shape[idx];
            let p2: [f64; 3] = shape[0];
            self.draw_edge(p1, p2, color, graphics, camera);
        }
    }

    fn draw_edge(
        &self,
        a: [f64; 3],
        b: [f64; 3],
        rgb: (f32, f32, f32),
        graphics: &mut Graphics2D,
        camera: &mut Camera,
    ) {
        let position: Vector3D = self.get_shape_position();
        let scale: f64 = self.get_shape_scale();

        let point_a: (f64, f64, f64) = (
            (a[0] * scale) + position.x,
            (a[1] * scale) + position.y,
            (a[2] * scale) + position.z,
        );
        let point_b: (f64, f64, f64) = (
            (b[0] * scale) + position.x,
            (b[1] * scale) + position.y,
            (b[2] * scale) + position.z,
        );
        let point_av: Vector3D = Vector3D::new(point_a.0, point_a.1, point_a.2);
        let point_bv: Vector3D = Vector3D::new(point_b.0, point_b.1, point_b.2);

        let point_av: Option<Vector3D> = camera.get_screen_coordinates(point_av);
        let point_bv: Option<Vector3D> = camera.get_screen_coordinates(point_bv);
        if point_av.is_none() || point_bv.is_none() {
            return;
        }

        let point_av = point_av.unwrap();
        let point_bv = point_bv.unwrap();
        let p_scale = point_av.subtract_vector(point_bv).get_length() / 2.0;
        let alpha = self.get_scale_alpha(p_scale);
        let color: Color = Color::from_rgba(rgb.0, rgb.1, rgb.2, alpha);

        let point_a_2d = (point_av.x as f32, point_av.y as f32);
        let point_b_2d = (point_bv.x as f32, point_bv.y as f32);
        let thickness = self.thickness as f32;

        graphics.draw_line(point_a_2d, point_b_2d, thickness, color);
    }

    fn get_static_shading_sequence(&self, shape_length: usize) -> Vec<f32> {
        let mut shading: Vec<f32> = Vec::new();
        for i in (0..shape_length.pow(2)).step_by(shape_length as usize) {
            let value = i as f32 / shape_length.pow(2) as f32;
            shading.push(value);
        }
        shading
    }

    fn get_scale_alpha(&self, scale: f64) -> f32 {
        let max_scale: f32 = 500.0;
        let min_scale: f32 = max_scale / 2.0;
        if scale < min_scale as f64 {
            return 1.0;
        }
        let alpha_normalized = (scale as f32 - min_scale) / (max_scale - min_scale);
        let alpha_clamped = alpha_normalized.clamp(0.0, 1.0);
        let alpha = 1.0 - alpha_clamped;
        alpha
    }

    fn get_rgb_values(&self, color: Color) -> (f32, f32, f32) {
        let r: f32 = color.r();
        let g: f32 = color.g();
        let b: f32 = color.b();
        (r, g, b)
    }

    fn get_shaded_rgb(&self, rgb: (f32, f32, f32), shade_value: f32) -> (f32, f32, f32) {
        let rgb: (f32, f32, f32) = (
            rgb.0 * shade_value,
            rgb.1 * shade_value,
            rgb.2 * shade_value,
        );
        rgb
    }

    fn get_shape_position(&self) -> Vector3D {
        self.physics.position
    }

    fn get_shape_scale(&self) -> f64 {
        self.physics.scale
    }
}
