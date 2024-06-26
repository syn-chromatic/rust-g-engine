use crate::components::frustum::Frustum;
use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;

pub struct Camera {
    pub frustum: Frustum,
    pub yaw: f64,
    pub pitch: f64,
    pub camera_position: Vector3D,
    pub camera_target: Vector3D,
    pub side_direction: Vector3D,
    pub up_direction: Vector3D,
    pub look_direction: Vector3D,
    pub y_lock: bool,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let frustum: Frustum = Frustum::new(width, height);
        let yaw: f64 = 250.0;
        let pitch: f64 = 0.0;

        let side_direction: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
        let up_direction: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
        let look_direction: Vector3D = Vector3D::new(0.0, 0.0, 1.0);

        let camera_position: Vector3D = Vector3D::new(-100.0, 200.0, 500.0);
        let camera_target: Vector3D = camera_position.clone().add_elements(0.0, 0.0, -1.0);
        let y_lock: bool = false;

        Camera {
            frustum,
            yaw,
            pitch,
            camera_position,
            camera_target,
            side_direction,
            up_direction,
            look_direction,
            y_lock,
        }
    }

    pub fn calibrate(&mut self) {
        self.apply_target_adjustment();
    }

    pub fn toggle_y_lock(&mut self) {
        self.y_lock = !self.y_lock;
    }

    pub fn set_camera_position(&mut self, position: Vector3D) {
        self.camera_position = position;
        self.camera_target = position.add_elements(0.0, 0.0, -1.0);
    }

    pub fn apply_view_transform(&mut self, position: Vector3D) -> Vector3D {
        let look_dir: Vector3D = self.look_direction;
        let side_dir: Vector3D = self.side_direction;
        let up_dir: Vector3D = self.up_direction;

        let point: Vector3D = self.camera_position.subtract_vector(&position);
        let x: f64 = point.dot_product(&side_dir);
        let y: f64 = point.dot_product(&up_dir);
        let z: f64 = point.dot_product(&look_dir);

        let translated_point: Vector3D = Vector3D::new(x, y, z);
        translated_point
    }

    pub fn ndc_to_screen_coordinates(&self, position: Vector3D) -> Vector3D {
        let width: f64 = self.frustum.width as f64;
        let height: f64 = self.frustum.height as f64;

        let x: f64 = (position.x + 1.0) * width / 2.0;
        let y: f64 = (1.0 - position.y) * height / 2.0;
        let z: f64 = position.z;

        let screen_coordinates = Vector3D::new(x, y, z);
        screen_coordinates
    }

    pub fn calculate_perspective_projection(&self, position: Vector3D) -> Vector3D {
        let width: f64 = self.frustum.width as f64;
        let height: f64 = self.frustum.height as f64;
        let fov_degrees: f64 = self.frustum.fov;
        let zn: f64 = self.frustum.near_plane;
        let zf: f64 = self.frustum.far_plane;

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

    pub fn transform_vertex(&mut self, vertex: Vector3D) -> Vector3D {
        let vertex: Vector3D = self.apply_view_transform(vertex);
        let vertex: Vector3D = self.calculate_perspective_projection(vertex);
        let vertex: Vector3D = self.ndc_to_screen_coordinates(vertex);
        vertex
    }

    pub fn transform_line(&mut self, v1: Vector3D, v2: Vector3D) -> Option<(Vector3D, Vector3D)> {
        let v1: Vector3D = self.apply_view_transform(v1);
        let v2: Vector3D = self.apply_view_transform(v2);

        let clipped_line: Option<(Vector3D, Vector3D)> = self.frustum.clip_line_to_frustum(v1, v2);
        if clipped_line.is_some() {
            let (v1, v2): (Vector3D, Vector3D) = clipped_line.unwrap();
            let v1: Vector3D = self.calculate_perspective_projection(v1);
            let v2: Vector3D = self.calculate_perspective_projection(v2);

            let v1: Vector3D = self.ndc_to_screen_coordinates(v1);
            let v2: Vector3D = self.ndc_to_screen_coordinates(v2);
            return Some((v1, v2));
        }

        None
    }

    fn apply_polygon_view_transform(&mut self, polygon: &mut Polygon) {
        match polygon {
            Polygon::Triangle(ref mut triangle) => {
                for vertex in &mut triangle.vertices {
                    *vertex = self.apply_view_transform(vertex.clone());
                }
            }
            Polygon::Quad(ref mut quad) => {
                for vertex in &mut quad.vertices {
                    *vertex = self.apply_view_transform(vertex.clone());
                }
            }
        }
    }

    fn apply_polygon_perspective_transform(&mut self, polygon: &mut Polygon) {
        match polygon {
            Polygon::Triangle(ref mut triangle) => {
                for vertex in &mut triangle.vertices {
                    *vertex = self.calculate_perspective_projection(*vertex);
                }
            }
            Polygon::Quad(ref mut quad) => {
                for vertex in &mut quad.vertices {
                    *vertex = self.calculate_perspective_projection(*vertex);
                }
            }
        }
    }

    fn apply_polygon_screen_transform(&mut self, polygon: &mut Polygon) {
        match polygon {
            Polygon::Triangle(ref mut triangle) => {
                for vertex in &mut triangle.vertices {
                    *vertex = self.ndc_to_screen_coordinates(*vertex);
                }
            }
            Polygon::Quad(ref mut quad) => {
                for vertex in &mut quad.vertices {
                    *vertex = self.ndc_to_screen_coordinates(*vertex);
                }
            }
        }
    }

    pub fn apply_projection_polygons(&mut self, polygons: &mut Vec<Polygon>) {
        let mut projected_polygons: Vec<Polygon> = Vec::with_capacity(polygons.len());

        for mut polygon in polygons.drain(..) {
            self.apply_polygon_view_transform(&mut polygon);

            if self.frustum.is_polygon_outside_frustum(&polygon) {
                continue;
            }

            if self.frustum.is_polygon_crossing_frustum(&polygon) {
                let mut clipped_polygons: Vec<Polygon> =
                    self.frustum.clip_polygon_against_frustum(&polygon);

                for mut clipped_polygon in clipped_polygons.drain(..) {
                    self.apply_polygon_perspective_transform(&mut clipped_polygon);
                    self.apply_polygon_screen_transform(&mut clipped_polygon);
                    projected_polygons.push(clipped_polygon);
                }
                continue;
            }

            self.apply_polygon_perspective_transform(&mut polygon);
            self.apply_polygon_screen_transform(&mut polygon);
            projected_polygons.push(polygon);
        }

        polygons.extend(projected_polygons);
    }

    pub fn handle_mouse_movement(&mut self, dx: f64, dy: f64) {
        let sens_x: f64 = 0.3;
        let sens_y: f64 = 0.3;

        self.yaw += dx * -sens_x;
        self.pitch += dy * sens_y;

        if self.pitch >= 90.0 {
            self.pitch = 90.0
        }
        if self.pitch <= -90.0 {
            self.pitch = -90.0
        }

        self.apply_target_adjustment();
    }

    fn apply_direction_adjustment(&mut self) {
        self.look_direction = self.camera_target.subtract_vector(&self.camera_position);
        self.look_direction = self.look_direction.normalize();
        self.side_direction = self.look_direction.cross_product(&self.up_direction);
        self.side_direction = self.side_direction.normalize();

        let yaw_rad: f64 = self.yaw.to_radians();
        let pitch_rad: f64 = (self.pitch + 90.0).to_radians();
        let pitch_rad_cos: f64 = pitch_rad.cos();
        let pitch_rad_sin: f64 = pitch_rad.sin();

        let yaw_rad_cos: f64 = yaw_rad.cos();
        let yaw_rad_sin: f64 = yaw_rad.sin();

        let up_x: f64 = yaw_rad_cos * pitch_rad_cos;
        let up_y: f64 = pitch_rad_sin;
        let up_z: f64 = yaw_rad_sin * pitch_rad_cos;
        self.up_direction = Vector3D::new(up_x, up_y, up_z).normalize();
    }

    fn apply_target_adjustment(&mut self) {
        let yaw_rad: f64 = self.yaw.to_radians();
        let pitch_rad: f64 = self.pitch.to_radians();

        let pitch_rad_cos: f64 = pitch_rad.cos();
        let pitch_rad_sin: f64 = pitch_rad.sin();

        let yaw_rad_cos: f64 = yaw_rad.cos();
        let yaw_rad_sin: f64 = yaw_rad.sin();

        let direction_x: f64 = yaw_rad_cos * pitch_rad_cos;
        let direction_y: f64 = pitch_rad_sin;
        let direction_z: f64 = yaw_rad_sin * pitch_rad_cos;

        let direction: Vector3D = Vector3D::new(direction_x, direction_y, direction_z);
        self.camera_target = self.camera_position.add_vector(&direction);
        self.apply_direction_adjustment();
    }

    pub fn increment_position_x(&mut self, increment: f64) {
        let mut side_vector: Vector3D = self.side_direction.multiply(increment);
        if self.y_lock {
            side_vector.y = 0.0;
        }
        self.camera_position = self.camera_position.add_vector(&side_vector);
        self.camera_target = self.camera_position.add_vector(&self.look_direction);
    }

    pub fn increment_position_y(&mut self, increment: f64) {
        let up_vector = self.up_direction.multiply(increment);
        self.camera_position = self.camera_position.add_vector(&up_vector);
        self.camera_target = self.camera_position.add_vector(&self.look_direction);
    }

    pub fn increment_position_z(&mut self, increment: f64) {
        let mut look_vector: Vector3D = self.look_direction.multiply(-increment);
        if self.y_lock {
            look_vector.y = 0.0;
        }
        self.camera_position = self.camera_position.add_vector(&look_vector);
        self.camera_target = self.camera_position.add_vector(&self.look_direction);
    }

    pub fn increment_planes(&mut self, increment: f64) {
        let mut near_plane: f64 = self.frustum.near_plane;
        let mut far_plane: f64 = self.frustum.far_plane;

        if (near_plane + increment) >= 0.1 {
            near_plane += increment;
            far_plane += increment;
            self.frustum.near_plane = near_plane.clamp(0.0, f64::INFINITY);
            self.frustum.far_plane = far_plane.clamp(0.0, f64::INFINITY);
            println!("{:?}: {:.2?}", "Near Plane", self.frustum.near_plane);
            println!("{:?}: {:.2?}", "Far Plane", self.frustum.far_plane);
        }
    }
}
