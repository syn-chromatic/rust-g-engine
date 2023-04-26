use crate::abstracts::body::{Body, BodyType};
use crate::components::camera::Camera;
use crate::components::color::RGBA;

use super::vectors::Vector3D;
use crate::components::font::ArialFont;
use crate::components::font::FontSettings;
use crate::components::font::FontType;
use crate::components::graphics::Graphics;
use crate::components::physics::Physics;
use crate::components::polygons::Mesh;
use crate::components::shape::Shape;
use crate::components::text_writer::TextWriter;
use crate::components::vertices::Sphere;
use crate::configurations::body_configurations;

pub struct Simulation {
    pub camera: Camera,
    pub objects: Vec<BodyType>,
    pub polygon_count: usize,
    pub draw_polygons: bool,
    pub draw_mesh: bool,
    timestep_hz: f64,
    text_writer: TextWriter,
}

impl Simulation {
    pub fn new(camera: Camera, resolution: (u32, u32)) -> Simulation {
        let objects: Vec<BodyType> = vec![];
        let timestep_hz: f64 = 5.0;
        let polygon_count: usize = 0;

        let arial_font: ArialFont = ArialFont::new();
        let font_type: FontType = FontType::ArialFont(arial_font);
        let font_color: RGBA = RGBA::new(1.0, 1.0, 1.0, 1.0);
        let font: FontSettings = FontSettings::new(font_type, 14, font_color, 1.8, 1);
        let text_writer: TextWriter = TextWriter::new(resolution, font);

        Simulation {
            camera,
            objects,
            polygon_count,
            draw_polygons: true,
            draw_mesh: false,
            timestep_hz,
            text_writer,
        }
    }

    pub fn toggle_draw_polygons(&mut self) {
        self.draw_polygons = !self.draw_polygons;
        self.draw_mesh = !self.draw_mesh;
    }

    pub fn setup_objects(&mut self) {
        // self.timestep_hz = 10000.0;
        // let z = 0.0;

        // let grid = body_configurations::get_grid();
        // self.objects.push(grid);

        // let grid_cuboid = body_configurations::get_grid_cuboid();
        // self.objects.push(grid_cuboid);

        // let obj = body_configurations::get_obj("./assets/town_square.obj");
        // self.objects.push(obj);

        // let obj = body_configurations::get_obj("./assets/cottage.obj");
        // self.objects.push(obj);

        // let obj = body_configurations::get_obj("./assets/plane.obj");
        // self.objects.push(obj);

        // let obj = body_configurations::get_obj("./assets/TerrorTubby_01.obj");
        // self.objects.push(obj);

        // let sphere = body_configurations::get_sphere_light_highmass();
        // self.objects.push(sphere);

        // let sphere = body_configurations::get_sphere_light1();
        // self.objects.push(sphere);

        // let sphere = body_configurations::get_sphere_light2();

        // let sphere = body_configurations::get_sphere_light3();
        // self.objects.push(sphere);

        // for i in 0..100 {
        //     let sphere = body_configurations::get_sphere_light3();
        //     self.objects.push(sphere);
        // }

        let system = body_configurations::orbiting_system(Vector3D::new(0.0, 0.0, 0.0));
        self.objects.extend(system);

        // let system =
        //     body_configurations::orbiting_system2(Vector3D::new(8_000_000.0, 4_000_000.0, 0.0));
        // self.objects.extend(system);

        let camera_position = Vector3D::new(-250_000.0, 200.0, -2_000_000.0);
        self.camera.set_camera_position(camera_position);

        for object in self.objects.iter_mut() {
            let physics = object.physics();
            let mesh_cluster = &physics.mesh_cluster;
            for mesh in mesh_cluster {
                let polygon_len = mesh.polygons.len();
                self.polygon_count += polygon_len;
            }
        }
    }

    pub fn increment_timestep(&mut self, direction: i32) {
        let min_timestep_hz: f64 = 0.1;
        let max_timestep_hz: f64 = 100.0 * 1000.0;
        let proportion: f64 = 0.05;
        let increment: f64 = self.timestep_hz * proportion * direction as f64;
        let timestep: f64 = self.timestep_hz + increment;
        let timestep: f64 = timestep.max(min_timestep_hz);
        let timestep: f64 = timestep.min(max_timestep_hz);
        self.timestep_hz = timestep;
    }

    pub fn shoot(&mut self) {
        let camera_position: Vector3D = self.camera.camera_position;
        let camera_target: Vector3D = self.camera.camera_target;

        let camera_dir: Vector3D = camera_target.subtract_vector(&camera_position);
        let camera_dir: Vector3D = camera_dir.normalize().multiply(-1.0);

        let mass = 1_000_000.0;

        let mut sphere = Sphere::new(50_000.0, 10, 10);
        // let mut sphere = Cuboid::new(50_000.0, 50_000.0, 50_000.0);
        sphere.set_offset(camera_position.x, camera_position.y, camera_position.z);
        sphere.set_color(RGBA::from_random());
        sphere.set_shader(RGBA::from_rgb(0.5, 0.5, 0.5));

        let mesh = sphere.get_triangle_mesh();
        let mut meshes: Vec<Mesh> = Vec::new();
        meshes.push(mesh);

        let mut body = Shape::new(meshes);
        body.physics()
            .set_position(camera_position.x, camera_position.y, camera_position.z);
        body.physics().set_mass(mass);
        let velocity = camera_dir.multiply(200_000.0);
        //  let velocity = camera_dir.multiply(1.0);
        body.physics()
            .set_velocity(velocity.x, velocity.y, velocity.z);
        let body_type = BodyType::Shape(body);
        self.objects.push(body_type);
    }

    pub fn compute_objects(&mut self) {
        let timestep: f64 = 1.0 / self.timestep_hz;
        let objects = &mut self.objects;

        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                let (physics1, physics2) = {
                    let (left, right) = objects.split_at_mut(j);
                    (left[i].physics(), right[0].physics())
                };

                physics1.apply_forces(physics2, timestep);
            }
        }

        for object in self.objects.iter_mut() {
            let physics: &mut Physics = object.physics();
            physics.update(timestep);
        }
    }

    fn get_timestep_text(&self) -> String {
        if self.timestep_hz >= 1000.0 {
            let khz = self.timestep_hz / 1000.0;
            let text = format!("Timestep:  {:.2} khz", khz);
            return text;
        }
        let khz = self.timestep_hz;
        let text = format!("Timestep:  {:.2} hz", khz);
        text
    }

    fn write_fps_text(&mut self, fps: f64) {
        let header_text = format!("Engine information");
        let text = format!("{:.2} FPS", fps);
        self.text_writer.add_text_top_left(header_text, None);
        self.text_writer.add_text_top_left(text, None);
    }

    fn write_timestep_text(&mut self) {
        let text = self.get_timestep_text();
        self.text_writer.add_text_top_left(text, None);
    }

    fn write_object_count(&mut self) {
        let object_count = self.objects.len();
        let text_object_count = format!("Objects:  {}", object_count);
        let text_polygon_count = format!("Polygon Count: {}", self.polygon_count);
        let text_draw_polygons = format!("Show Polygons: {}", self.draw_polygons);
        let text_draw_mesh = format!("Show Mesh: {}", self.draw_mesh);
        self.text_writer.add_text_top_left(text_object_count, None);
        self.text_writer.add_text_top_left(text_polygon_count, None);
        self.text_writer.add_text_top_left(text_draw_polygons, None);
        self.text_writer.add_text_top_left(text_draw_mesh, None);
    }

    fn write_camera_information(&mut self) {
        let camera = &self.camera;
        let cp = camera.camera_position;
        let clt = camera.camera_target;
        let cld = camera.look_direction;
        let clu = camera.up_direction;
        let cls = camera.side_direction;

        let info_header = format!("Camera Information");
        let y_lock = format!("Y-Lock:  {}", camera.y_lock);
        let fov = format!("FOV:  {}", camera.frustum.fov);
        let near_plane = format!("Near Plane:  {}", camera.frustum.near_plane);
        let far_plane = format!("Far Plane:  {}", camera.frustum.far_plane);
        let yaw = format!("Yaw:  {:.2}", camera.yaw);
        let pitch = format!("Pitch:  {:.2}", camera.pitch);
        let position = format!("Position:  {}", cp.to_string());
        let target = format!("Target:  {}", clt.to_string());
        let look_dir = format!("Look (d):  {}", cld.to_string());
        let up_dir = format!("Up (d):  {}", clu.to_string());
        let side_dir = format!("Side (d):  {}", cls.to_string());

        self.text_writer.add_text_top_left("".to_string(), None);
        self.text_writer.add_text_top_left(info_header, None);
        self.text_writer.add_text_top_left(y_lock, None);
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

    fn draw_text(&mut self, graphics: &mut Graphics) {
        self.text_writer.draw(graphics);
    }

    pub fn simulate(&mut self, graphics: &mut Graphics, fps: f64) {
        self.compute_objects();

        self.write_fps_text(fps);
        self.write_timestep_text();
        self.write_object_count();
        self.write_camera_information();
        self.draw_text(graphics);
    }
}
