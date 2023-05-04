use crate::components::bvh::BVHNode;
use crate::components::color::RGBA;
use crate::components::decomposition::MeshDecompose;
use crate::components::polygons::Mesh;
use crate::components::shaders::Light;
use crate::components::vectors::Vector3D;

use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Physics {
    pub mesh: Mesh,
    pub mesh_cluster: Option<Vec<Mesh>>,
    pub position: Vector3D,
    pub velocity: Vector3D,
    pub acceleration: Vector3D,
    pub angular_velocity: Vector3D,
    pub angular_acceleration: Vector3D,
    pub moment_of_inertia: Vector3D,
    pub mass: f64,
    pub g_const: f64,
    pub gravity: f64,
    pub is_stationary: bool,
    pub last_contact_point: Option<Vector3D>,
}

impl Physics {
    pub fn new(mesh: Mesh, mesh_cluster: Option<Vec<Mesh>>) -> Physics {
        let position: Vector3D = Vector3D::default(0.0);
        let velocity: Vector3D = Vector3D::default(0.0);
        let acceleration: Vector3D = Vector3D::default(0.0);
        let angular_velocity: Vector3D = Vector3D::default(0.0);
        let angular_acceleration: Vector3D = Vector3D::default(0.0);
        let mass: f64 = 1.0;
        let moment_of_inertia: Vector3D = Self::get_moment_of_inertia(&mesh, mass);
        let g_const: f64 = 0.8;
        let gravity: f64 = -9.8;
        let is_stationary: bool = false;

        Physics {
            mesh,
            mesh_cluster,
            position,
            velocity,
            acceleration,
            angular_velocity,
            angular_acceleration,
            moment_of_inertia,
            mass,
            g_const,
            gravity,
            is_stationary,
            last_contact_point: None,
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = Vector3D::new(x, y, z);
    }

    pub fn set_velocity(&mut self, x: f64, y: f64, z: f64) {
        self.velocity = Vector3D::new(x, y, z);
    }

    pub fn set_angular_velocity(&mut self, x: f64, y: f64, z: f64) {
        self.angular_velocity = Vector3D::new(x, y, z);
    }

    pub fn set_acceleration(&mut self, x: f64, y: f64, z: f64) {
        self.acceleration = Vector3D::new(x, y, z);
    }

    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
        let mesh: &Mesh = &self.mesh;
        let moment_of_inertia: Vector3D = Self::get_moment_of_inertia(mesh, mass);
        self.moment_of_inertia = moment_of_inertia;
        println!("{:#?}", moment_of_inertia);
    }

    pub fn get_moment_of_inertia(mesh: &Mesh, mass: f64) -> Vector3D {
        let mut moment_of_inertia: Vector3D = Vector3D::default(0.0);
        let center_of_mass: Vector3D = mesh.get_center_of_mass();

        let num_tetrahedrons = mesh
            .polygons
            .iter()
            .filter(|p| p.get_vertices().len() == 3)
            .count();
        let tetrahedron_mass: f64 = mass / (num_tetrahedrons as f64);

        for polygon in &mesh.polygons {
            let vertices: &[Vector3D] = polygon.get_vertices();

            if vertices.len() > 3 {
                continue;
            }

            for i in 1..vertices.len() - 1 {
                let b: Vector3D = vertices[i].subtract_vector(&center_of_mass);
                let c: Vector3D = vertices[i + 1].subtract_vector(&center_of_mass);

                let ia: f64 =
                    (b.y * b.y + b.z * b.z + c.y * c.y + c.z * c.z + b.y * c.y + b.z * c.z) / 60.0;
                let ib: f64 = (b.x * c.x + b.x * c.z + b.z * c.x) / 120.0;
                let ic: f64 = (b.x * c.x + b.x * c.y + b.y * c.x) / 120.0;
                let id: f64 =
                    (b.x * b.x + b.z * b.z + c.x * c.x + c.z * c.z + b.x * c.x + b.z * c.z) / 60.0;
                let ie: f64 = (b.y * c.y + b.y * c.z + b.z * c.y) / 120.0;
                let ig: f64 =
                    (b.x * b.x + b.y * b.y + c.x * c.x + c.y * c.y + b.x * c.x + b.y * c.y) / 60.0;

                let tetrahedron_inertia: Vector3D = Vector3D::new(
                    tetrahedron_mass * (id + ig - 2.0 * ie),
                    tetrahedron_mass * (ia + ig - 2.0 * ic),
                    tetrahedron_mass * (ia + id - 2.0 * ib),
                );

                moment_of_inertia = moment_of_inertia.add_vector(&tetrahedron_inertia);
            }
        }

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

    pub fn freeze_velocity(&mut self) {
        self.velocity = Vector3D::default(0.0);
        self.acceleration = Vector3D::default(0.0);
    }

    fn apply_gravity(&mut self) {
        let mut acceleration = self.acceleration;
        acceleration.y = acceleration.y + self.gravity;
        self.acceleration = acceleration;
    }

    fn set_cluster_color(&mut self, color: RGBA) {
        if let Some(mesh_cluster) = self.mesh_cluster.as_mut() {
            for mesh in mesh_cluster.iter_mut() {
                mesh.set_uniform_color(color);
            }
        }
    }

    fn set_bounding_color(&mut self, target: &mut Physics, bounding_collided: bool) {
        if !bounding_collided {
            let color = RGBA::from_rgb(0.6, 0.6, 0.6);
            self.set_cluster_color(color);
            target.set_cluster_color(color);
        } else {
            let color = RGBA::from_rgb(0.8, 0.2, 0.2);
            self.set_cluster_color(color);
            target.set_cluster_color(color);
        }
    }

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

    fn get_mesh_from_idx(&self, idx: usize) -> &Mesh {
        if idx != 0 {
            if let Some(mesh_cluster) = &self.mesh_cluster {
                if mesh_cluster.len() > idx {
                    let mesh = &mesh_cluster[idx];
                    return mesh;
                }
            }
            println!("Physics intersection test fallback to main mesh.")
        }
        &self.mesh
    }

    fn get_ref_mesh_or_cluster(&self) -> Vec<&Mesh> {
        if let Some(mesh_cluster) = &self.mesh_cluster {
            return mesh_cluster.iter().collect();
        }

        let mut mesh_vec: Vec<&Mesh> = Vec::new();
        mesh_vec.push(&self.mesh);
        mesh_vec
    }

    fn get_mesh_or_cluster_idx_shift(&self) -> usize {
        if let Some(_) = &self.mesh_cluster {
            return 1;
        }
        0
    }

    pub fn get_bounding_collisions(&self, target: &Physics) -> Vec<(usize, usize)> {
        let mut collision_idxs: Vec<(usize, usize)> = Vec::new();
        let self_idx_shift: usize = self.get_mesh_or_cluster_idx_shift();
        let target_idx_shift: usize = target.get_mesh_or_cluster_idx_shift();

        let self_cluster: Vec<&Mesh> = self.get_ref_mesh_or_cluster();
        let target_cluster: Vec<&Mesh> = target.get_ref_mesh_or_cluster();

        for (self_idx, self_mesh) in self_cluster.iter().enumerate() {
            for (target_idx, target_mesh) in target_cluster.iter().enumerate() {
                let self_bvh: &BVHNode = &self_mesh.bvh_node;
                let target_bvh: &BVHNode = &target_mesh.bvh_node;
                let bounding_collided: bool = self_bvh.aabb_intersects(&target_bvh);
                if bounding_collided {
                    let self_idx: usize = self_idx + self_idx_shift;
                    let target_idx: usize = target_idx + target_idx_shift;
                    collision_idxs.push((self_idx, target_idx));
                }
            }
        }

        collision_idxs
    }

    pub fn apply_forces(&mut self, target: &mut Physics, timestep: f64) {
        // if !target.is_stationary {
        self.apply_collision(target, timestep);
        // self.apply_attraction(target);
        // }
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

    pub fn apply_collision(&mut self, target: &mut Physics, timestep: f64) {
        let bounding_collisions: Vec<(usize, usize)> = self.get_bounding_collisions(target);
        // self.set_bounding_color(target, bounding_collided);

        if bounding_collisions.is_empty() {
            return;
        }

        for (self_idx, target_idx) in bounding_collisions {
            let self_mesh: &Mesh = self.get_mesh_from_idx(self_idx);
            let target_mesh: &Mesh = target.get_mesh_from_idx(target_idx);

            let intersection: Option<(Vector3D, Vector3D)> =
                self_mesh.is_intersecting_bvh(target_mesh);

            if let Some((mtv, contact_point)) = intersection {
                self.last_contact_point = Some(contact_point);
                let direction: Vector3D = mtv.multiply(-1.0);
                let direction: Vector3D = direction.normalize();
                self.apply_collision_velocity(target, direction);
                self.apply_collision_angular_velocity(target, direction, contact_point);
                // let distance: f64 = self_mesh.get_distance_bvh(&target_mesh);
                let distance = mtv.get_length();

                if distance > 0.0 {
                    let distance = -distance - 100.0;
                    // let distance:f64 = distance - (1.0 / timestep);
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
        let self_vec: Vector3D = direction.multiply(distance);
        self.position = self.position.add_vector(&self_vec);
        self.update_mesh_position(&self_vec);
    }

    pub fn apply_collision_velocity(&mut self, target: &mut Physics, direction: Vector3D) {
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

    pub fn apply_collision_angular_velocity(
        &mut self,
        target: &mut Physics,
        direction: Vector3D,
        contact_point: Vector3D,
    ) {
        let e: f64 = 0.6;

        let m1_inv: f64 = 1.0 / self.mass;
        let m2_inv: f64 = 1.0 / target.mass;

        let r1: Vector3D = contact_point.subtract_vector(&self.get_center_of_mass());
        let r2: Vector3D = contact_point.subtract_vector(&target.get_center_of_mass());

        let angular_vel_r1: Vector3D = self.angular_velocity.cross_product(&r1);
        let angular_vel_r2: Vector3D = target.angular_velocity.cross_product(&r2);

        let v_rel1: Vector3D = self.velocity.add_vector(&angular_vel_r1);
        let v_rel2: Vector3D = target.velocity.add_vector(&angular_vel_r2);

        let v_rel_dir: f64 = (v_rel1.subtract_vector(&v_rel2)).dot_product(&direction);

        let inv_inertia1: Vector3D = self.moment_of_inertia.inverse();
        let inv_inertia2: Vector3D = target.moment_of_inertia.inverse();
        let neg_direction: Vector3D = direction.negate();

        let r1_cross_dir: Vector3D = r1.cross_product(&direction);
        let r2_cross_dir: Vector3D = r2.cross_product(&neg_direction);

        let r1_cross_dir_in1: Vector3D = r1_cross_dir.multiply_vector(&inv_inertia1);
        let r2_cross_dir_in2: Vector3D = r2_cross_dir.multiply_vector(&inv_inertia2);

        let in1_cross: Vector3D = r1_cross_dir_in1.cross_product(&r1);
        let in2_cross: Vector3D = r2_cross_dir_in2.cross_product(&r2);

        let eff_mass_rot1: f64 = in1_cross.dot_product(&direction);
        let eff_mass_rot2: f64 = in2_cross.dot_product(&neg_direction);

        let eff_mass_total: f64 = m1_inv + m2_inv + eff_mass_rot1 + eff_mass_rot2;

        let j: f64 = -(1.0 + e) * v_rel_dir / eff_mass_total;

        let impulse: Vector3D = direction.multiply(j);
        let impulse_neg: Vector3D = neg_direction.multiply(j);

        let angular_impulse1: Vector3D = r1.cross_product(&impulse);
        let angular_impulse2: Vector3D = r2.cross_product(&impulse_neg);

        let ang_vel_change1: Vector3D = angular_impulse1.multiply_vector(&inv_inertia1);
        let ang_vel_change2: Vector3D = angular_impulse2.multiply_vector(&inv_inertia2);

        self.angular_velocity = self.angular_velocity.add_vector(&ang_vel_change1);
        target.angular_velocity = target.angular_velocity.add_vector(&ang_vel_change2);

        self.angular_acceleration = inv_inertia1.multiply_vector(&angular_impulse1);
        target.angular_acceleration = inv_inertia2.multiply_vector(&angular_impulse2);
    }

    fn get_center_of_mass(&self) -> Vector3D {
        let mut center_of_mass: Vector3D = Vector3D::default(0.0);
        let mesh: &Mesh = &self.mesh;
        center_of_mass = center_of_mass.add_vector(&mesh.get_center_of_mass());

        center_of_mass
    }

    fn update_position(&mut self, timestep: f64) {
        let accel_change: Vector3D = self.acceleration.multiply(timestep);
        self.velocity = self.velocity.add_vector(&accel_change);

        let velocity_change: Vector3D = self.velocity.multiply(timestep);
        self.position = self.position.add_vector(&velocity_change);

        let angular_accel_change: Vector3D = self.angular_acceleration.multiply(timestep);
        self.angular_velocity = self.angular_velocity.add_vector(&angular_accel_change);

        let angular_velocity: Vector3D = self.angular_velocity.multiply(timestep);
        let rotation_axis: Vector3D = angular_velocity.normalize();
        let rotation_angle: f64 = angular_velocity.get_length();

        self.update_mesh_position(&velocity_change);
        self.update_mesh_rotation(&rotation_axis, rotation_angle);
        self.update_light_position(&velocity_change);
    }

    fn set_light_position(mesh: &mut Mesh, translation: &Vector3D) {
        if mesh.light.is_some() {
            let mut light: Light = mesh.light.unwrap();
            light.position = light.position.add_vector(translation);
            light.target = light.target.add_vector(translation);
            mesh.light = Some(light);
        }
    }

    fn update_light_position(&mut self, translation: &Vector3D) {
        Self::set_light_position(&mut self.mesh, translation);

        if let Some(mesh_cluster) = &mut self.mesh_cluster {
            for mesh in mesh_cluster {
                Self::set_light_position(mesh, translation)
            }
        }
    }

    fn update_mesh_rotation(&mut self, axis: &Vector3D, angle: f64) {
        let centroid: &Vector3D = &self.mesh.get_mesh_centroid();
        self.mesh.rotate_polygons_around_axis(axis, centroid, angle);

        if let Some(mesh_cluster) = &mut self.mesh_cluster {
            for mesh in mesh_cluster.iter_mut() {
                mesh.rotate_polygons_around_axis(axis, centroid, angle);
            }
        }
    }

    fn update_mesh_position_decompose(&mut self, translation: &Vector3D) {
        let mesh_decompose = MeshDecompose::new(1.0);
        self.mesh.translate_polygons(translation);

        let meshes = mesh_decompose.decompose(&self.mesh);
        self.mesh_cluster = Some(meshes);
    }

    fn update_mesh_position(&mut self, translation: &Vector3D) {
        self.mesh.translate_polygons(translation);

        if let Some(mesh_cluster) = &mut self.mesh_cluster {
            for mesh in mesh_cluster.iter_mut() {
                mesh.translate_polygons(translation);
            }
        }
    }

    fn reset_accelerations(&mut self) {
        self.acceleration = Vector3D::default(0.0);
        self.angular_acceleration = Vector3D::default(0.0);
    }

    fn handle_stationary_update(&mut self) {
        if self.is_stationary {
            self.acceleration = Vector3D::default(0.0);
            self.velocity = Vector3D::default(0.0);
        }
    }

    pub fn update(&mut self, timestep: f64) {
        self.handle_stationary_update();
        self.update_position(timestep);
        self.reset_accelerations();
    }
}
