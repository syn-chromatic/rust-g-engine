use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::physics::Physics;
use crate::vector_3d::Vector3D;
use crate::{body::Body, camera::Camera};

#[derive(Clone, Debug)]
pub struct Particle {
    pub physics: Physics,
    color: Color,
}

impl Body for Particle {
    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Color::from_rgb(r, g, b);
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &Camera) {
        self.draw_circle(graphics, camera);
    }

    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
}

impl Particle {
    pub fn new(shape: Vec<[f64; 3]>) -> Particle {
        let physics: Physics = Physics::new(shape.clone());
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        Particle { physics, color }
    }

    fn perspective_projection(
        &self,
        position: Vector3D,
        fov: f64,
        near_clipping_plane: f64,
        far_clipping_plane: f64,
    ) -> Vector3D {
        let aspect: f64 = 1200.0 / 800.0;
        let h: f64 = (fov.to_radians() / 2.0).tan() * near_clipping_plane;
        let w: f64 = h * aspect;

        let x: f64 = position.x * (2.0 * near_clipping_plane) / (w + w - position.x * w);
        let y: f64 = position.y * (2.0 * near_clipping_plane) / (h + h - position.y * h);
        let z: f64 = -(far_clipping_plane + near_clipping_plane)
            / (far_clipping_plane - near_clipping_plane)
            - (2.0 * far_clipping_plane * near_clipping_plane)
                / (position.z * (far_clipping_plane - near_clipping_plane));
        let w: f64 = -1.0 * (position.z * (far_clipping_plane - near_clipping_plane))
            / (far_clipping_plane * near_clipping_plane);

        let xp: f64 = x / w;
        let yp: f64 = y / w;
        let zp: f64 = z / w;

        return Vector3D::new(xp, yp, zp);
    }

    // fn perspective_projection(&self, position: Vector3D) -> Vector3D {
    //     let distance: f64 = 5.0;
    //     let zp: f64 = 1.0 / (distance - position.z);
    //     let xp: f64 = position.x * zp;
    //     let yp: f64 = position.y * zp;
    //     return Vector3D::new(xp, yp, zp);
    // }

    // fn perspective_projection(&self, position: Vector3D) -> Vector3D {
    //     let camera_position = Point3::new(0.0, 0.0, 0.0); // assume camera at origin
    //     let camera_direction = Point3::new(0.0, 0.0, -1.0); // assume camera points in -z direction
    //     let up_direction = Vector3::new(0.0, 1.0, 0.0); // assume up direction is +y

    //     let aspect_ratio = 1200 / 800; // assume screen aspect ratio
    //     let fov = 60; // assume field of view of 60 degrees
    //     let znear = 0.1; // assume near clipping plane distance of 0.1 units
    //     let zfar = 100.0; // assume far clipping plane distance of 100 units

    //     let view = Matrix4::look_at_lh(&camera_position, &camera_direction, &up_direction);
    //     let projection = Matrix4::new_perspective(aspect_ratio, fov, znear, zfar);

    //     let view_projection = projection * view;

    //     let mut position_homogeneous = position.to_homogeneous();
    //     position_homogeneous.w = 1.0; // set w component to 1

    //     let projected_homogeneous = view_projection * position_homogeneous;
    //     let mut projected = projected_homogeneous.to_vector();
    //     projected /= projected.w; // normalize by w component

    //     Vector3D::new(projected.x, projected.y, projected.z)
    // }

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
        relative_z = f64::max(0.5, relative_z).min(f64::INFINITY);
        relative_z
    }

    fn draw_circle(&self, graphics: &mut Graphics2D, camera: &Camera) {
        let position = camera.perspective_projection(self.physics.position);
        let x = position.x;
        let y = position.y;

        let radius = camera.interpolate_radius(position, self.physics.scale);
        // let radius = radius + self.physics.scale;

        let x = (1200.0 / 2.0) + x;
        let y = (800.0 / 2.0) + y;

        // let radius = f64::max(f64::INFINITY, radius).min(0.1);
        // println!("{:?} | {:?} | {:?}", x, y, radius);

        let z_alpha: f32 = self.get_z_alpha(radius);
        let rgb: (f32, f32, f32) = self.get_rgb_values(self.color);
        let color: Color = Color::from_rgba(rgb.0, rgb.1, rgb.2, z_alpha);

        let p: Vector2<f32> = Vector2::new(x, y).into_f32();
        graphics.draw_circle(p, radius as f32, color);
    }

    // fn draw_circle(&self, graphics: &mut Graphics2D, camera: &Camera) {
    //     let x: f64 = self.physics.position.x;
    //     let y: f64 = self.physics.position.y;
    //     let radius: f64 = self.get_relative_z();

    //     let z_alpha: f32 = self.get_z_alpha(radius);
    //     let rgb: (f32, f32, f32) = self.get_rgb_values(self.color);
    //     let color: Color = Color::from_rgba(rgb.0, rgb.1, rgb.2, z_alpha);

    //     let p: Vector2<f32> = Vector2::new(x, y).into_f32();
    //     graphics.draw_circle(p, radius as f32, color);
    // }
}
