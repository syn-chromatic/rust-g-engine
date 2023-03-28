use speedy2d::Graphics2D;

use crate::body::{Body, BodyType};
use crate::camera::Camera;
use crate::color::RGBA;

use crate::configurations::body_configurations;
use crate::font::ArialFont;
use crate::font::FontSettings;
use crate::font::FontType;
use crate::shape::Shape;
use crate::text_writer::TextWriter;
use crate::vertices::GridHorizontal;

pub struct Simulation {
    pub camera: Camera,
    center_point: (f64, f64),
    objects: Vec<BodyType>,
    timestep_hz: f64,
    text_writer: TextWriter,
}

impl Simulation {
    pub fn new(camera: Camera, resolution: (u32, u32)) -> Simulation {
        let objects: Vec<BodyType> = vec![];
        let timestep_hz: f64 = 5000.0;

        let center_x: f64 = resolution.0 as f64 / 2.0;
        let center_y: f64 = resolution.1 as f64 / 2.0;
        let center_point: (f64, f64) = (center_x, center_y);

        let center_point = (0.0, 0.0);

        let arial_font: ArialFont = ArialFont::new();
        let font_type: FontType = FontType::ArialFont(arial_font);
        let font_color: RGBA = RGBA::new(1.0, 1.0, 1.0, 1.0);
        let font: FontSettings = FontSettings::new(font_type, 14, font_color, 1.8, 1);
        let text_writer: TextWriter = TextWriter::new(resolution, font);

        Simulation {
            camera,
            center_point,
            objects,
            timestep_hz,
            text_writer,
        }
    }

    pub fn setup_objects(&mut self) {
        self.timestep_hz = 10000.0;
        let z = 0.0;

        let grid = body_configurations::get_grid();
        self.objects.push(grid);

        let obj = body_configurations::get_obj();
        self.objects.push(obj);
    }

    pub fn compute_objects(&mut self, graphics: &mut Graphics2D) {
        let timestep: f64 = 1.0 / self.timestep_hz;
        let mut objects_cl: Vec<BodyType> = self.objects.clone();
        for (i, pl1) in self.objects.iter_mut().enumerate() {
            // let pl1_physics = pl1.physics();
            // for (j, pl2) in objects_cl.iter_mut().enumerate() {
            //     if i == j {
            //         continue;
            //     }

            //     let pl2_physics = pl2.physics();
            //     pl1_physics.apply_forces(pl2_physics, timestep);
            // }
            // // pl1_physics.update(timestep);
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

    fn draw_text(&mut self, graphics: &mut Graphics2D) {
        self.text_writer.draw(graphics);
    }

    pub fn simulate(&mut self, graphics: &mut Graphics2D, frame_time: f32) {
        self.compute_objects(graphics);
        self.write_fps_text(frame_time);
        self.write_timestep_text();
        self.write_object_count();
        self.write_camera_information();
        self.draw_text(graphics);
    }
}
