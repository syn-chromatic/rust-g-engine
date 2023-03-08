use crate::vector_3d::Vector3D;

pub struct Camera {
    width: u32,
    height: u32,
    near_plane: f64,
    far_plane: f64,
    yaw: f64,
    pitch: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let near_plane = 60.0;
        let far_plane = 160.0;
        let yaw = 0.0;
        let pitch = 0.0;

        Camera {
            width,
            height,
            near_plane,
            far_plane,
            yaw,
            pitch,
        }
    }

    pub fn handle_mouse_movement(&mut self, dx: f64, dy: f64) {
        let sensitivity = 0.005;
        self.yaw += dx * sensitivity;
        self.pitch += dy * sensitivity;
    }

    pub fn interpolate_radius(&self, position: Vector3D, radius: f64) -> f64 {
        let z: f64 = position.z;
        let interpolation_value: f64 = (z - self.near_plane) / (self.far_plane - self.near_plane);
        let radius_scaled: f64 = radius * interpolation_value;
        radius_scaled
    }

    pub fn yaw_projection(&self, position: Vector3D) -> Vector3D {

        let yaw_x = position.x * self.yaw.to_radians().cos() - position.z * self.yaw.to_radians().sin();
        let yaw_y = position.y;
        let yaw_z = position.x * self.yaw.to_radians().sin() + position.z * self.yaw.to_radians().cos();

        return Vector3D::new(yaw_x, yaw_y, yaw_z);
    }

    pub fn pitch_projection(&self, position: Vector3D) -> Vector3D {
        let pitch_x = position.x;
        let pitch_y = position.y * self.pitch.to_radians().cos() - position.z * self.pitch.to_radians().sin();
        let pitch_z = position.y * self.pitch.to_radians().sin() + position.z * self.pitch.to_radians().cos();

        return Vector3D::new(pitch_x, pitch_y, pitch_z);
    }

    pub fn perspective_projection(&self, position: Vector3D) -> Vector3D {
        let position = self.yaw_projection(position);
        let position = self.pitch_projection(position);

        let x: f64 = (position.x * self.near_plane) / position.z;
        let y: f64 = (position.y * self.near_plane) / position.z;

        let z: f64 = (self.far_plane + self.near_plane) / (self.near_plane - self.far_plane);
        let w: f64 = -position.z / (self.far_plane - self.near_plane);

        let half_w: f64 = self.width as f64 / 2.0;
        let half_h: f64 = self.height as f64 / 2.0;
        let xp: f64 = (x * w) + half_w;
        let yp: f64 = (y * w) + half_h;
        let zp: f64 = z * w;

        return Vector3D::new(xp, yp, zp);
    }

    pub fn increase_distance(&mut self, increment: f64) {
        self.increase_near_plane(increment);
        self.increase_far_plane(increment);
    }

    pub fn decrease_distance(&mut self, increment: f64) {
        self.decrease_near_plane(increment);
        self.decrease_far_plane(increment);
    }

    pub fn increase_near_plane(&mut self, increment: f64) {
        self.near_plane += increment;
        println!("{:?}: {:.2?}", "Near Plane", self.near_plane)
    }

    pub fn decrease_near_plane(&mut self, increment: f64) {
        self.near_plane -= increment;
        println!("{:?}: {:.2?}", "Near Plane", self.near_plane)
    }

    pub fn increase_far_plane(&mut self, increment: f64) {
        self.far_plane += increment;
        println!("{:?}: {:.2?}", "Far Plane", self.far_plane)
    }

    pub fn decrease_far_plane(&mut self, increment: f64) {
        self.far_plane -= increment;
        println!("{:?}: {:.2?}", "Far Plane", self.far_plane)
    }
}
