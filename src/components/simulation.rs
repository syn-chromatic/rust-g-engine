use crate::abstracts::body::{Body, BodyType};
use crate::components::camera::Camera;
use crate::components::color::RGBA;

use crate::components::font::ArialFont;
use crate::components::font::FontSettings;
use crate::components::font::FontType;
use crate::components::graphics::Graphics;
use crate::components::text_writer::TextWriter;
use crate::configurations::body_configurations;

pub struct Simulation {
    pub camera: Camera,
    pub objects: Vec<BodyType>,
    polygon_count: usize,
    timestep_hz: f64,
    text_writer: TextWriter,
}

impl Simulation {
    pub fn new(camera: Camera, resolution: (u32, u32)) -> Simulation {
        let objects: Vec<BodyType> = vec![];
        let timestep_hz: f64 = 0.5;
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
            timestep_hz,
            text_writer,
        }
    }

    pub fn setup_objects(&mut self) {
        // self.timestep_hz = 10000.0;
        // let z = 0.0;

        // let grid = body_configurations::get_grid();
        // self.objects.push(grid);

        // let obj = body_configurations::get_obj("./town_square.obj");
        // self.objects.push(obj);

        // let obj = body_configurations::get_obj("./cottage.obj");
        // self.objects.push(obj);

        // let obj = body_configurations::get_obj("./plane.obj");
        // self.objects.push(obj);

        let sphere = body_configurations::get_sphere_light_highmass();
        self.objects.push(sphere);

        // let sphere = body_configurations::get_sphere_light1();
        // self.objects.push(sphere);

        // let sphere = body_configurations::get_sphere_light2();

        // let sphere = body_configurations::get_sphere_light3();
        // self.objects.push(sphere);

        for i in 0..100 {
            let sphere = body_configurations::get_sphere_light3();
            self.objects.push(sphere);
        }

        for object in self.objects.iter_mut() {
            let physics = object.physics();
            let mesh = &physics.mesh;
            let polygon_len = mesh.polygons.len();
            self.polygon_count += polygon_len;
        }
    }

    pub fn increment_timestep(&mut self, increment: f64) {
        if (self.timestep_hz + increment) > 10.0 {
            self.timestep_hz += increment;
        }
    }

    pub fn compute_objects(&mut self) {
        let timestep: f64 = 1.0 / self.timestep_hz;

        for i in 0..self.objects.len() {
            for j in (i + 1)..self.objects.len() {
                let (physics1, physics2) = {
                    let (left, right) = self.objects.split_at_mut(j);
                    (left[i].physics(), right[0].physics())
                };

                physics1.apply_forces(physics2);
            }
        }

        for object in self.objects.iter_mut() {
            object.physics().update(timestep);
        }
    }

    fn write_fps_text(&mut self, fps: f64) {
        let header_text = format!("Simulation information");
        let text = format!("{:.2} FPS", fps);
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
        let text_object_count = format!("Objects:  {}", object_count);
        let text_polygon_count = format!("Polygon Count: {}", self.polygon_count);
        self.text_writer.add_text_top_left(text_object_count, None);
        self.text_writer.add_text_top_left(text_polygon_count, None);
    }

    fn write_camera_information(&mut self) {
        let camera = &self.camera;
        let cp = camera.camera_position;
        let clt = camera.camera_target;
        let cld = camera.look_direction;
        let clu = camera.up_direction;
        let cls = camera.side_direction;

        let info_header = format!("Camera Information");
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
