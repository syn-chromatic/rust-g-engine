use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::physics::Physics;

#[derive(Clone, Debug)]
pub struct Shape {
    pub physics: Physics,
    color: Color,
    line_thickness: f64,
}

impl Shape {
    pub fn new(shape: Vec<[f64; 3]>) -> Shape {
        let physics = Physics::new(shape.clone());
        let color = Color::from_rgb(1.0, 1.0, 1.0);
        let line_thickness = 3.0;
        Shape {
            physics,
            color,
            line_thickness,
        }
    }

    fn perspective_projection(&self, xyz_point: [f64; 3]) -> [f64; 3] {
        let distance: f64 = 5.0;
        let zp: f64 = 1.0 / (distance - xyz_point[2]);
        let xp: f64 = xyz_point[0] * zp;
        let yp: f64 = xyz_point[1] * zp;
        return [xp, yp, zp];
    }

    fn draw_edge(&self, a: [f64; 3], b: [f64; 3], color_shading: f64, graphics: &mut Graphics2D) {
        let scale = self.physics.scale;
        let z = self.physics.position.z;
        let mut relative_z: f64 = scale + z;
        relative_z = f64::max(0.1, relative_z).min(f64::INFINITY);

        let x1: f64 = a[0] * relative_z + self.physics.position.x;
        let y1: f64 = a[1] * relative_z + self.physics.position.y;
        let x2: f64 = b[0] * relative_z + self.physics.position.x;
        let y2: f64 = b[1] * relative_z + self.physics.position.y;

        let rgb: (f32, f32, f32) = (
            self.color.r() * color_shading as f32,
            self.color.g() * color_shading as f32,
            self.color.b() * color_shading as f32,
        );
        let color: Color = Color::from_rgb(rgb.0, rgb.1, rgb.2);

        let p1: Vector2<f32> = Vector2::new(x1, y1).into_f32();
        let p2: Vector2<f32> = Vector2::new(x2, y2).into_f32();
        let thickness = self.line_thickness as f32;
        graphics.draw_line(p1, p2, thickness, color);
    }

    fn draw_edge_perspective(
        &self,
        a: [f64; 3],
        b: [f64; 3],
        color_shading: f64,
        graphics: &mut Graphics2D,
    ) {
        let a: [f64; 3] = self.perspective_projection(a);
        let b: [f64; 3] = self.perspective_projection(b);
        self.draw_edge(a, b, color_shading, graphics)
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Color::from_rgb(r, g, b);
    }

    pub fn draw_shape(&self, graphics: &mut Graphics2D) {
        for i in 0..4 {
            let s1: usize = (i + 1) % 4;
            let s2: usize = i + 4;
            let s3: usize = s1 + 4;
            let shape_i: [f64; 3] = self.physics.shape[i];
            let shape_s1: [f64; 3] = self.physics.shape[s1];
            let shape_s2: [f64; 3] = self.physics.shape[s2];
            let shape_s3: [f64; 3] = self.physics.shape[s3];
            self.draw_edge(shape_i, shape_s1, 1.0, graphics);
            self.draw_edge(shape_i, shape_s2, 0.85, graphics);
            self.draw_edge(shape_s2, shape_s3, 0.75, graphics);
        }
    }
}
