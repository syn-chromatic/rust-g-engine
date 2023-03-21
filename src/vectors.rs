#[derive(Clone, Debug, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn get_str(&self) -> String {
        let x_string = format!("{:.2}", self.x);
        let y_string = format!("{:.2}", self.y);
        let z_string = format!("{:.2}", self.z);
        let vector_string = format!("[{}, {}, {}]", x_string, y_string, z_string);
        vector_string
    }

    pub fn get_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn multiply(&self, num: f64) -> Vector3D {
        Vector3D::new(self.x * num, self.y * num, self.z * num)
    }

    pub fn divide(&self, num: f64) -> Vector3D {
        Vector3D::new(self.x / num, self.y / num, self.z / num)
    }

    pub fn add_vector(&self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    pub fn subtract_vector(&self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    pub fn multiply_vector(self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self.x * vec.x, self.y * vec.y, self.z * vec.z)
    }

    pub fn get_midpoint(self, vec: Vector3D) -> Vector3D {
        let x_mid = (self.x + vec.x) / 2.0;
        let y_mid = (self.y + vec.y) / 2.0;
        let z_mid = (self.z + vec.z) / 2.0;
        Vector3D::new(x_mid, y_mid, z_mid)
    }

    pub fn get_length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn get_length(&self) -> f64 {
        let length_squared: f64 = self.get_length_squared();
        if length_squared <= 0.0 {
            return 0.0;
        }

        length_squared.sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let length: f64 = self.get_length();

        if length == 0.0 {
            return Vector3D::new(0.0, 0.0, 0.0);
        }
        Vector3D::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot_product(&self, vec: Vector3D) -> f64 {
        return (self.x * vec.x) + (self.y * vec.y) + (self.z * vec.z);
    }

    pub fn cross_product(&self, vec: Vector3D) -> Vector3D {
        let x: f64 = self.y * vec.z - self.z * vec.y;
        let y: f64 = self.z * vec.x - self.x * vec.z;
        let z: f64 = self.x * vec.y - self.y * vec.x;
        Vector3D::new(x, y, z)
    }

    pub fn set_magnitude(&self, magnitude: f64) -> Vector3D {
        let length: f64 = self.get_length();
        let mut x: f64 = self.x;
        let mut y: f64 = self.y;
        let mut z: f64 = self.z;

        if length != 0.0 {
            x = (x / length) * magnitude;
            y = (y / length) * magnitude;
            z = (z / length) * magnitude;
        }
        Vector3D::new(x, y, z)
    }
}
