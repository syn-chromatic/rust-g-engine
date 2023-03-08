use crate::vector_3d::Vector3D;

pub struct Camera {
    width: u32,
    height: u32,
    near_plane: f64,
    far_plane: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let near_plane = 60.0;
        let far_plane = 160.0;

        Camera {
            width,
            height,
            near_plane,
            far_plane,
        }
    }

    pub fn interpolate_radius(&self, position: Vector3D, radius: f64) -> f64 {
        let z = position.z;

        let interpolation_value = (z - self.near_plane) / (self.far_plane - self.near_plane);
        let radius_scaled = radius * interpolation_value;
        radius_scaled
    }

    pub fn perspective_projection(&self, position: Vector3D) -> Vector3D {
        let x = (position.x * self.near_plane) / position.z;
        let y = (position.y * self.near_plane) / position.z;

        let z: f64 = (self.far_plane + self.near_plane) / (self.near_plane - self.far_plane);
        let w: f64 = -position.z / (self.far_plane - self.near_plane);

        let xp: f64 = x * w;
        let yp: f64 = y * w;
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
