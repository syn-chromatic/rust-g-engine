#[derive(Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn multiply(&self, num: f64) -> Vector3D {
        Vector3D::new(self.x * num, self.y * num, self.z * num)
    }

    pub fn divide(&self, num: f64) -> Vector3D {
        Vector3D::new(self.x / num, self.y / num, self.z / num)
    }

    pub fn add_vector(&self, vec: &Vector3D) -> Vector3D {
        Vector3D::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    pub fn subtract_vector(&self, vec: &Vector3D) -> Vector3D {
        Vector3D::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    pub fn get_length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn set_magnitude(&mut self, magnitude: f64) -> &mut Vector3D {
        let length: f64 = self.get_length();
        if length != 0.0 {
            self.x = (self.x / length) * magnitude;
            self.y = (self.y / length) * magnitude;
            self.z = (self.z / length) * magnitude;
        }
        self
    }
}
