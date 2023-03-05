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

pub struct CubeShape {}

impl CubeShape {
    pub fn new() -> CubeShape {
        CubeShape {}
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
