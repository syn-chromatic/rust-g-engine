use super::polygons::Mesh;
use crate::abstracts::body::{Body, BodyType};
use crate::components::backface_culling::BackfaceCulling;
use crate::components::bvh::BVHNode;
use crate::components::frametime::FrameTimeHandler;
use crate::components::graphics::Graphics;
use crate::components::polygons::Polygon;
use crate::components::shaders::Light;
use crate::components::shaders::Shaders;
use crate::components::simulation::Simulation;
use crate::components::z_buffer::ZBufferSort;
use rayon::prelude::*;

pub struct DrawCall {
    pub graphics: Graphics,
    pub frame_timing: FrameTimeHandler,
    pub simulation: Simulation,
    pub light: Light,
    shaders: Shaders,
    backface_culling: BackfaceCulling,
    z_buffer_sort: ZBufferSort,
    bvh_node: BVHNode,
}

impl DrawCall {
    pub fn new(graphics: Graphics, simulation: Simulation) -> DrawCall {
        let frame_timing = FrameTimeHandler::new(30);
        let light = Light::get_light();
        let shaders = Shaders::new();
        let backface_culling = BackfaceCulling::new();
        let z_buffer_sort = ZBufferSort::new();
        let bvh_node = BVHNode::new();
        DrawCall {
            graphics,
            frame_timing,
            simulation,
            light,
            shaders,
            backface_culling,
            z_buffer_sort,
            bvh_node,
        }
    }

    fn get_camera_light(&self) -> Light {
        let camera = &self.simulation.camera;
        let camera_position = camera.camera_position;
        let camera_target = camera.camera_target;
        let light_camera = Light::get_light_from_position(camera_position, camera_target);
        light_camera
    }

    fn get_lights(&self, meshes: Vec<Mesh>) -> Vec<Light> {
        let mut lights: Vec<Light> = vec![];
        for mesh in meshes {
            if mesh.light.is_some() {
                lights.push(mesh.light.unwrap());
            }
        }
        lights
    }

    fn get_meshes(&self, objects: Vec<BodyType>) -> Vec<Mesh> {
        let mut meshes: Vec<Mesh> = vec![];

        for body in objects.iter() {
            let mesh = body.mesh();
            meshes.push(mesh.clone());
        }
        meshes
    }

    // fn get_meshes(&self, objects: Vec<BodyType>) -> Vec<Mesh> {
    //     let meshes: Vec<Mesh> = objects.par_iter().map(|body| body.mesh().clone()).collect();

    //     meshes
    // }

    fn cull_backfaces_meshes(&self, meshes: Vec<Mesh>) {}

    fn apply_lighting_meshes(&self, meshes: Vec<Mesh>, lights: Vec<Light>) {}

    fn cull_backfaces_mesh(&self, mut mesh: Mesh) -> Mesh {
        let camera = &self.simulation.camera;
        let camera_position = camera.camera_position;
        mesh = self.backface_culling.cull_backfaces(mesh, &camera_position);
        mesh
    }

    fn apply_lighting_mesh(&self, mut mesh: Mesh, lights: Vec<Light>, bvh_node: &BVHNode) -> Mesh {
        let camera = &self.simulation.camera;
        let camera_position = camera.camera_position;
        for light in lights {
            mesh = self
                .shaders
                .apply_pbr_lighting(mesh, &light, &camera_position, bvh_node);
        }
        mesh
    }

    fn apply_projection(&mut self, mut mesh: Mesh) -> Option<Mesh> {
        let camera = &mut self.simulation.camera;
        let mesh = camera.apply_projection_polygons(&mesh);
        mesh
    }

    fn apply_z_buffer_sort(&self, mut mesh: Mesh) -> Mesh {
        let camera = &self.simulation.camera;
        let camera_position = camera.camera_position;
        mesh = self
            .z_buffer_sort
            .get_sorted_polygons(mesh, camera_position);
        mesh
    }

    fn combine_meshes(&self, meshes: Vec<Mesh>) -> Mesh {
        let mut polygons: Vec<Polygon> = vec![];
        for mesh in meshes {
            polygons.extend(mesh.polygons);
        }
        let mesh = Mesh::new(polygons);
        mesh
    }

    // fn combine_meshes(&self, meshes: Vec<Mesh>) -> Mesh {
    //     let total_polygons = meshes.iter().map(|mesh| mesh.polygons.len()).sum();
    //     let mut polygons: Vec<Polygon> = Vec::with_capacity(total_polygons);

    //     for mesh in meshes {
    //         polygons.extend(mesh.polygons);
    //     }

    //     Mesh::new(polygons)
    // }

    // fn combine_meshes(&self, meshes: Vec<Mesh>) -> Mesh {
    //     let polygons: Vec<Polygon> = meshes
    //         .into_par_iter()
    //         .flat_map(|mesh| mesh.polygons)
    //         .collect();

    //     let mesh = Mesh::new(polygons);
    //     mesh
    // }

    fn set_bvh_node(&mut self, mesh: &Mesh) {
        let bvh_node = self.bvh_node.fresh_node(mesh.polygons.clone());
        self.bvh_node = bvh_node;
    }

    pub fn draw(&mut self, objects: Vec<BodyType>) {
        let meshes = self.get_meshes(objects);
        let mut mesh = self.combine_meshes(meshes);

        let lights = vec![self.light.clone()];
        self.set_bvh_node(&mesh);

        mesh = self.apply_z_buffer_sort(mesh);
        mesh = self.cull_backfaces_mesh(mesh);
        mesh = self.apply_lighting_mesh(mesh, lights, &self.bvh_node);
        let mesh = self.apply_projection(mesh);

        if mesh.is_some() {
            let mesh = mesh.unwrap();
            self.graphics.draw_polygons(mesh);
            self.graphics.update();
        }
    }
}
