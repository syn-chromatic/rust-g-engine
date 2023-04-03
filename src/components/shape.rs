use crate::abstracts::body::Body;
use crate::components::backface_culling::BackfaceCulling;
use crate::components::camera::Camera;
use crate::components::graphics::Graphics;
use crate::components::physics::Physics;
use crate::components::polygons::Mesh;
use crate::components::shaders::Light;
use crate::components::shaders::Shaders;
use crate::components::z_buffer::ZBufferSort;

#[derive(Clone, Debug)]
pub struct Shape {
    physics: Physics,
    light: Light,
}

impl Body for Shape {
    fn draw(
        &self,
        graphics: &mut Graphics,
        camera: &mut Camera,
        path_trace: bool,
        bounce_count: usize,
    ) {
        self.draw_shape(graphics, camera, path_trace, bounce_count);
    }

    fn physics(&mut self) -> &mut Physics {
        &mut self.physics
    }
    fn mesh(&self) -> &Mesh {
        &self.physics.mesh
    }
}

impl Shape {
    pub fn new(mesh: Mesh) -> Shape {
        let physics: Physics = Physics::new(mesh);
        let light = Light::get_light();
        Shape { physics, light }
    }

    fn draw_shape(
        &self,
        graphics: &mut Graphics,
        camera: &mut Camera,
        path_trace: bool,
        bounce_count: usize,
    ) {
        // let mut mesh = self.physics.mesh.clone();

        // let camera_position = camera.camera_position;
        // let camera_target = camera.camera_target;

        // let backface_culling = BackfaceCulling::new();
        // mesh = backface_culling.cull_backfaces(mesh, &camera_position);

        // let z_buffer_sort = ZBufferSort::new(camera_position);
        // mesh = z_buffer_sort.get_sorted_polygons(mesh);

        // let light = &self.light;
        // let light_camera = Light::get_light_from_position(camera_position, camera_target);
        // let shaders = Shaders::new();

        // mesh = shaders.apply_pbr_lighting(mesh, light, &camera_position);
        // mesh = shaders.apply_pbr_lighting(mesh, &light_camera, &camera_position);

        // let mesh = camera.apply_projection_polygons(&mesh);
        // if mesh.is_some() {
        //     let mesh = mesh.unwrap();
        //     graphics.draw_polygons(mesh);
        //     graphics.update();
        // }
    }
}
