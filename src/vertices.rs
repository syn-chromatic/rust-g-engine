use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub struct SphereShape {
    long_num: i32,
    lat_num: i32,
    points: i32,
}

impl SphereShape {
    pub fn new(long_num: i32, lat_num: i32, points: i32) -> SphereShape {
        SphereShape {
            long_num,
            lat_num,
            points,
        }
    }

    fn create_long_ponts(&self, vertices: &mut Vec<[f64; 3]>) {
        for i in 0..self.long_num {
            for j in 0..self.lat_num {
                let fi: f64 = i as f64;
                let fj: f64 = j as f64;
                let long_fnum: f64 = self.long_num as f64;
                let lat_fnum: f64 = self.lat_num as f64;
                let theta: f64 = 2.0 * PI * fi / long_fnum;
                let phi: f64 = PI * fj / (lat_fnum - 1.0);

                let x: f64 = 1.0 * phi.sin() * theta.cos();
                let y: f64 = 1.0 * phi.sin() * theta.sin();
                let z: f64 = 1.0 * phi.cos();

                let point: [f64; 3] = [x, y, z];

                vertices.push(point);
            }
        }
    }

    fn create_lat_points(&self, vertices: &mut Vec<[f64; 3]>) {
        for i in 0..self.points {
            let fi: f64 = i as f64;
            let pointsf: f64 = self.points as f64;
            let theta = PI * fi / pointsf;
            for j in 0..self.points + 1 {
                let fj: f64 = j as f64;
                let phi = 2.0 * PI * fj / pointsf;
                let x = 1.0 * theta.sin() * phi.cos();
                let y = 1.0 * theta.sin() * phi.sin();
                let z = 1.0 * theta.cos();

                let point: [f64; 3] = [x, y, z];
                vertices.push(point);
            }
        }
    }

    pub fn get_shape(&self) -> Vec<[f64; 3]> {
        let mut vertices: Vec<[f64; 3]> = vec![];
        self.create_long_ponts(&mut vertices);
        self.create_lat_points(&mut vertices);
        vertices
    }
}

pub struct CubeShape {
    scale: [f64; 3],
}

impl CubeShape {
    pub fn new(scale_x: f64, scale_y: f64, scale_z: f64) -> CubeShape {
        CubeShape {
            scale: [scale_x, scale_y, scale_z],
        }
    }

    pub fn get_shape(&self) -> Vec<[f64; 3]> {
        let shape: Vec<[f64; 3]> = vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [1.0, 1.0, -1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, -1.0, 1.0],
        ];
        shape
            .into_iter()
            .map(|vertex| {
                [
                    vertex[0] * self.scale[0],
                    vertex[1] * self.scale[1],
                    vertex[2] * self.scale[2],
                ]
            })
            .collect()
    }
}

pub struct ParticleCircle {
    circle_radius: i32,
}

impl ParticleCircle {
    pub fn new(circle_radius: i32) -> ParticleCircle {
        ParticleCircle { circle_radius }
    }

    pub fn generate(&self, px: f64, py: f64) -> Vec<[f64; 3]> {
        let mut particles: Vec<[f64; 3]> = vec![];
        let mut max_particle_size: f64 = 0.0;

        let mut circle_radius = self.circle_radius;

        while circle_radius > 1 {
            let size: f64 = 2.0;
            let angle_increment: f64 = (2.0 * PI / circle_radius as f64);

            for i in 0..circle_radius {
                let angle: f64 = i as f64 * angle_increment;
                let x: f64 = px + circle_radius as f64 * angle.cos();
                let y: f64 = py + circle_radius as f64 * angle.sin();

                if size > max_particle_size {
                    max_particle_size = size;
                }
                let particle: [f64; 3] = [x, y, size];
                particles.push(particle);
            }
            circle_radius -= (max_particle_size * PI) as i32;
            max_particle_size = 0.0;
        }
        particles
    }
}

// pub struct ParticleGalaxy {
//     num_particles: i32,
//     radius: f32,
//     center_x: f32,
//     center_y: f32,
//     particle_size: f32,
//     k: f32,
// }

// impl ParticleGalaxy {
//     pub fn new(num_particles: i32, radius: f32, center_x: f32, center_y: f32, particle_size: f32, k: f32) -> ParticleGalaxy {
//         ParticleGalaxy {num_particles, radius, center_x, center_y, particle_size, k}
//     }

//     pub fn generate(&self) {

//     }

// }

pub struct ParticleGalaxy {
    galaxy_radius: i32,
    arms: i32,
    particles_per_arm: i32,
    arm_rotation_speed: f64,
    arm_length_decay: f64,
}

impl ParticleGalaxy {
    pub fn new(
        galaxy_radius: i32,
        arms: i32,
        particles_per_arm: i32,
        arm_rotation_speed: f64,
        arm_length_decay: f64,
    ) -> ParticleGalaxy {
        ParticleGalaxy {
            galaxy_radius,
            arms,
            particles_per_arm,
            arm_rotation_speed,
            arm_length_decay,
        }
    }

    pub fn generate(&self, px: f64, py: f64) -> Vec<[f64; 5]> {
        let mut particles: Vec<[f64; 5]> = vec![];
        let mut max_particle_size: f64 = 0.0;

        for arm_index in 0..self.arms {
            let mut arm_angle: f64 = (arm_index as f64 / self.arms as f64) * 2.0 * PI;
            let mut arm_length: f64 = self.galaxy_radius as f64;

            for _ in 0..self.particles_per_arm {
                let size: f64 = 2.0;
                let arm_angle_cos = arm_angle.cos();
                let arm_angle_sin = arm_angle.sin();
                let x: f64 = px + arm_length * arm_angle_cos;
                let y: f64 = py + arm_length * arm_angle_sin;

                if size > max_particle_size {
                    max_particle_size = size;
                }
                let particle: [f64; 5] = [x, y, size, arm_angle_cos, arm_angle_sin];
                particles.push(particle);

                arm_angle += self.arm_rotation_speed;
                arm_length *= self.arm_length_decay;
            }
        }

        particles
    }
}
