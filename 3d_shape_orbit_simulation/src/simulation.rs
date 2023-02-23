use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Rc;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::Font;
use speedy2d::font::FormattedTextBlock;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use speedy2d::Graphics2D;

use crate::shape::Shape;
use crate::shape::ShapeBase;

pub struct Simulation {
    center_x: f64,
    center_y: f64,
    objects: Vec<Shape>,
    font: Font,
    objects_text_xy: [f32; 2],
    fps_text_xy: [f32; 2],
}

impl Simulation {
    pub fn new(center_x: f64, center_y: f64) -> Simulation {
        let objects: Vec<Shape> = vec![];
        let bytes: &[u8; 367112] = include_bytes!("../fonts/arial.ttf");
        let font: Font = Font::new(bytes).unwrap();

        let ox: f32 = center_x as f32 - 300.0;
        let oy: f32 = center_y as f32 - 350.0;
        let fx: f32 = center_x as f32 - 300.0;
        let fy: f32 = center_y as f32 - 300.0;

        let objects_text_xy: [f32; 2] = [ox, oy];
        let fps_text_xy: [f32; 2] = [fx, fy];

        Simulation {
            center_x,
            center_y,
            objects,
            font,
            objects_text_xy,
            fps_text_xy,
        }
    }

    fn get_shape(&self) -> Vec<[f64; 3]> {
        let shape: Vec<[f64; 3]> = vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];
        shape
    }

    pub fn add_center_object(&mut self) {
        let x: f64 = self.center_x;
        let y: f64 = self.center_y;
        let z: f64 = 0.0;
        let mass: f64 = 800_000.0;
        let shape: Vec<[f64; 3]> = self.get_shape();
        let color: Color = Color::from_rgb(0.8, 0.3, 0.3);
        let scale: f64 = mass / 10_000.0;

        let mut shape_base: ShapeBase = ShapeBase::new(shape, x, y, z);
        shape_base.set_velocity([0.0, 0.0, 0.0]);
        shape_base.set_color(color);
        shape_base.set_mass(mass);
        shape_base.set_scale(scale);
        let shape: Shape = Shape::new(shape_base);
        self.objects.push(shape);
    }

    pub fn add_orbiting_object(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let x_rnd: f64 = rng.gen_range(-100.0..-50.0);
        let y_rnd: f64 = rng.gen_range(-100.0..-50.0);

        let x: f64 = self.center_x - x_rnd;
        let y: f64 = self.center_y - y_rnd;
        let z: f64 = 0.0;

        let mass: f64 = rng.gen_range(50.0..100.0);
        let shape: Vec<[f64; 3]> = self.get_shape();
        let scale: f64 = mass / 50.0;

        let mut shape_base: ShapeBase = ShapeBase::new(shape, x, y, z);
        shape_base.set_velocity([10.0, -3.0, 1.0]);
        shape_base.set_mass(mass);
        shape_base.set_scale(scale);

        let shape: Shape = Shape::new(shape_base);
        self.objects.push(shape);
    }

    pub fn setup_objects(&mut self) {
        self.add_center_object();

        for _ in 0..150 {
            self.add_orbiting_object();
        }
    }

    pub fn compute_objects(&mut self, graphics: &mut Graphics2D) {
        let mut objects_clone: Vec<Shape> = self.objects.clone();

        for (i, pl1) in self.objects.iter_mut().enumerate() {
            for (j, pl2) in objects_clone.iter_mut().enumerate() {
                if i != j {
                    pl1.apply_attraction(pl2);
                }
            }
            pl1.update_object(graphics);
        }
    }

    fn get_text_block(&self, string: String) -> Rc<FormattedTextBlock> {
        let text_options: TextOptions = TextOptions::new();
        let text_scale: f32 = 32.0;
        let text_block: Rc<FormattedTextBlock> =
            self.font.layout_text(&string, text_scale, text_options);
        text_block
    }

    pub fn write_object_count(&self, graphics: &mut Graphics2D) {
        let len_objects: usize = self.objects.len();
        let objects_str: String = format!("Objects: {:.2}", len_objects);
        let objects_block: Rc<FormattedTextBlock> = self.get_text_block(objects_str);
        let x: f32 = self.objects_text_xy[0];
        let y: f32 = self.objects_text_xy[1];
        let position: Vector2<f32> = Vector2::new(x, y);
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        graphics.draw_text(position, color, &objects_block);
    }

    pub fn write_fps(&self, frame_time: f32, graphics: &mut Graphics2D) {
        let fps_str: String = format!("{:.2} FPS", 1.0 / frame_time);
        let fps_block: Rc<FormattedTextBlock> = self.get_text_block(fps_str);
        let x: f32 = self.fps_text_xy[0];
        let y: f32 = self.fps_text_xy[1];
        let position: Vector2<f32> = Vector2::new(x, y);
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        graphics.draw_text(position, color, &fps_block);
    }
}
