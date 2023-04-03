#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn to_string(&self) -> String {
        format!("[{:.2}, {:.2}, {:.2}]", self.x, self.y, self.z)
    }

    pub fn to_tuple(&self) -> (f64, f64, f64) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        (x, y, z)
    }
    pub fn to_vec(&self) -> [f64; 3] {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        [x, y, z]
    }

    pub fn clamp(&self, min_value: f64, max_value: f64) -> Self {
        let x = self.x.min(max_value).max(min_value);
        let y = self.y.min(max_value).max(min_value);
        let z = self.z.min(max_value).max(min_value);

        Vector3D::new(x, y, z)
    }

    pub fn add(&self, num: f64) -> Self {
        let x = self.x + num;
        let y = self.y + num;
        let z = self.z + num;

        Vector3D::new(x, y, z)
    }

    pub fn subtract(&self, num: f64) -> Self {
        let x = self.x - num;
        let y = self.y - num;
        let z = self.z - num;

        Vector3D::new(x, y, z)
    }

    pub fn multiply(&self, num: f64) -> Self {
        let x = self.x * num;
        let y = self.y * num;
        let z = self.z * num;

        Vector3D::new(x, y, z)
    }

    pub fn divide(&self, num: f64) -> Self {
        let x = self.x / num;
        let y = self.y / num;
        let z = self.z / num;

        Vector3D::new(x, y, z)
    }

    pub fn add_vector(&self, vec: &Self) -> Self {
        let x = self.x + vec.x;
        let y = self.y + vec.y;
        let z = self.z + vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn subtract_vector(&self, vec: &Self) -> Self {
        let x = self.x - vec.x;
        let y = self.y - vec.y;
        let z = self.z - vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn multiply_vector(&self, vec: &Self) -> Self {
        let x = self.x * vec.x;
        let y = self.y * vec.y;
        let z = self.z * vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn divide_vector(&self, vec: &Self) -> Self {
        let x = self.x / vec.x;
        let y = self.y / vec.y;
        let z = self.z / vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn normalize(&self) -> Self {
        let length = self.get_length();
        if length == 0.0 {
            return Vector3D::new(0.0, 0.0, 0.0);
        }
        let x = self.x / length;
        let y = self.y / length;
        let z = self.z / length;

        Vector3D::new(x, y, z)
    }

    pub fn dot_product(&self, vec: &Self) -> f64 {
        let x = self.x * vec.x;
        let y = self.y * vec.y;
        let z = self.z * vec.z;

        x + y + z
    }

    pub fn cross_product(&self, vec: &Self) -> Self {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;

        Vector3D::new(x, y, z)
    }

    pub fn set_magnitude(&self, magnitude: f64) -> Self {
        let length = self.get_length();

        let x = self.x;
        let y = self.y;
        let z = self.z;

        if length > 0.0 {
            let x = (self.x / length) * magnitude;
            let y = (self.y / length) * magnitude;
            let z = (self.z / length) * magnitude;
        }
        Vector3D::new(x, y, z)
    }

    pub fn lerp_interpolation(&self, vec: &Self, t: f64) -> Self {
        let x = self.x + (vec.x - self.x) * t;
        let y = self.y + (vec.y - self.y) * t;
        let z = self.z + (vec.z - self.z) * t;

        Vector3D::new(x, y, z)
    }

    pub fn get_midpoint(&self, vec: &Self) -> Self {
        let x = (self.x + vec.x) / 2.0;
        let y = (self.y + vec.y) / 2.0;
        let z = (self.z + vec.z) / 2.0;

        Vector3D::new(x, y, z)
    }

    pub fn get_length_squared(&self) -> f64 {
        let x = self.x.powi(2);
        let y = self.y.powi(2);
        let z = self.z.powi(2);

        x + y + z
    }

    pub fn get_length(&self) -> f64 {
        let length_squared = self.get_length_squared();
        if length_squared == 0.0 {
            return 0.0;
        }
        length_squared.sqrt()
    }

    pub fn get_distance(&self, vec: &Self) -> f64 {
        let x = (self.x - vec.x).powi(2);
        let y = (self.y - vec.y).powi(2);
        let z = (self.z - vec.z).powi(2);

        let distance = (x + y + z).sqrt();
        distance
    }
}
