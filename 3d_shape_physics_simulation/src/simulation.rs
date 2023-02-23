use crate::shape::Shape;
use crate::shape::ShapeBase;
use crate::vector_3d::Vector3D;
use rand::Rng;
use speedy2d::color::Color;
use speedy2d::Graphics2D;

pub struct Simulation {
    center_x: f64,
    center_y: f64,
    objects: Vec<Shape>,
}

impl Simulation {
    pub fn new(center_x: f64, center_y: f64) -> Simulation {
        let objects = vec![];
        Simulation {
            center_x,
            center_y,
            objects,
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

    pub fn create_center_object(&mut self) {
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

    pub fn create_orbiting_object(&mut self) {
        let mut rng = rand::thread_rng();
        let xrnd = rng.gen_range(-100.0..-50.0);
        let yrnd = rng.gen_range(-100.0..-50.0);

        let x: f64 = self.center_x - xrnd;
        let y: f64 = self.center_y - yrnd;
        let z: f64 = 0.0;


        let mass: f64 = rng.gen_range(50.0..100.0);
        let shape = self.get_shape();
        let scale = mass / 200.0;

        let mut shape_base = ShapeBase::new(shape, x, y, z);
        shape_base.set_velocity([10.0, -3.0, 1.0]);
        shape_base.set_mass(mass);
        shape_base.set_scale(scale);


        let shape = Shape::new(shape_base);
        self.objects.push(shape);
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
}
