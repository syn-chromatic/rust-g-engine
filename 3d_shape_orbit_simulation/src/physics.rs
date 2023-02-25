use crate::vector_3d::Vector3D;

#[derive(Clone, Debug)]
pub struct Physics {
    pub shape: Vec<[f64; 3]>,
    pub position: Vector3D,
    pub velocity: Vector3D,
    pub acceleration: Vector3D,
    pub spin_velocity: Vector3D,
    pub spin_acceleration: Vector3D,
    pub mass: f64,
    pub scale: f64,
}

impl Physics {
    pub fn new(shape: Vec<[f64; 3]>) -> Physics {
        let position: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let velocity: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let acceleration: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let spin_velocity: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let spin_acceleration: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let mass: f64 = 1.0;
        let scale: f64 = 1.0;

        Physics {
            shape,
            position,
            velocity,
            acceleration,
            spin_velocity,
            spin_acceleration,
            mass,
            scale,
        }
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

    fn rotate_z(&self, xyz_point: [f64; 3], theta: f64) -> [f64; 3] {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = cs * xyz_point[0] - sn * xyz_point[1];
        let y: f64 = sn * xyz_point[0] + cs * xyz_point[1];
        let z: f64 = xyz_point[2];
        let xyz_point: [f64; 3] = [x, y, z];
        xyz_point
    }

    fn constrain(&self, value: f64, min_value: f64, max_value: f64) -> f64 {
        f64::max(min_value, value).min(max_value)
    }

    fn calculate_position(&mut self) {
        self.position = self.position.add_vector(self.velocity);
        self.velocity = self.velocity.add_vector(self.acceleration);
    }

    fn calculate_spin(&mut self) {
        self.spin_velocity = self.spin_velocity.add_vector(self.spin_acceleration);
        let x_rotation: f64 = self.spin_velocity.x;
        let y_rotation: f64 = self.spin_velocity.y;
        let z_rotation: f64 = self.spin_velocity.z;

        let shape: &mut Vec<[f64; 3]> = &mut Vec::new();
        for point in &self.shape {
            let mut point: [f64; 3] = *point;
            point = self.rotate_x(point, x_rotation);
            point = self.rotate_y(point, y_rotation);
            point = self.rotate_z(point, z_rotation);
            shape.push(point);
        }
        self.shape = shape.to_vec();
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = Vector3D::new(x, y, z);
    }

    pub fn set_velocity(&mut self, x: f64, y: f64, z: f64) {
        self.velocity = Vector3D::new(x, y, z);
    }

    pub fn set_spin_velocity(&mut self, x: f64, y: f64, z: f64) {
        self.spin_velocity = Vector3D::new(x, y, z);
    }

    pub fn set_acceleration(&mut self, x: f64, y: f64, z: f64) {
        self.acceleration = Vector3D::new(x, y, z);
    }

    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn apply_attraction(&mut self, target: &Physics) {
        let mut force: Vector3D = target.position.subtract_vector(self.position);
        let distance: f64 = force.get_length();
        let g_const: f64 = 0.0001;
        let strength: f64 = g_const * ((self.mass * target.mass) / distance);
        force = force.set_magnitude(strength);
        force = force.divide(self.mass);
        self.acceleration = self.acceleration.add_vector(force);
        self.spin_acceleration = self.spin_acceleration.add_vector(force);
    }

    pub fn move_object(&mut self) {
        self.calculate_position();
        self.calculate_spin();
        self.acceleration = self.acceleration.multiply(0.0);
        self.spin_acceleration = self.spin_acceleration.multiply(0.0);
    }
}
