use crate::vector_3d::Vector3D;

pub struct Camera {
    width: u32,
    height: u32,
    camera_position: Vector3D,
    near_plane: f64,
    far_plane: f64,
    yaw: f64,
    pitch: f64,
    prev_mouse_pos: (f64, f64),
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let camera_position = Vector3D::new(0.0, 0.0, 0.0);
        let near_plane = 60.0;
        let far_plane = 160.0;
        let yaw = 0.0;
        let pitch = 0.0;
        let prev_mouse_pos = (width as f64 / 2.0, height as f64 / 2.0);

        Camera {
            width,
            height,
            camera_position,
            near_plane,
            far_plane,
            yaw,
            pitch,
            prev_mouse_pos,
        }
    }

    pub fn handle_mouse_movement(&mut self, x: f64, y: f64) {
        let px = self.prev_mouse_pos.0;
        let py = self.prev_mouse_pos.1;

        let dx = x - px;
        let dy = y - py;
        println!("{:?} {:?}", dx, dy);

        let sensitivity = 0.5;
        self.yaw += dx * sensitivity;
        self.pitch += dy * sensitivity;
        self.prev_mouse_pos = (x, y);
    }

    pub fn interpolate_radius(&self, position: Vector3D, radius: f64) -> f64 {
        let z: f64 = position.z;
        let interpolation_value: f64 = (z + self.near_plane) / (self.far_plane - self.near_plane);
        let radius_scaled: f64 = radius * interpolation_value;
        radius_scaled
    }

    pub fn yaw_projection(&self, position: Vector3D) -> Vector3D {
        let yaw_radians: f64 = self.yaw.to_radians();
        let yaw_cos: f64 = yaw_radians.cos();
        let yaw_sin: f64 = yaw_radians.sin();

        let yaw_x: f64 = position.x * yaw_cos - position.z * yaw_sin;
        let yaw_y: f64 = position.y;
        let yaw_z: f64 = position.x * yaw_sin + position.z * yaw_cos;

        return Vector3D::new(yaw_x, yaw_y, yaw_z);
    }

    pub fn pitch_projection(&self, position: Vector3D) -> Vector3D {
        let pitch_radians: f64 = self.pitch.to_radians();
        let pitch_cos: f64 = pitch_radians.cos();
        let pitch_sin: f64 = pitch_radians.sin();

        let pitch_x: f64 = position.x;
        let pitch_y: f64 = (position.y * pitch_cos) - (position.z * pitch_sin);
        let pitch_z: f64 = (position.y * pitch_sin) + (position.z * pitch_cos);

        return Vector3D::new(pitch_x, pitch_y, pitch_z);
    }

    pub fn perspective_projection(&self, position: Vector3D) -> Vector3D {
        // let direction = self.camera_position.subtract_vector(position).normalize();
        // let position = position.add_vector(self.camera_position);

        let position: Vector3D = self.yaw_projection(position);
        let position: Vector3D = self.pitch_projection(position);

        let x: f64 = (position.x * self.near_plane) / position.z;
        let y: f64 = (position.y * self.near_plane) / position.z;

        let z: f64 = (self.far_plane + self.near_plane) / (self.near_plane - self.far_plane);
        let w: f64 = -position.z / (self.far_plane - self.near_plane);

        let half_w: f64 = self.width as f64 / 2.0;
        let half_h: f64 = self.height as f64 / 2.0;
        let xp: f64 = (x * w) + half_w;
        let yp: f64 = (y * w) + half_h;
        let zp: f64 = z * w;

        let position = Vector3D::new(xp, yp, zp);
        position
    }

    pub fn increment_distance(&mut self, increment: f64) {
        if (self.near_plane + increment) >= 0.0 {
            self.near_plane += increment;
            self.far_plane += increment;
            self.near_plane = self.near_plane.clamp(0.0, f64::INFINITY);
            self.far_plane = self.far_plane.clamp(0.0, f64::INFINITY);
            println!("{:?}: {:.2?}", "Near Plane", self.near_plane);
            println!("{:?}: {:.2?}", "Far Plane", self.far_plane);
        }
    }

    pub fn move_camera(&mut self, direction: Vector3D) {
        let movement_speed = 10.0;
        let (x, y, z) = self.camera_position.get_tuple();
        let new_x = x + direction.x * movement_speed;
        let new_y = y + direction.y * movement_speed;
        let new_z = z + direction.z * movement_speed;
        self.camera_position = Vector3D::new(new_x, new_y, new_z);
    }
}
