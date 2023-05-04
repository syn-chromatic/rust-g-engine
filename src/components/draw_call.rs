use super::polygons::Mesh;
use super::vectors::Vector3D;
use crate::abstracts::body::Body;
use crate::abstracts::body::BodyType;
use crate::components::backface_culling::BackfaceCulling;
use crate::components::color::RGBA;
use crate::components::frametime::FrameTimeHandler;
use crate::components::graphics::Graphics;
use crate::components::polygons::Polygon;
use crate::components::shaders::Light;
use crate::components::shaders::Shaders;
use crate::components::simulation::Simulation;
use crate::components::buffer_sort::ZBufferSort;
use crate::Camera;
use rayon::prelude::*;

pub struct DrawCall {
    pub graphics: Graphics,
    pub frame_timing: FrameTimeHandler,
    pub simulation: Simulation,
    pub light: Light,
    shaders: Shaders,
    backface_culling: BackfaceCulling,
    z_buffer_sort: ZBufferSort,
}

impl DrawCall {
    pub fn new(graphics: Graphics, simulation: Simulation) -> DrawCall {
        let frame_timing: FrameTimeHandler = FrameTimeHandler::new(30);
        let light: Light = Light::get_light();
        let shaders: Shaders = Shaders::new();
        let backface_culling: BackfaceCulling = BackfaceCulling::new();
        let z_buffer_sort: ZBufferSort = ZBufferSort::new();
        DrawCall {
            graphics,
            frame_timing,
            simulation,
            light,
            shaders,
            backface_culling,
            z_buffer_sort,
        }
    }

    fn get_camera_light(&self) -> Light {
        let camera: &Camera = &self.simulation.camera;
        let camera_position: Vector3D = camera.camera_position;
        let camera_target: Vector3D = camera.camera_target;
        let light_camera: Light = Light::get_light_from_position(camera_position, camera_target);
        light_camera
    }

    fn get_lights(&self, meshes: &[Mesh]) -> Vec<Light> {
        let mut lights: Vec<Light> = vec![];
        for mesh in meshes {
            let mesh_light: &Option<Light> = &mesh.light;
            if mesh_light.is_some() {
                lights.push(mesh_light.unwrap());
            }
        }
        let camera_light: Light = self.get_camera_light();
        lights.push(camera_light);
        lights
    }

    fn get_meshes(&mut self) -> Vec<Mesh> {
        let objects: &Vec<BodyType> = &self.simulation.objects;
        let meshes = objects.par_iter().map(|body| body.mesh().clone()).collect();
        meshes
    }

    fn get_mesh_clusters(&mut self) -> Vec<Vec<Mesh>> {
        let objects: &Vec<BodyType> = &self.simulation.objects;
        let meshes = objects
            .par_iter()
            .map(|body| body.mesh_cluster().as_ref().unwrap_or(&Vec::new()).clone())
            .collect();
        meshes
    }

    fn draw_convex_hulls(&mut self, meshes: &[Mesh]) {
        let camera: &mut Camera = &mut self.simulation.camera;
        let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
        let thickness = 1.0;

        for mesh in meshes {
            let vertices = &mesh.bvh_node.vertices;
            for i in 0..vertices.len() {
                let v1: Vector3D = vertices[i];
                let v2: Vector3D = vertices[(i + 1) % vertices.len()];

                let line: Option<(Vector3D, Vector3D)> = camera.transform_line(v1, v2);
                if line.is_some() {
                    let (v1, v2): (Vector3D, Vector3D) = line.unwrap();
                    self.graphics.draw_line(v1, v2, color, thickness);
                }
            }
        }
    }

    fn draw_bounding_box(&mut self, meshes: &Vec<Mesh>) {
        let camera: &mut Camera = &mut self.simulation.camera;
        let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
        let thickness = 1.0;

        for mesh in meshes {
            let aabb_points = mesh.bvh_node.get_aabb_points();
            let edges = [
                (0, 1),
                (1, 3),
                (3, 2),
                (2, 0),
                (4, 5),
                (5, 7),
                (7, 6),
                (6, 4),
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7),
            ];

            for &(start, end) in edges.iter() {
                let v1: Vector3D = aabb_points[start];
                let v2: Vector3D = aabb_points[end];

                let line: Option<(Vector3D, Vector3D)> = camera.transform_line(v1, v2);
                if line.is_some() {
                    let (v1, v2): (Vector3D, Vector3D) = line.unwrap();
                    self.graphics.draw_line(v1, v2, color, thickness);
                }
            }
        }
    }

    fn draw_bounding_box_clusters(&mut self, clusters: &Vec<Vec<Mesh>>) {
        let camera: &mut Camera = &mut self.simulation.camera;
        let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
        let thickness = 1.0;

        for mesh_cluster in clusters {
            for mesh in mesh_cluster {
                let aabb_points = mesh.bvh_node.get_aabb_points();
                let edges = [
                    (0, 1),
                    (1, 3),
                    (3, 2),
                    (2, 0),
                    (4, 5),
                    (5, 7),
                    (7, 6),
                    (6, 4),
                    (0, 4),
                    (1, 5),
                    (2, 6),
                    (3, 7),
                ];

                for &(start, end) in edges.iter() {
                    let v1: Vector3D = aabb_points[start];
                    let v2: Vector3D = aabb_points[end];

                    let line: Option<(Vector3D, Vector3D)> = camera.transform_line(v1, v2);
                    if line.is_some() {
                        let (v1, v2): (Vector3D, Vector3D) = line.unwrap();
                        self.graphics.draw_line(v1, v2, color, thickness);
                    }
                }
            }
        }
    }

    fn draw_contact_points(&mut self) {
        let camera: &mut Camera = &mut self.simulation.camera;
        let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
        let radius: f64 = 5.0;

        for object in self.simulation.objects.iter_mut() {
            let last_contact_point: Option<Vector3D> = object.physics().last_contact_point;
            if last_contact_point.is_some() {
                let contact_point: Vector3D = last_contact_point.unwrap();
                // let contact_point: Vector3D = camera.transform_vertex(contact_point);

                let contact_point: Vector3D = camera.apply_view_transform(contact_point);

                let is_inside_frustum: bool = camera.frustum.is_point_in_frustum(contact_point);
                if !is_inside_frustum {
                    continue;
                }
                let contact_point: Vector3D =
                    camera.calculate_perspective_projection(contact_point);
                let contact_point: Vector3D = camera.ndc_to_screen_coordinates(contact_point);

                self.graphics.draw_circle(contact_point, color, radius);
            }
        }
    }

    // fn draw_center_of_masses(&mut self) {
    //     let camera: &mut Camera = &mut self.simulation.camera;
    //     let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
    //     let radius: f64 = 5.0;

    //     for object in self.simulation.objects.iter_mut() {
    //         let mesh = &object.physics().mesh_cluster[0];
    //         let center_of_mass = mesh.get_center_of_mass();

    //         let center_of_mass: Vector3D = camera.apply_view_transform(center_of_mass);

    //         let is_inside_frustum: bool = camera.frustum.is_point_in_frustum(center_of_mass);
    //         if !is_inside_frustum {
    //             continue;
    //         }
    //         let center_of_mass: Vector3D = camera.calculate_perspective_projection(center_of_mass);
    //         let center_of_mass: Vector3D = camera.ndc_to_screen_coordinates(center_of_mass);

    //         self.graphics.draw_circle(center_of_mass, color, radius);
    //     }
    // }

    fn draw_inertias(&mut self) {
        let camera: &mut Camera = &mut self.simulation.camera;
        let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
        let radius: f64 = 5.0;

        for object in self.simulation.objects.iter_mut() {
            let inertia = object.physics().moment_of_inertia;

            let inertia: Vector3D = camera.apply_view_transform(inertia);

            let is_inside_frustum: bool = camera.frustum.is_point_in_frustum(inertia);
            if !is_inside_frustum {
                continue;
            }
            let inertia: Vector3D = camera.calculate_perspective_projection(inertia);
            let inertia: Vector3D = camera.ndc_to_screen_coordinates(inertia);

            self.graphics.draw_circle(inertia, color, radius);
        }
    }

    fn get_total_polygons(&self, meshes: &Vec<Mesh>) -> usize {
        let mut total_polygons: usize = 0;

        for mesh in meshes {
            total_polygons += mesh.polygons.len();
        }

        total_polygons
    }

    fn combine_meshes(&mut self, meshes: &Vec<Mesh>) -> Mesh {
        let total_polygons: usize = self.get_total_polygons(meshes);
        self.simulation.polygon_count = total_polygons;
        let mut polygons: Vec<Polygon> = Vec::with_capacity(total_polygons);

        for mesh in meshes {
            polygons.extend(mesh.polygons.clone());
        }

        Mesh::new(polygons)
    }

    fn get_total_polygons_clusters(&self, clusters: &Vec<Vec<Mesh>>) -> usize {
        let mut total_polygons: usize = 0;

        for meshes in clusters {
            for mesh in meshes {
                total_polygons += mesh.polygons.len();
            }
        }

        total_polygons
    }

    fn combine_mesh_clusters(&mut self, clusters: &Vec<Vec<Mesh>>) -> Mesh {
        let total_polygons: usize = self.get_total_polygons_clusters(clusters);
        self.simulation.polygon_count = total_polygons;
        let mut polygons: Vec<Polygon> = Vec::with_capacity(total_polygons);

        for meshes in clusters {
            for mesh in meshes {
                polygons.extend(mesh.polygons.clone());
            }
        }

        Mesh::new(polygons)
    }

    fn cull_backfaces_meshes(&self, meshes: &[Mesh]) {}

    fn apply_lighting_meshes(&self, meshes: &[Mesh], lights: &[Light]) {}

    fn cull_backfaces_mesh(&self, mut mesh: Mesh) -> Mesh {
        let camera: &Camera = &self.simulation.camera;
        let camera_position: Vector3D = camera.camera_position;
        mesh = self.backface_culling.cull_backfaces(mesh, &camera_position);
        mesh
    }

    fn apply_lighting_mesh(&self, mut mesh: Mesh, lights: &[Light]) -> Mesh {
        let camera: &Camera = &self.simulation.camera;
        let camera_position: Vector3D = camera.camera_position;
        for light in lights {
            mesh = self
                .shaders
                .apply_pbr_lighting(mesh, &light, &camera_position);
        }
        mesh
    }

    fn apply_projection(&mut self, mesh: Mesh) -> Mesh {
        let camera: &mut Camera = &mut self.simulation.camera;
        let mesh: Mesh = camera.apply_projection_polygons(mesh);
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

    fn get_lights_1(&self) -> Vec<Light> {
        let lights: Vec<Light> = vec![self.get_camera_light(), self.light.clone()];
        lights
    }

    fn get_lights_2(&self) -> Vec<Light> {
        let lights: Vec<Light> = vec![self.get_camera_light()];
        lights
    }

    fn get_lights_3(&self) -> Vec<Light> {
        let lights: Vec<Light> = vec![self.light.clone()];
        lights
    }

    pub fn draw_meshes(&mut self) {
        let meshes: Vec<Mesh> = self.get_meshes();
        // self.draw_contact_points();
        // self.draw_center_of_masses();
        // self.draw_inertias();
        // self.draw_bounding_box(&meshes);

        if self.simulation.draw_mesh {
            self.draw_convex_hulls(&meshes);
        }

        if self.simulation.draw_polygons {
            let mut mesh: Mesh = self.combine_meshes(&meshes);
            let lights: Vec<Light> = self.get_lights_2();

            mesh = self.apply_z_buffer_sort(mesh);
            mesh = self.cull_backfaces_mesh(mesh);

            mesh = self.apply_lighting_mesh(mesh, &lights);
            mesh = self.apply_projection(mesh);

            self.graphics.draw_polygons(mesh);
        }
    }

    pub fn draw_clusters(&mut self) {
        let clusters: Vec<Vec<Mesh>> = self.get_mesh_clusters();
        self.draw_bounding_box_clusters(&clusters);
        // let mut mesh: Mesh = self.combine_mesh_clusters(&clusters);

        // let lights: Vec<Light> = self.get_lights_2();

        // mesh = self.cull_backfaces_mesh(mesh);
        // mesh = self.apply_z_buffer_sort(mesh);
        // mesh = self.apply_lighting_mesh(mesh, &lights);
        // mesh = self.apply_projection(mesh);

        // self.graphics.draw_polygons(mesh);
    }

    pub fn draw(&mut self) {
        self.draw_meshes();
        // self.draw_clusters();

        self.graphics.update();
    }
}
