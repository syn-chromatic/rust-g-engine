use crate::vectors::Vector3D;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: u32,
    pub near_plane: f64,
    pub far_plane: f64,
    pub yaw: f64,
    pub pitch: f64,
    pub camera_position: Vector3D,
    pub camera_target: Vector3D,
    pub side_direction: Vector3D,
    pub up_direction: Vector3D,
    pub look_direction: Vector3D,
    pub previous_pointer: (f64, f64),
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let camera_position: Vector3D = Vector3D::new(0.0, 0.0, 1000.0);
        let camera_target: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let side_direction: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
        let up_direction: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
        let look_direction: Vector3D = Vector3D::new(0.0, 0.0, 1.0);

        let fov: u32 = 90;
        let near_plane: f64 = 0.1;
        let far_plane: f64 = 50_000.0;
        let yaw: f64 = 0.0;
        let pitch: f64 = 0.0;
        let previous_pointer: (f64, f64) = (width as f64 / 2.0, height as f64 / 2.0);

        Camera {
            width,
            height,
            fov,
            near_plane,
            far_plane,
            yaw,
            pitch,
            camera_position,
            camera_target,
            side_direction,
            up_direction,
            look_direction,
            previous_pointer,
        }
    }

    fn apply_view_transformation(&mut self, position: Vector3D) -> Vector3D {
        self.apply_direction_adjustment();

        let look_dir: Vector3D = self.look_direction;
        let side_dir: Vector3D = self.side_direction;
        let up_dir: Vector3D = self.up_direction;

        let point: Vector3D = position.subtract_vector(self.camera_position);
        let x: f64 = point.dot_product(side_dir);
        let y: f64 = point.dot_product(up_dir);
        let z: f64 = point.dot_product(look_dir);

        let translated_point: Vector3D = Vector3D::new(x, y, z);
        translated_point
    }

    fn ndc_to_screen_coordinates(&self, position: Vector3D) -> Vector3D {
        let width: f64 = self.width as f64;
        let height: f64 = self.height as f64;

        let x: f64 = (position.x + 1.0) * width / 2.0;
        let y: f64 = (1.0 - position.y) * height / 2.0;
        let z: f64 = position.z;

        let screen_coordinates = Vector3D::new(x, y, z);
        screen_coordinates
    }

    fn calculate_perspective_projection(&self, position: Vector3D) -> Vector3D {
        let width: f64 = self.width as f64;
        let height: f64 = self.height as f64;
        let fov_degrees: f64 = self.fov as f64;
        let zn: f64 = self.near_plane;
        let zf: f64 = self.far_plane;

        let xi: f64 = position.x;
        let yi: f64 = position.y;
        let zi: f64 = position.z;

        let aspect_ratio: f64 = width / height;
        let fov_rad: f64 = (fov_degrees / 2.0).to_radians().tan();

        let mut xo: f64 = xi * (1.0 / (fov_rad * aspect_ratio));
        let mut yo: f64 = yi * (1.0 / (fov_rad));
        let mut zo: f64 = zi * -((zf - zn) / (zn - zf)) + ((2.0 * zf * zn) / (zn - zf));

        if zi != 0.0 {
            xo /= -zi;
            yo /= -zi;
            zo /= -zi;
        }

        let vo: Vector3D = Vector3D::new(xo, yo, zo);
        vo
    }

    pub fn handle_mouse_movement(&mut self, x: f64, y: f64) {
        let sens_x: f64 = 0.3;
        let sens_y: f64 = 0.1;

        let dx = x - self.previous_pointer.0;
        let dy = y - self.previous_pointer.1;
        self.previous_pointer = (x, y);

        self.yaw += dx * sens_x;
        self.pitch += dy * -sens_y;

        if self.pitch > 89.0 {
            self.pitch = 89.0
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0
        }

        self.apply_mouse_movement();
    }

    fn apply_direction_adjustment(&mut self) {
        self.look_direction = self.camera_target.subtract_vector(self.camera_position);
        self.look_direction = self.look_direction.normalize();
        self.side_direction = self.look_direction.cross_product(self.up_direction);
        self.side_direction = self.side_direction.normalize();

        let yaw_rad: f64 = self.yaw.to_radians();
        let pitch_rad: f64 = (self.pitch - 90.0).to_radians();
        let up_x: f64 = yaw_rad.cos() * pitch_rad.cos();
        let up_y: f64 = pitch_rad.sin();
        let up_z: f64 = yaw_rad.sin() * pitch_rad.cos();
        self.up_direction = Vector3D::new(up_x, up_y, up_z).normalize();
    }

    fn apply_mouse_movement(&mut self) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let direction_x = yaw_rad.cos() * pitch_rad.cos();
        let direction_y = pitch_rad.sin();
        let direction_z = yaw_rad.sin() * pitch_rad.cos();

        let direction = Vector3D::new(direction_x, direction_y, direction_z);
        self.camera_target = self.camera_position.add_vector(direction);
        self.apply_direction_adjustment();
    }

    pub fn increment_position_x(&mut self, increment: f64) {
        let mut side_vector: Vector3D = self.side_direction.multiply(-increment);
        side_vector.y = 0.0;
        self.camera_position = self.camera_position.add_vector(side_vector);
        self.camera_target = self.camera_position.add_vector(self.look_direction);
    }

    pub fn increment_position_y(&mut self, increment: f64) {
        let up_vector = self.up_direction.multiply(increment);
        self.camera_position = self.camera_position.add_vector(up_vector);
        self.camera_target = self.camera_position.add_vector(self.look_direction);
    }

    pub fn increment_position_z(&mut self, increment: f64) {
        let mut look_vector: Vector3D = self.look_direction.multiply(increment);
        look_vector.y = 0.0;
        self.camera_position = self.camera_position.add_vector(look_vector);
        self.camera_target = self.camera_position.add_vector(self.look_direction);
    }

    pub fn get_screen_coordinates(&mut self, position: Vector3D) -> Option<Vector3D> {
        if !self.is_point_in_frustum(position) {
            return None;
        }

        let view: Vector3D = self.apply_view_transformation(position);
        let projection: Vector3D = self.calculate_perspective_projection(view);
        let screen: Vector3D = self.ndc_to_screen_coordinates(projection);
        Some(screen)
    }

    fn is_point_in_frustum(&self, position: Vector3D) -> bool {
        let look_dir: Vector3D = self
            .camera_target
            .subtract_vector(self.camera_position)
            .normalize();
        let side_dir: Vector3D = look_dir.cross_product(self.up_direction).normalize();
        let up_dir: Vector3D = side_dir.cross_product(look_dir).normalize();

        let near_dir: Vector3D = look_dir.multiply(self.near_plane);
        let near_center: Vector3D = self.camera_position.add_vector(near_dir);
        let far_center: Vector3D = self
            .camera_position
            .add_vector(look_dir.multiply(self.far_plane));

        let aspect_ratio: f64 = (self.width as f64 / self.height as f64);
        let fov_rad: f64 = (self.fov as f64 / 2.0).to_radians().tan();
        let margin: f64 = 2.0;

        let near_height: f64 = 2.0 * self.near_plane * fov_rad * margin;
        let near_width: f64 = near_height * aspect_ratio;
        let far_height: f64 = 2.0 * self.far_plane * fov_rad * margin;
        let far_width: f64 = far_height * aspect_ratio;

        let near_up: Vector3D = up_dir.multiply(near_height / 2.0);
        let near_right: Vector3D = side_dir.multiply(near_width / 2.0);
        let far_up: Vector3D = up_dir.multiply(far_height / 2.0);
        let far_right: Vector3D = side_dir.multiply(far_width / 2.0);

        let points: [Vector3D; 8] = [
            near_center
                .subtract_vector(near_right)
                .subtract_vector(near_up),
            near_center.add_vector(near_right).subtract_vector(near_up),
            near_center.add_vector(near_right).add_vector(near_up),
            near_center.subtract_vector(near_right).add_vector(near_up),
            far_center
                .subtract_vector(far_right)
                .subtract_vector(far_up),
            far_center.add_vector(far_right).subtract_vector(far_up),
            far_center.add_vector(far_right).add_vector(far_up),
            far_center.subtract_vector(far_right).add_vector(far_up),
        ];

        let planes: [(Vector3D, Vector3D, Vector3D); 6] = [
            (points[0], points[3], points[2]),
            (points[4], points[5], points[6]),
            (points[0], points[1], points[5]),
            (points[2], points[3], points[7]),
            (points[0], points[4], points[7]),
            (points[1], points[2], points[6]),
        ];

        for plane in planes.iter() {
            let (a, b, c) = *plane;
            let ab: Vector3D = b.subtract_vector(a);
            let ac: Vector3D = c.subtract_vector(a);
            let normal: Vector3D = ab.cross_product(ac).normalize();
            let ap: Vector3D = position.subtract_vector(a);

            if normal.dot_product(ap) < 0.0 {
                return false;
            }
        }
        true
    }

    pub fn increment_distance(&mut self, increment: f64) {
        let mut near_plane: f64 = self.near_plane;
        let mut far_plane: f64 = self.far_plane;

        if (near_plane + increment) >= 0.1 {
            near_plane += increment;
            far_plane += increment;
            self.near_plane = near_plane.clamp(0.0, f64::INFINITY);
            self.far_plane = far_plane.clamp(0.0, f64::INFINITY);
            println!("{:?}: {:.2?}", "Near Plane", self.near_plane);
            println!("{:?}: {:.2?}", "Far Plane", self.far_plane);
        }
    }
}
