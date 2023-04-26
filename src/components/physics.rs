use crate::components::color::RGBA;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::shaders::Light;
use crate::components::vectors::Vector3D;

use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Physics {
    pub mesh_cluster: Vec<Mesh>,
    pub position: Vector3D,
    pub velocity: Vector3D,
    pub acceleration: Vector3D,
    pub spin_velocity: Vector3D,
    pub spin_acceleration: Vector3D,
    pub mass: f64,
    pub scale: f64,
    pub g_const: f64,
    pub gravity: f64,
    pub is_stationary: bool,
}

impl Physics {
    pub fn new(mesh_cluster: Vec<Mesh>) -> Physics {
        let position: Vector3D = Vector3D::default(0.0);
        let velocity: Vector3D = Vector3D::default(0.0);
        let acceleration: Vector3D = Vector3D::default(0.0);
        let spin_velocity: Vector3D = Vector3D::default(0.0);
        let spin_acceleration: Vector3D = Vector3D::default(0.0);
        let mass: f64 = 1.0;
        let scale: f64 = 1.0;
        let g_const: f64 = 0.8;
        let gravity = -9.8;
        let is_stationary = false;

        Physics {
            mesh_cluster,
            position,
            velocity,
            acceleration,
            spin_velocity,
            spin_acceleration,
            mass,
            scale,
            g_const,
            gravity,
            is_stationary,
        }
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

    pub fn get_moment_of_inertia(&self) -> f64 {
        let moment_of_inertia: f64 = (2.0 / 5.0) * self.mass * self.scale.powi(2);
        moment_of_inertia
    }

    fn rotate_x(&self, xyz_point: &Vector3D, theta: f64) -> Vector3D {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = xyz_point.x;
        let y: f64 = cs * xyz_point.y - sn * xyz_point.z;
        let z: f64 = sn * xyz_point.y + cs * xyz_point.z;
        Vector3D::new(x, y, z)
    }

    fn rotate_y(&self, xyz_point: &Vector3D, theta: f64) -> Vector3D {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = cs * xyz_point.x + sn * xyz_point.z;
        let y: f64 = xyz_point.y;
        let z: f64 = -sn * xyz_point.x + cs * xyz_point.z;
        Vector3D::new(x, y, z)
    }

    fn rotate_z(&self, xyz_point: &Vector3D, theta: f64) -> Vector3D {
        let cs: f64 = theta.cos();
        let sn: f64 = theta.sin();
        let x: f64 = cs * xyz_point.x - sn * xyz_point.y;
        let y: f64 = sn * xyz_point.x + cs * xyz_point.y;
        let z: f64 = xyz_point.z;
        Vector3D::new(x, y, z)
    }

    fn constrain(&self, value: f64, min_value: f64, max_value: f64) -> f64 {
        f64::max(min_value, value).min(max_value)
    }

    fn get_random_direction(&self) -> Vector3D {
        println!("Retrieving Random Direction");
        let mut rng: ThreadRng = rand::thread_rng();
        let x_rnd: f64 = rng.gen_range(-1.0..1.0);
        let y_rnd: f64 = rng.gen_range(-1.0..1.0);
        let z_rnd: f64 = rng.gen_range(-1.0..1.0);

        let direction = Vector3D::new(x_rnd, y_rnd, z_rnd);
        direction
    }

    fn ensure_direction(&self, mut direction: Vector3D) -> Vector3D {
        if direction.get_length() == 0.0 {
            direction = self.get_random_direction();
        }
        direction
    }

    pub fn apply_forces(&mut self, target: &mut Physics, timestep: f64) {
        // if !target.is_stationary{
        self.apply_collision(target, timestep);
        // self.apply_attraction(target);
        // }
    }

    fn apply_gravity(&mut self) {
        let mut acceleration = self.acceleration;
        acceleration.y = acceleration.y + self.gravity;
        self.acceleration = acceleration;
    }

    pub fn apply_attraction(&mut self, target: &mut Physics) {
        let tts_distance: Vector3D = target.position.subtract_vector(&self.position);
        let distance: f64 = tts_distance.get_length();

        if distance > 0.0 {
            let strength: f64 = self.g_const * (self.mass * target.mass) / distance.powi(2);
            let force: Vector3D = tts_distance.set_magnitude(strength);
            let acceleration1: Vector3D = force.divide(self.mass);
            let acceleration2: Vector3D = force.divide(-target.mass);

            self.acceleration = self.acceleration.add_vector(&acceleration1);
            target.acceleration = target.acceleration.add_vector(&acceleration2);
        }
    }

    pub fn freeze_velocity(&mut self) {
        self.velocity = Vector3D::default(0.0);
        self.acceleration = Vector3D::default(0.0);
    }

    pub fn get_bounding_collision(&self, target: &Physics) -> Vec<(usize, usize)> {
        let mut collision_idxs: Vec<(usize, usize)> = Vec::new();
        for (self_idx, self_mesh) in self.mesh_cluster.iter().enumerate() {
            for (target_idx, target_mesh) in target.mesh_cluster.iter().enumerate() {
                let self_bvh = &self_mesh.bvh_node;
                let target_bvh = &target_mesh.bvh_node;
                let bounding_collided: bool = self_bvh.aabb_intersects(&target_bvh);
                if bounding_collided {
                    collision_idxs.push((self_idx, target_idx));
                }
            }
        }
        collision_idxs
    }

    // fn set_bounding_color(&mut self, target: &mut Physics, bounding_collided: bool) {
    //     if !bounding_collided {
    //         let color = RGBA::from_rgb(0.6, 0.6, 0.6);
    //         self.mesh.set_uniform_color(color);
    //         target.mesh.set_uniform_color(color);
    //     } else {
    //         let color = RGBA::from_rgb(0.8, 0.2, 0.2);
    //         self.mesh.set_uniform_color(color);
    //         target.mesh.set_uniform_color(color);
    //     }
    // }

    fn get_mass_ratio(&self, target: &Physics) -> f64 {
        if self.mass > target.mass {
            let mass_ratio: f64 = target.mass / self.mass;
            return mass_ratio;
        } else if self.mass < target.mass {
            let mass_ratio: f64 = self.mass / target.mass;
            let mass_ratio: f64 = 1.0 - mass_ratio;
            return mass_ratio;
        }
        0.5
    }

    pub fn apply_collision(&mut self, target: &mut Physics, timestep: f64) {
        let bounding_collisions: Vec<(usize, usize)> = self.get_bounding_collision(target);

        if bounding_collisions.is_empty() {
            return;
        }

        for (self_idx, target_idx) in bounding_collisions {
            let self_mesh = &self.mesh_cluster[self_idx].clone();
            let target_mesh = &target.mesh_cluster[target_idx].clone();

            let self_mtv: Option<Vector3D> = self_mesh.is_intersecting_bvh(target_mesh);

            if let Some(mtv) = self_mtv {
                let direction: Vector3D = mtv.multiply(-1.0);
                let direction: Vector3D = direction.normalize();
                self.apply_collision_velocities(target, direction);
                let distance: f64 = self_mesh.get_distance(&target_mesh);

                if distance < 0.0 {
                    let direction: Vector3D = self.ensure_direction(direction);
                    let mass_ratio: f64 = self.get_mass_ratio(target);
                    let self_correction: f64 = distance * mass_ratio;
                    let target_correction: f64 = distance - self_correction;

                    self.apply_shift_correction(direction, -self_correction);
                    target.apply_shift_correction(direction, target_correction);
                }
            }
        }
    }

    pub fn apply_shift_correction(&mut self, direction: Vector3D, distance: f64) {
        let self_vec = direction.multiply(distance);
        self.position = self.position.add_vector(&self_vec);
        self.update_mesh_position(&self_vec);
    }

    pub fn apply_collision_velocities(&mut self, target: &mut Physics, direction: Vector3D) {
        let e: f64 = 0.6;
        let v1i: f64 = self.velocity.dot_product(&direction);
        let v2i: f64 = target.velocity.dot_product(&direction);

        let m1: f64 = self.mass;
        let m2: f64 = target.mass;

        let v1f: f64 = ((m1 - e * m2) * v1i + (1.0 + e) * m2 * v2i) / (m1 + m2);
        let v2f: f64 = ((m2 - e * m1) * v2i + (1.0 + e) * m1 * v1i) / (m1 + m2);

        let v1f_vec: Vector3D = direction.multiply(v1f);
        let v2f_vec: Vector3D = direction.multiply(v2f);

        let v1i_vec: Vector3D = direction.multiply(v1i);
        let v2i_vec: Vector3D = direction.multiply(v2i);

        let v1p: Vector3D = self.velocity.subtract_vector(&v1i_vec);
        let v2p: Vector3D = target.velocity.subtract_vector(&v2i_vec);

        let v1: Vector3D = v1p.add_vector(&v1f_vec);
        let v2: Vector3D = v2p.add_vector(&v2f_vec);

        self.velocity = v1;
        target.velocity = v2;
    }

    // fn apply_spin_forces(&mut self, timestep: f64) {
    //     let timestep_spin_acc = self.spin_acceleration.multiply(timestep);
    //     self.spin_velocity = self.spin_velocity.add_vector(&timestep_spin_acc);
    //     let x_rotation = self.spin_velocity.x * timestep;
    //     let y_rotation = self.spin_velocity.y * timestep;
    //     let z_rotation = self.spin_velocity.z * timestep;

    //     let mut mesh_polygons = self.mesh.polygons.clone();
    //     for polygon in &mut mesh_polygons {
    //         match polygon {
    //             Polygon::Triangle(triangle) => {
    //                 for idx in 0..triangle.vertices.len() {
    //                     let vertex = triangle.vertices[idx];
    //                     let rotated_vertex = self.rotate_x(&vertex, x_rotation);
    //                     let rotated_vertex = self.rotate_y(&rotated_vertex, y_rotation);
    //                     let rotated_vertex = self.rotate_z(&rotated_vertex, z_rotation);
    //                     triangle.vertices[idx] = rotated_vertex;
    //                 }
    //             }
    //             Polygon::Quad(quad) => {
    //                 for idx in 0..quad.vertices.len() {
    //                     let vertex = quad.vertices[idx];
    //                     let rotated_vertex = self.rotate_x(&vertex, x_rotation);
    //                     let rotated_vertex = self.rotate_y(&rotated_vertex, y_rotation);
    //                     let rotated_vertex = self.rotate_z(&rotated_vertex, z_rotation);
    //                     quad.vertices[idx] = rotated_vertex;
    //                 }
    //             }
    //         }
    //     }
    //     self.mesh.polygons = mesh_polygons;
    // }

    fn update_position(&mut self, timestep: f64) {
        let acceleration_change = self.acceleration.multiply(timestep);
        self.velocity = self.velocity.add_vector(&acceleration_change);

        let velocity_change = self.velocity.multiply(timestep);
        self.position = self.position.add_vector(&velocity_change);

        self.update_light_position(&velocity_change);
        self.update_mesh_position(&velocity_change);
    }

    fn update_light_position(&mut self, translation: &Vector3D) {
        for mesh in self.mesh_cluster.iter_mut() {
            if mesh.light.is_some() {
                let mut light: Light = mesh.light.unwrap();
                light.position = light.position.add_vector(translation);
                light.target = light.target.add_vector(translation);
                mesh.light = Some(light);
            }
        }
    }

    fn update_mesh_position(&mut self, translation: &Vector3D) {
        for mesh in self.mesh_cluster.iter_mut() {
            mesh.translate_polygons(translation);
        }
    }

    pub fn update(&mut self, timestep: f64) {
        if self.is_stationary {
            self.acceleration = Vector3D::default(0.0);
            self.velocity = Vector3D::default(0.0);
        }

        self.update_position(timestep);
        self.acceleration = Vector3D::default(0.0);
    }
}
