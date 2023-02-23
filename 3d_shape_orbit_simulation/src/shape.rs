use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::vector_3d::Vector3D;

#[derive(Clone, Debug)]
pub struct ShapeBase {
    shape: Vec<[f64; 3]>,
    color: Color,
    x_angle: f64,
    y_angle: f64,
    z_angle: f64,
    mass: f64,
    scale: f64,
    position: Vector3D,
    velocity: Vector3D,
    acceleration: Vector3D,
}

impl ShapeBase {
    pub fn new(shape: Vec<[f64; 3]>, x: f64, y: f64, z: f64) -> ShapeBase {
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        let x_angle: f64 = 0.0;
        let y_angle: f64 = 0.0;
        let z_angle: f64 = 0.0;
        let scale: f64 = 1.0;
        let mass: f64 = 1.0;

        let position: Vector3D = Vector3D::new(x, y, z);
        let velocity: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let acceleration: Vector3D = Vector3D::new(0.0, 0.0, 0.0);

        ShapeBase {
            shape,
            color,
            x_angle,
            y_angle,
            z_angle,
            mass,
            scale,
            position,
            velocity,
            acceleration,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale
    }

    pub fn set_velocity(&mut self, vel_xyz: [f64; 3]) {
        self.velocity.x = vel_xyz[0];
        self.velocity.y = vel_xyz[1];
        self.velocity.z = vel_xyz[2];
    }

    pub fn set_acceleration(&mut self, acc_xyz: [f64; 3]) {
        self.acceleration.x = acc_xyz[0];
        self.acceleration.y = acc_xyz[1];
        self.acceleration.z = acc_xyz[2];
    }

    fn rotate_z(&self, xyz_point: [f64; 3], theta: f64) -> [f64; 3] {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = cs * xyz_point[0] - sn * xyz_point[1];
        let y: f64 = sn * xyz_point[0] + cs * xyz_point[1];
        let z: f64 = xyz_point[2];

        let xyz_point: [f64; 3] = [x, y, z];
        xyz_point
    }

    fn rotate_x(&self, xyz_point: [f64; 3], theta: f64) -> [f64; 3] {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = xyz_point[0];
        let y: f64 = cs * xyz_point[1] - sn * xyz_point[2];
        let z: f64 = sn * xyz_point[1] + cs * xyz_point[2];
        let xyz_point: [f64; 3] = [x, y, z];
        xyz_point
    }

    fn rotate_y(&self, xyz_point: [f64; 3], theta: f64) -> [f64; 3] {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = cs * xyz_point[0] + sn * xyz_point[2];
        let y: f64 = xyz_point[1];
        let z: f64 = -sn * xyz_point[0] + cs * xyz_point[2];
        let xyz_point: [f64; 3] = [x, y, z];
        xyz_point
    }

    fn perspective_projection(&self, x: f64, y: f64, z: f64) -> [f64; 3] {
        let distance: f64 = 5.0;
        let zp: f64 = 1.0 / (distance - z);
        let xp: f64 = x * zp;
        let yp: f64 = y * zp;
        let xyz: [f64; 3] = [xp, yp, zp];
        xyz
    }

    fn draw_line(&mut self, p1: [f64; 2], p2: [f64; 2], color: Color, graphics: &mut Graphics2D) {
        let p1_spv: Vector2<f32> = Vector2 { x: p1[0], y: p1[1] }.into_f32();
        let p2_spv: Vector2<f32> = Vector2 { x: p2[0], y: p2[1] }.into_f32();
        let thickness: f32 = 5.0;
        graphics.draw_line(p1_spv, p2_spv, thickness, color);
    }

    fn draw_edge(
        &mut self,
        a: [f64; 3],
        b: [f64; 3],
        scale: f64,
        color_shading: f64,
        graphics: &mut Graphics2D,
    ) {
        let x1: f64 = a[0] * scale + self.position.x;
        let y1: f64 = a[1] * scale + self.position.y;
        let x2: f64 = b[0] * scale + self.position.x;
        let y2: f64 = b[1] * scale + self.position.y;

        let rgb: (f32, f32, f32) = (
            self.color.r() * color_shading as f32,
            self.color.g() * color_shading as f32,
            self.color.b() * color_shading as f32,
        );
        let color: Color = Color::from_rgb(rgb.0, rgb.1, rgb.2);
        self.draw_line([x1, y1], [x2, y2], color, graphics);
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub base: ShapeBase,
}

impl Shape {
    pub fn new(base: ShapeBase) -> Shape {
        Shape { base: base }
    }

    pub fn draw_shape(&mut self, scale: f64, graphics: &mut Graphics2D) {
        for i in 0..4 {
            let s1: usize = (i + 1) % 4;
            let s2: usize = i + 4;
            let s3: usize = s1 + 4;
            let shape_i: [f64; 3] = self.base.shape[i];
            let shape_s1: [f64; 3] = self.base.shape[s1];
            let shape_s2: [f64; 3] = self.base.shape[s2];
            let shape_s3: [f64; 3] = self.base.shape[s3];
            self.base.draw_edge(shape_i, shape_s1, scale, 1.0, graphics);
            self.base
                .draw_edge(shape_i, shape_s2, scale, 0.85, graphics);
            self.base
                .draw_edge(shape_s2, shape_s3, scale, 0.75, graphics);
        }
    }

    pub fn add_x_angle_rotation(&mut self, rotation: f64) {
        let x_angle: f64 = self.base.x_angle + rotation;
        let mut new_shape: Vec<[f64; 3]> = vec![];
        for p in &self.base.shape {
            let edge: [f64; 3] = self.base.rotate_x(*p, x_angle);
            new_shape.push(edge);
        }
        self.base.shape = new_shape;
    }

    pub fn add_y_angle_rotation(&mut self, rotation: f64) {
        let y_angle: f64 = self.base.y_angle + rotation;
        let mut new_shape: Vec<[f64; 3]> = vec![];
        for p in &self.base.shape {
            let edge: [f64; 3] = self.base.rotate_y(*p, y_angle);
            new_shape.push(edge);
        }
        self.base.shape = new_shape;
    }

    pub fn add_z_angle_rotation(&mut self, rotation: f64) {
        let z_angle: f64 = self.base.z_angle + rotation;
        let mut new_shape: Vec<[f64; 3]> = vec![];
        for p in &self.base.shape {
            let edge: [f64; 3] = self.base.rotate_z(*p, z_angle);
            new_shape.push(edge);
        }
        self.base.shape = new_shape;
    }

    pub fn add_total_angle_rotation(&mut self, rotation: f64) {
        self.add_x_angle_rotation(rotation);
        self.add_y_angle_rotation(rotation);
        self.add_z_angle_rotation(rotation);
    }

    fn move_object(&mut self, graphics: &mut Graphics2D) {
        let position: &Vector3D = &self.base.position;
        let base_scale: f64 = self.base.scale;
        let new_scale: f64 = base_scale + position.z;
        let norm_scale: f64 = f64::max(0.1, new_scale).min(f64::INFINITY);
        self.draw_shape(norm_scale, graphics);
    }

    pub fn update_object(&mut self, graphics: &mut Graphics2D) {
        self.base.position = self.base.position.add_vector(&self.base.velocity);
        self.base.velocity = self.base.velocity.add_vector(&self.base.acceleration);
        self.base.acceleration = self.base.acceleration.multiply(0.0);
        self.move_object(graphics);
    }

    pub fn apply_force(&mut self, force: &Vector3D) {
        let force: Vector3D = force.divide(self.base.mass);
        self.base.acceleration = self.base.acceleration.add_vector(&force);
    }

    pub fn apply_angular_force(&mut self, force: &Vector3D) {
        self.add_x_angle_rotation(force.x / 100000.0);
        self.add_y_angle_rotation(force.y / 100000.0);
        self.add_z_angle_rotation(force.z / 100000.0);
    }

    pub fn apply_attraction(&mut self, target: &mut Shape) {
        let mut force: Vector3D = target.base.position.subtract_vector(&self.base.position);
        let distance: f64 = force.get_length();
        let g_const: f64 = 0.0001;
        let strength: f64 = g_const * (self.base.mass * target.base.mass) / distance;
        let force = force.set_magnitude(strength);
        self.apply_force(force);
        self.apply_angular_force(force);
    }
}
