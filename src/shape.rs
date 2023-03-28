use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::body::Body;
use crate::camera::Camera;
use crate::physics::Physics;
use crate::polygons::Mesh;
use crate::polygons::Polygon;
use crate::shaders;
use crate::shaders::Light;
use crate::shaders::Shaders;
use crate::vectors::Vector3D;
use crate::vertices;
use speedy2d::dimen::Vec2;

#[derive(Clone, Debug)]
pub struct Shape {
    physics: Physics,
    color: Color,
    light: Light,
    thickness: f64,
}

impl Body for Shape {
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn draw(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        self.draw_shape(graphics, camera);
    }

    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
}

impl Shape {
    pub fn new(mesh: Mesh) -> Shape {
        let physics: Physics = Physics::new(mesh);
        let color: Color = Color::from_rgb(1.0, 1.0, 1.0);
        let light = Light::get_light();
        let thickness: f64 = 3.0;
        Shape {
            physics,
            color,
            light,
            thickness,
        }
    }

    fn draw_shape(&self, graphics: &mut Graphics2D, camera: &mut Camera) {
        let mut mesh = &self.physics.mesh;
        let mesh = camera.apply_projection_polygons(mesh);
        if mesh.is_some() {
            let mesh = mesh.unwrap();
            let mesh =
                Shaders::new(mesh).apply_lighting(self.light.clone(), camera.camera_position);

            for polygon in mesh.polygons {
                match polygon {
                    Polygon::Triangle(triangle) => {
                        let vertices = triangle.vertices;
                        let shader = triangle.shader;
                        let color = triangle.color;
                        let color = color.multiply(&shader);

                        let v1 = vertices[0];
                        let v2 = vertices[1];
                        let v3 = vertices[2];

                        let p1 = Vec2::new(v1.x as f32, v1.y as f32);
                        let p2 = Vec2::new(v2.x as f32, v2.y as f32);
                        let p3 = Vec2::new(v3.x as f32, v3.y as f32);

                        let points = [p1, p2, p3];
                        let color_sp2d = color.to_sp2d_color();
                        graphics.draw_triangle(points, color_sp2d);

                        let black = Color::from_rgb(0.2, 0.2, 0.2);
                        graphics.draw_line(p1, p2, 0.2, black);
                        graphics.draw_line(p2, p3, 0.2, black);
                        graphics.draw_line(p3, p1, 0.2, black);
                    }
                    Polygon::Quad(quad) => {
                        for idx in 0..quad.vertices.len() {
                            let vertex = quad.vertices[idx];
                        }
                    }
                }
            }
        }
    }
}
