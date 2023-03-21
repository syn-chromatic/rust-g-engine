use rand::rngs::ThreadRng;
use rand::Rng;

use std::fmt::format;
use std::rc::Rc;
use std::time::Instant;
use std::vec;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
// use speedy2d::font::Font;
// use speedy2d::font::FormattedTextBlock;
// use speedy2d::font::TextLayout;
// use speedy2d::font::TextOptions;
use speedy2d::window::WindowHelper;
use speedy2d::Graphics2D;

use crate::body::{Body, BodyType};
use crate::camera::Camera;
use crate::debug;
use crate::particle::Particle;
use crate::physics::Physics;
use crate::shape::Shape;
use crate::vertices::ParticleCircle;
use crate::vertices::{CubeShape, SphereShape};
use crate::grid::GridGround;
use crate::text_writer::TextWriter;
use crate::text_writer::Font;
use crate::color::RGBA;

pub struct Simulation {
    pub camera: Camera,
    background_color: Color,
    center_point: (f64, f64),
    objects: Vec<BodyType>,
    timestep_hz: f64,
    text_writer: TextWriter,
}

impl Simulation {
    pub fn new(camera: Camera, canvas_resolution: (u32, u32)) -> Simulation {
        let objects: Vec<BodyType> = vec![];
        let timestep_hz: f64 = 5000.0;

        let background_color = Color::from_rgb(0.15, 0.15, 0.15);

        let center_x: f64 = canvas_resolution.0 as f64 / 2.0;
        let center_y: f64 = canvas_resolution.1 as f64 / 2.0;
        let center_point: (f64, f64) = (center_x, center_y);

        let fx: f32 = center_point.0 as f32 - 800.0;
        let fy: f32 = center_point.1 as f32 - 500.0;

        // let fps_txp: (f32, f32) = (fx, fy);
        // let fps_txc: Color = Color::from_rgb(1.0, 1.0, 1.0);

        let center_point = (0.0, 0.0);
        let font_type = format!("Arial");
        let font_style = format!("Bold");
        let font_color = RGBA::new(1.0, 1.0, 1.0, 1.0);
        let font = Font::new(font_type, 14, font_style, font_color, 1.8, 1);
        let text_writer = TextWriter::new(canvas_resolution.0, canvas_resolution.1, font);

        Simulation {
            camera,
            background_color,
            center_point,
            objects,
            timestep_hz,
            text_writer,

        }
    }

    pub fn add_center_cube(&mut self, x: f64, y: f64, z: f64) {
        let x: f64 = x + self.center_point.0;
        let y: f64 = y + self.center_point.1;
        let z: f64 = z;
        let mass: f64 = 100.0;

        let mut rng: ThreadRng = rand::thread_rng();
        let x_scale: f64 = 1.0;
        let y_scale: f64 = rng.gen_range(1.0..10.0);
        let z_scale: f64 = 1.0;

        let shape: Vec<[f64; 3]> = CubeShape::new(x_scale, y_scale, z_scale).get_shape();
        let color: Color = Color::from_rgb(1.0, 0.4, 0.4);
        let scale: f64 = 50.0;

        let mut shape: Shape = Shape::new(shape);
        shape.set_color(color);

        let physics: &mut Physics = shape.physics();
        physics.set_position(x, y, z);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Shape(shape));
    }

    pub fn add_center_sphere(&mut self) {
        let x: f64 = self.center_point.0;
        let y: f64 = self.center_point.1;
        let z: f64 = 0.0;
        let mass: f64 = 10_000_000.0;
        let shape: Vec<[f64; 3]> = SphereShape::new(20, 20, 20).get_shape();
        let color = Color::from_rgb(0.8, 0.3, 0.3);
        let scale: f64 = mass / 50_000.0;

        let mut shape: Shape = Shape::new(shape);
        shape.set_color(color);

        let physics: &mut Physics = shape.physics();
        physics.set_position(x, y, z);
        physics.set_mass(mass);
        physics.set_scale(scale);
        physics.set_spin_velocity(0.0, 0.0, 0.0);

        self.objects.push(BodyType::Shape(shape));
    }

    pub fn add_center_particle(&mut self) {
        let x: f64 = self.center_point.0;
        let y: f64 = self.center_point.1;
        let z: f64 = -10.0;
        let mass: f64 = 10_000_000.0;
        let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
        let color = Color::from_rgb(0.8, 0.3, 0.3);
        let scale: f64 = mass / 50_000.0;

        let mut particle: Particle = Particle::new(shape);
        particle.set_color(color);

        let physics: &mut Physics = particle.physics();
        physics.set_position(x, y, z);
        physics.set_mass(mass);
        physics.set_scale(scale);
        physics.set_spin_velocity(0.0, 0.0, 0.0);

        self.objects.push(BodyType::Particle(particle));
    }

    pub fn add_particle_t1(&mut self, z: f64) {
        let px = 1000.0 + self.center_point.0;
        let py = -200.0 + self.center_point.1;
        let pz = z - 5.0;

        let mass = 500_000_000_000_000.0;
        let shape = vec![[0.0, 0.0, 0.0]];
        let color = Color::from_rgb(0.9, 0.25, 0.25);
        let scale = 800.0;

        let vx = 10_000.0;
        let vy = -100_000.0;

        let mut p = Particle::new(shape);
        p.set_color(color);

        let physics: &mut Physics = p.physics();
        physics.set_position(px, py, pz);
        physics.set_velocity(vx, vy, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t15(&mut self, z: f64) {
        let px = -1000.0 + self.center_point.0;
        let py = 200.0 + self.center_point.1;
        let pz = z - 5.0;

        let mass = 500_000_000_000_000.0;
        let shape = vec![[0.0, 0.0, 0.0]];
        let color = Color::from_rgb(0.8, 0.3, 0.2);
        let scale = 800.0;

        let vx = -10_000.0;
        let vy = 100_000.0;

        let mut p = Particle::new(shape);
        p.set_color(color);

        let physics: &mut Physics = p.physics();
        physics.set_position(px, py, pz);
        physics.set_velocity(vx, vy, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t2(&mut self, z: f64) {
        let px = 500.0 + self.center_point.0;
        let py = 0.0 + self.center_point.1;
        let pz = z;

        let mass = 20.0;
        let shape = vec![[0.0, 0.0, 0.0]];
        let scale = 20.0;

        let vx = -5_000.0;
        let vy = 0.0;

        let mut p = Particle::new(shape);
        let physics: &mut Physics = p.physics();
        physics.set_position(px, py, pz);
        physics.set_velocity(vx, vy, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t3(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let x: f64 = self.center_point.0 - 0.0;
        let y: f64 = self.center_point.1 - 0.0;
        let z: f64 = 0.0;

        let mass: f64 = rng.gen_range(1.0..30.0);
        let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
        let scale: f64 = mass / 10.0;

        let mut p: Particle = Particle::new(shape);
        let physics: &mut Physics = p.physics();
        physics.set_position(x, y, z);
        physics.set_velocity(-10.0, -30.0, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t4(&mut self, z: f64) {
        let particles = ParticleCircle::new(150).generate(5_000.0, 10_000.0);

        for particle in particles {
            let px: f64 = particle[0] + self.center_point.0;
            let py: f64 = particle[1] + self.center_point.1;
            let pz: f64 = z;

            let mass: f64 = particle[2] * 10.0;
            let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];

            let mut rng: ThreadRng = rand::thread_rng();
            // let scale = rng.gen_range(1.0..1.0);
            let scale = 1.0;

            let mut p = Particle::new(shape);
            let physics: &mut Physics = p.physics();
            physics.set_position(px, py, pz);
            physics.set_mass(mass);
            physics.set_scale(scale);
            physics.set_velocity(10_000.0, 50_000.0, 50_000.0);
            self.objects.push(BodyType::Particle(p));
        }
    }

    pub fn add_particle_t45(&mut self, z: f64) {
        let particles = ParticleCircle::new(100).generate(-100.0, -1000.0);

        for particle in particles {
            let px: f64 = particle[0] + self.center_point.0;
            let py: f64 = particle[1] + self.center_point.1;
            let pz: f64 = z;

            let mass: f64 = particle[2];
            let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
            let scale = particle[2];

            let mut p = Particle::new(shape);
            let physics: &mut Physics = p.physics();
            physics.set_position(px, py, pz);
            physics.set_mass(mass);
            physics.set_scale(scale);
            self.objects.push(BodyType::Particle(p));
        }
    }

    pub fn add_grid(&mut self) {

        let mut grid: GridGround = GridGround::new(200, 200, 200.0);
        self.objects.push(BodyType::Grid(grid));

    }

    pub fn add_orbiting_object(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let x_rnd: f64 = rng.gen_range(200.0..400.0);
        let y_rnd: f64 = rng.gen_range(200.0..400.0);

        let x: f64 = self.center_point.0 - x_rnd;
        let y: f64 = self.center_point.1 - y_rnd;
        let z: f64 = -10.0;

        let mass: f64 = rng.gen_range(10.0..50.0);
        let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
        let scale: f64 = 1.0;

        let mut particle: Particle = Particle::new(shape);
        let physics: &mut Physics = particle.physics();

        physics.set_position(x, y, z);
        physics.set_velocity(100.0, 300.0, 1.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(particle));
    }

    pub fn setup_gravity_configuration(&mut self) {
        self.timestep_hz = 10.0;
        self.add_center_particle();

        for _ in 0..1000 {
            self.add_orbiting_object();
        }
    }

    pub fn setup_collision_configuration(&mut self) {
        self.timestep_hz = 5000.0;
        let z = 0.0;

        self.add_particle_t1(z);
        self.add_particle_t15(z);
        // self.add_particle_t4(z);
        // self.add_particle_t45(z);
        self.add_grid();

        let mut rng: ThreadRng = rand::thread_rng();


        for _ in 0..50 {
            let x: f64 = rng.gen_range(1.0..20_000.0);
            let y: f64 = 0.0;
            let z: f64 = rng.gen_range(1.0..20_000.0);
            self.add_center_cube(x, y, z);

        }




        // for _ in 0..1500 {
        //     self.add_particle_t3();
        // }
    }

    pub fn compute_objects(&mut self, graphics: &mut Graphics2D) {

        let timestep: f64 = 1.0 / self.timestep_hz;
        let mut objects_cl: Vec<BodyType> = self.objects.clone();
        for (i, pl1) in self.objects.iter_mut().enumerate() {
            let pl1_physics = pl1.physics();
            for (j, pl2) in objects_cl.iter_mut().enumerate() {
                if i == j {
                    continue;
                }

                let pl2_physics = pl2.physics();
                pl1_physics.apply_forces(pl2_physics, timestep);
            }
            // pl1_physics.update(timestep);
            pl1.draw(graphics, &mut self.camera);
        }

    }


    fn write_fps_text(&mut self, frame_time: f32) {
        let header_text = format!("Simulation information");
        let text = format!("{:.2} FPS", 1.0 / frame_time);
        self.text_writer.add_text_top_left(header_text, None);
        self.text_writer.add_text_top_left(text, None);

    }

    fn write_timestep_text(&mut self) {
        let khz = self.timestep_hz / 1000.0;
        let text = format!("Timestep:  {:.1} khz", khz);
        self.text_writer.add_text_top_left(text, None);
    }

    fn write_object_count(&mut self) {
        let object_count = self.objects.len();
        let text = format!("Objects:  {}", object_count);
        self.text_writer.add_text_top_left(text, None);
    }

    fn write_camera_information(&mut self) {
        let camera = &self.camera;
        let cp = camera.camera_position;
        let clt = camera.camera_target;
        let cld = camera.look_direction;
        let clu = camera.up_direction;
        let cls = camera.side_direction;

        let info_header = format!("Camera Information");
        let fov = format!("FOV:  {}", camera.fov);
        let near_plane = format!("Near Plane:  {}", camera.near_plane);
        let far_plane = format!("Far Plane:  {}", camera.far_plane);
        let yaw = format!("Yaw:  {}", camera.yaw);
        let pitch = format!("Pitch:  {}", camera.pitch);
        let position = format!("Position:  {:?}", cp.get_str());
        let target = format!("Target:  {:?}", clt.get_str());
        let look_dir = format!("Look (d):  {:?}", cld.get_str());
        let up_dir = format!("Up (d):  {:?}", clu.get_str());
        let side_dir = format!("Side (d):  {:?}", cls.get_str());

        self.text_writer.add_text_top_left("".to_string(), None);
        self.text_writer.add_text_top_left(info_header, None);
        self.text_writer.add_text_top_left(fov, None);
        self.text_writer.add_text_top_left(near_plane, None);
        self.text_writer.add_text_top_left(far_plane, None);
        self.text_writer.add_text_top_left(yaw, None);
        self.text_writer.add_text_top_left(pitch, None);
        self.text_writer.add_text_top_left(position, None);
        self.text_writer.add_text_top_left(target, None);
        self.text_writer.add_text_top_left(look_dir, None);
        self.text_writer.add_text_top_left(up_dir, None);
        self.text_writer.add_text_top_left(side_dir, None);
    }

    fn draw_text(&mut self, graphics: &mut Graphics2D) {
        self.text_writer.draw(graphics);
    }

    pub fn simulate(&mut self, graphics: &mut Graphics2D) {
        graphics.clear_screen(self.background_color);
        let frame_st: Instant = Instant::now();
        self.compute_objects(graphics);
        let frame_time: f32 = Instant::now().duration_since(frame_st).as_secs_f32();
        self.write_fps_text(frame_time);
        self.write_timestep_text();
        self.write_object_count();
        self.write_camera_information();
        self.draw_text(graphics);

    }
}
