use std::rc::Rc;

use crate::shape::Shape;
use crate::shape::ShapeBase;
use rand::Rng;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::Font;
use speedy2d::font::FormattedTextBlock;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use speedy2d::Graphics2D;

pub struct Simulation {
    center_x: f64,
    center_y: f64,
    objects: Vec<Shape>,
    font: Font,
}

impl Simulation {
    pub fn new(center_x: f64, center_y: f64) -> Simulation {
        let objects = vec![];
        let bytes = include_bytes!("../fonts/arial.ttf");
        let font = Font::new(bytes).unwrap();

        Simulation {
            center_x,
            center_y,
            objects,
            font,
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
        let shape = self.get_shape();
        let color = Color::from_rgb(0.8, 0.3, 0.3);
        let scale = mass / 10_000.0;

        let mut shape_base = ShapeBase::new(shape, x, y, z);
        shape_base.set_velocity([0.0, 0.0, 0.0]);
        shape_base.set_color(color);
        shape_base.set_mass(mass);
        shape_base.set_scale(scale);
        let shape = Shape::new(shape_base);
        self.objects.push(shape);
    }

    pub fn add_orbiting_object(&mut self) {
        let mut rng = rand::thread_rng();
        let x_rnd = rng.gen_range(-100.0..-50.0);
        let y_rnd = rng.gen_range(-100.0..-50.0);

        let x: f64 = self.center_x - x_rnd;
        let y: f64 = self.center_y - y_rnd;
        let z: f64 = 0.0;

        let mass: f64 = rng.gen_range(50.0..100.0);
        let shape = self.get_shape();
        let scale = mass / 50.0;

        let mut shape_base = ShapeBase::new(shape, x, y, z);
        shape_base.set_velocity([10.0, -3.0, 1.0]);
        shape_base.set_mass(mass);
        shape_base.set_scale(scale);

        let shape = Shape::new(shape_base);
        self.objects.push(shape);
    }

    pub fn setup_objects(&mut self) {
        self.add_center_object();

        for _ in 0..150 {
            self.add_orbiting_object();
        }
    }

    pub fn compute_objects(&mut self, graphics: &mut Graphics2D) {
        let mut objects_clone = self.objects.clone();

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
        let text_options = TextOptions::new();
        let text_scale = 32.0;
        let text_block = self.font.layout_text(&string, text_scale, text_options);
        text_block
    }

    pub fn write_fps(&self, frame_time: f32, graphics: &mut Graphics2D) {
        let fps = format!("{:.2} FPS", 1.0 / frame_time);
        let fps_block = self.get_text_block(fps);
        let x = self.center_x as f32 - 300.0;
        let y = self.center_y as f32 - 300.0;
        let position = Vector2::new(x, y);
        let color = Color::from_rgb(1.0, 1.0, 1.0);
        graphics.draw_text(position, color, &fps_block);
    }
}
