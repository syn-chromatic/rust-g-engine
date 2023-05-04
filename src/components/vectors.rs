use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::f64::EPSILON;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    hash_value: Option<u64>,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D {
            x,
            y,
            z,
            hash_value: None,
        }
    }

    pub fn from_array(array: [f64; 3]) -> Self {
        let x: f64 = array[0];
        let y: f64 = array[1];
        let z: f64 = array[2];

        Vector3D {
            x,
            y,
            z,
            hash_value: None,
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

    pub fn inverse(&self) -> Vector3D {
        let x: f64 = 1.0 / self.x;
        let y: f64 = 1.0 / self.y;
        let z: f64 = 1.0 / self.z;

        Vector3D::new(x, y, z)
    }

    pub fn negate(&self) -> Vector3D {
        let x: f64 = -self.x;
        let y: f64 = -self.y;
        let z: f64 = -self.z;

        Vector3D::new(x, y, z)
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

    pub fn component_min(&self, other: &Self) -> Self {
        Vector3D::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn component_max(&self, other: &Self) -> Self {
        Vector3D::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
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

    pub fn get_angle_between(&self, other: &Vector3D) -> f64 {
        let dot_product: f64 = self.dot_product(other);
        let magnitude_product: f64 = self.get_length() * other.get_length();
        let cosine_angle: f64 = dot_product / magnitude_product;

        let clamped_cosine: f64 = cosine_angle.max(-1.0).min(1.0);
        let angle: f64 = clamped_cosine.acos();

        angle
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

    pub fn rotate_around_axis(&self, axis: &Vector3D, angle: f64) -> Self {
        let half_angle = angle / 2.0;
        let sin_half_angle = half_angle.sin();
        let cos_half_angle = half_angle.cos();

        let q = Quaternion::new(
            cos_half_angle,
            axis.x * sin_half_angle,
            axis.y * sin_half_angle,
            axis.z * sin_half_angle,
        );

        let p = Quaternion::new(0.0, self.x, self.y, self.z);
        let q_conjugate = q.conjugate();

        let rotated_p = q.multiply(&p).multiply(&q_conjugate);
        Vector3D::new(rotated_p.x, rotated_p.y, rotated_p.z)
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
    // fn eq(&self, other: &Self) -> bool {
    //     (self.x - other.x).abs() < EPSILON
    //         && (self.y - other.y).abs() < EPSILON
    //         && (self.z - other.z).abs() < EPSILON
    // }

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector3D {}

// impl PartialOrd for Vector3D {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if (self.x - other.x).abs() <= EPSILON {
//             if (self.y - other.y).abs() <= EPSILON {
//                 if (self.z - other.z).abs() <= EPSILON {
//                     Some(Ordering::Equal)
//                 } else {
//                     self.z.partial_cmp(&other.z)
//                 }
//             } else {
//                 self.y.partial_cmp(&other.y)
//             }
//         } else {
//             self.x.partial_cmp(&other.x)
//         }
//     }
// }

impl PartialOrd for Vector3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x {
            if self.y == other.y {
                if self.z == other.z {
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

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    pub fn multiply(&self, other: &Quaternion) -> Self {
        let w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        let x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        let y = self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x;
        let z = self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w;

        Self::new(w, x, y, z)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.w, -self.x, -self.y, -self.z)
    }
}
