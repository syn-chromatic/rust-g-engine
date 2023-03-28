use crate::color::RGBA;
use crate::polygons::Mesh;
use crate::polygons::Polygon;
use crate::polygons::Quad;
use crate::polygons::Triangle;
use crate::vectors::Vector3D;
#[derive(Clone, Debug)]
pub struct Light {
    pub position: Vector3D,
    pub ambient: Vector3D,
    pub diffuse: Vector3D,
    pub specular: Vector3D,
}

impl Light {
    pub fn new(
        position: Vector3D,
        ambient: Vector3D,
        diffuse: Vector3D,
        specular: Vector3D,
    ) -> Self {
        Light {
            position,
            ambient,
            diffuse,
            specular,
        }
    }

    pub fn get_light() -> Self {
        let light_position = Vector3D::new(300.0, 500.0, 400.0);
        let ambient = Vector3D::new(0.6, 0.6, 0.6);
        let diffuse = Vector3D::new(0.1, 0.1, 0.1);
        let specular = Vector3D::new(1.0, 1.0, 1.0);

        let light = Light::new(light_position, ambient, diffuse, specular);
        light
    }
}
#[derive(Clone, Debug)]
pub struct Shaders {
    mesh: Mesh,
}

impl Shaders {
    pub fn new(mesh: Mesh) -> Self {
        Shaders { mesh }
    }

    pub fn apply_lighting(&mut self, light: Light, viewer_position: Vector3D) -> Mesh {
        for polygon in &mut self.mesh.polygons {
            match polygon {
                Polygon::Quad(_) => continue,
                Polygon::Triangle(triangle) => {
                    let vertices = triangle.vertices;
                    let v0 = vertices[0];
                    let v1 = vertices[1];
                    let v2 = vertices[2];

                    let edge1 = v1.subtract_vector(&v0);
                    let edge2 = v2.subtract_vector(&v0);
                    let normal = edge1.cross_product(&edge2);
                    let normal = normal.normalize();

                    let light_dir = light.position.subtract_vector(&v0);
                    let light_dir = light_dir.normalize();
                    let light_normal = normal.dot_product(&light_dir);

                    let viewer_dir = viewer_position.subtract_vector(&v0);
                    let viewer_dir = viewer_dir.normalize();

                    let reflection = normal.multiply(2.0 * light_normal);
                    let reflection = reflection.subtract_vector(&light_dir);
                    let reflection = reflection.normalize();

                    let ambient = light.ambient;

                    let light_clamped = light_normal.max(0.0);
                    let diffuse = light.diffuse.multiply(light_clamped);
                    let shininess = 16;

                    let reflection_perspective = reflection.dot_product(&viewer_dir);
                    let specular_clampled = reflection_perspective.max(0.0).powi(shininess);
                    let specular = light.specular.multiply(specular_clampled);

                    let shading = ambient.add_vector(&diffuse).add_vector(&specular);
                    let shading = shading.clamp(0.0, 1.0);
                    triangle.shader = RGBA::from_vector(shading);
                }
            }
        }
        self.mesh.clone()
    }
}
