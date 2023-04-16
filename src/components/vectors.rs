use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::f64::EPSILON;

#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    hash_value: Option<u64>,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let hash_value: Option<u64> = None;
        Vector3D {
            x,
            y,
            z,
            hash_value,
        }
    }

    pub fn default(value: f64) -> Vector3D {
        Vector3D::new(value, value, value)
    }

    pub fn to_string(&self) -> String {
        format!("[{:.2}, {:.2}, {:.2}]", self.x, self.y, self.z)
    }

    pub fn to_tuple(&self) -> (f64, f64, f64) {
        let x: f64 = self.x;
        let y: f64 = self.y;
        let z: f64 = self.z;
        (x, y, z)
    }
    pub fn to_array(&self) -> [f64; 3] {
        let x: f64 = self.x;
        let y: f64 = self.y;
        let z: f64 = self.z;
        [x, y, z]
    }

    pub fn get_hash(&mut self) -> u64 {
        if self.hash_value.is_some() {
            return self.hash_value.unwrap();
        }
        let mut hasher: DefaultHasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value: u64 = hasher.finish();
        self.hash_value = Some(hash_value);
        hash_value
    }

    pub fn abs(&self) -> Self {
        let x: f64 = self.x.abs();
        let y: f64 = self.y.abs();
        let z: f64 = self.z.abs();

        Vector3D::new(x, y, z)
    }

    pub fn clamp(&self, min_value: f64, max_value: f64) -> Self {
        let x: f64 = self.x.min(max_value).max(min_value);
        let y: f64 = self.y.min(max_value).max(min_value);
        let z: f64 = self.z.min(max_value).max(min_value);

        Vector3D::new(x, y, z)
    }

    pub fn add(&self, num: f64) -> Self {
        let x: f64 = self.x + num;
        let y: f64 = self.y + num;
        let z: f64 = self.z + num;

        Vector3D::new(x, y, z)
    }

    pub fn subtract(&self, num: f64) -> Self {
        let x: f64 = self.x - num;
        let y: f64 = self.y - num;
        let z: f64 = self.z - num;

        Vector3D::new(x, y, z)
    }

    pub fn multiply(&self, num: f64) -> Self {
        let x: f64 = self.x * num;
        let y: f64 = self.y * num;
        let z: f64 = self.z * num;

        Vector3D::new(x, y, z)
    }

    pub fn divide(&self, num: f64) -> Self {
        let x: f64 = self.x / num;
        let y: f64 = self.y / num;
        let z: f64 = self.z / num;

        Vector3D::new(x, y, z)
    }

    pub fn add_elements(&self, x: f64, y: f64, z: f64) -> Self {
        let x: f64 = self.x + x;
        let y: f64 = self.y + y;
        let z: f64 = self.z + z;

        Vector3D::new(x, y, z)
    }

    pub fn add_vector(&self, vec: &Self) -> Self {
        let x: f64 = self.x + vec.x;
        let y: f64 = self.y + vec.y;
        let z: f64 = self.z + vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn subtract_vector(&self, vec: &Self) -> Self {
        let x: f64 = self.x - vec.x;
        let y: f64 = self.y - vec.y;
        let z: f64 = self.z - vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn multiply_vector(&self, vec: &Self) -> Self {
        let x: f64 = self.x * vec.x;
        let y: f64 = self.y * vec.y;
        let z: f64 = self.z * vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn divide_vector(&self, vec: &Self) -> Self {
        let x: f64 = self.x / vec.x;
        let y: f64 = self.y / vec.y;
        let z: f64 = self.z / vec.z;

        Vector3D::new(x, y, z)
    }

    pub fn normalize(&self) -> Self {
        let length = self.get_length();
        if length == 0.0 {
            return Vector3D::new(0.0, 0.0, 0.0);
        }
        let x: f64 = self.x / length;
        let y: f64 = self.y / length;
        let z: f64 = self.z / length;

        Vector3D::new(x, y, z)
    }

    pub fn dot_product(&self, vec: &Self) -> f64 {
        let x: f64 = self.x * vec.x;
        let y: f64 = self.y * vec.y;
        let z: f64 = self.z * vec.z;

        x + y + z
    }

    pub fn cross_product(&self, vec: &Self) -> Self {
        let x: f64 = self.y * vec.z - self.z * vec.y;
        let y: f64 = self.z * vec.x - self.x * vec.z;
        let z: f64 = self.x * vec.y - self.y * vec.x;

        Vector3D::new(x, y, z)
    }

    pub fn set_magnitude(&self, magnitude: f64) -> Self {
        let length: f64 = self.get_length();

        let mut x: f64 = self.x;
        let mut y: f64 = self.y;
        let mut z: f64 = self.z;

        if length > 0.0 {
            x = (self.x / length) * magnitude;
            y = (self.y / length) * magnitude;
            z = (self.z / length) * magnitude;
        }

        Vector3D::new(x, y, z)
    }

    pub fn lerp_interpolation(&self, vec: &Self, t: f64) -> Self {
        let x: f64 = self.x + (vec.x - self.x) * t;
        let y: f64 = self.y + (vec.y - self.y) * t;
        let z: f64 = self.z + (vec.z - self.z) * t;

        Vector3D::new(x, y, z)
    }

    pub fn get_midpoint(&self, vec: &Self) -> Self {
        let x: f64 = (self.x + vec.x) / 2.0;
        let y: f64 = (self.y + vec.y) / 2.0;
        let z: f64 = (self.z + vec.z) / 2.0;

        Vector3D::new(x, y, z)
    }

    pub fn get_length_squared(&self) -> f64 {
        let x: f64 = self.x.powi(2);
        let y: f64 = self.y.powi(2);
        let z: f64 = self.z.powi(2);

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
        let x: f64 = (self.x - vec.x).powi(2);
        let y: f64 = (self.y - vec.y).powi(2);
        let z: f64 = (self.z - vec.z).powi(2);

        let distance: f64 = (x + y + z).sqrt();
        distance
    }

    pub fn get_sum(&self) -> f64 {
        let x: f64 = self.x;
        let y: f64 = self.y;
        let z: f64 = self.z;

        x + y + z
    }
}


impl Hash for Vector3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.x.to_bits());
        state.write_u64(self.y.to_bits());
        state.write_u64(self.z.to_bits());
    }
}


impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }

    // fn eq(&self, other: &Self) -> bool {
    //     self.x == other.x && self.y == other.y && self.z == other.z
    // }



}

impl Eq for Vector3D {}


impl PartialOrd for Vector3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.x - other.x).abs() < EPSILON {
            if (self.y - other.y).abs() < EPSILON {
                if (self.z - other.z).abs() < EPSILON {
                    Some(Ordering::Equal)
                } else {
                    self.z.partial_cmp(&other.z)
                }
            } else {
                self.y.partial_cmp(&other.y)
            }
        } else {
            self.x.partial_cmp(&other.x)
        }
    }
}