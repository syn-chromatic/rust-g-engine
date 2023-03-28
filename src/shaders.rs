use crate::color::RGBA;
use crate::polygons::Mesh;
use crate::polygons::Polygon;
use crate::vectors::Vector3D;

#[derive(Clone, Debug)]
pub struct Light {
    pub position: Vector3D,
    pub ambient: Vector3D,
    pub diffuse: Vector3D,
    pub specular: Vector3D,
    pub lumens: f64,
}

impl Light {
    pub fn new(
        position: Vector3D,
        ambient: Vector3D,
        diffuse: Vector3D,
        specular: Vector3D,
        lumens: f64,
    ) -> Self {
        Light {
            position,
            ambient,
            diffuse,
            specular,
            lumens,
        }
    }
    pub fn get_light() -> Self {
        let light_position: Vector3D = Vector3D::new(50.0, 1000.0, 2000.0);
        let ambient: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
        let diffuse: Vector3D = Vector3D::new(0.2, 0.2, 0.2);
        let specular: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
        let lumens: f64 = 5_000_000.0;

        let light: Light = Light::new(light_position, ambient, diffuse, specular, lumens);
        light
    }
}
#[derive(Clone, Debug)]
pub struct Shaders {}

impl Shaders {
    pub fn new() -> Self {
        Shaders {}
    }

    pub fn apply_pbr_lighting(
        &mut self,
        mesh: Mesh,
        light: &Light,
        viewer_position: Vector3D,
    ) -> Mesh {
        let roughness: f64 = 0.5;
        let metallic: f64 = 0.5;
        let k_s: f64 = metallic;
        let k_d: f64 = 1.0 - k_s;
        let f0: f64 = 0.04;
        let constant_attenuation: f64 = 1.0;
        let linear_attenuation: f64 = 0.09;
        let quadratic_attenuation: f64 = 0.032;

        let mut mesh = mesh;

        for polygon in &mut mesh.polygons {
            match polygon {
                Polygon::Quad(_) => continue,
                Polygon::Triangle(triangle) => {
                    let vertices: [Vector3D; 3] = triangle.vertices;
                    let v0: Vector3D = vertices[0];
                    let v1: Vector3D = vertices[1];
                    let v2: Vector3D = vertices[2];

                    let centroid: Vector3D = v0.add_vector(&v1).add_vector(&v2).divide(3.0);

                    let edge1: Vector3D = v1.subtract_vector(&v0);
                    let edge2: Vector3D = v2.subtract_vector(&v0);
                    let normal: Vector3D = edge1.cross_product(&edge2);
                    let normal: Vector3D = normal.normalize();

                    let light_dir: Vector3D = light.position.subtract_vector(&centroid);
                    let distance: f64 = light_dir.get_length();
                    let light_dir: Vector3D = light_dir.normalize();
                    let light_intensity: f64 = light.lumens / (distance * distance);

                    let viewer_dir: Vector3D = viewer_position.subtract_vector(&centroid);
                    let viewer_dir: Vector3D = viewer_dir.normalize();

                    let halfway: Vector3D = light_dir.add_vector(&viewer_dir);
                    let halfway: Vector3D = halfway.normalize();

                    let diffuse_angle: f64 = normal.dot_product(&light_dir);
                    let diffuse_angle: f64 = diffuse_angle.max(0.0);
                    let n_dot_v: f64 = normal.dot_product(&viewer_dir).max(0.0);
                    let n_dot_l: f64 = normal.dot_product(&light_dir).max(0.0);
                    let n_dot_h: f64 = normal.dot_product(&halfway).max(0.0);

                    let roughness_sq = roughness * roughness;
                    let n_dov_vsq = n_dot_v * n_dot_v;
                    let n_dot_lsq = n_dot_l * n_dot_l;
                    let n_dot_hsq = n_dot_h * n_dot_h;

                    let g1: f64 = 2.0 * n_dot_v
                        / (n_dot_v + ((1.0 - roughness_sq) * n_dov_vsq + roughness_sq).sqrt());
                    let g2: f64 = 2.0 * n_dot_l
                        / (n_dot_l + ((1.0 - roughness_sq) * n_dot_lsq + roughness_sq).sqrt());

                    let attenuation: f64 = 1.0
                        / (constant_attenuation
                            + linear_attenuation * distance
                            + quadratic_attenuation * distance * distance);

                    let ambient: Vector3D = light.ambient.multiply(light_intensity);
                    let diffuse: Vector3D = light
                        .diffuse
                        .multiply(diffuse_angle * light_intensity * attenuation);

                    let f: f64 = f0 + (1.0 - f0) * (1.0 - n_dot_h).powi(5);
                    let g: f64 = g1 * g2;
                    let d: f64 = roughness_sq
                        / ((n_dot_hsq * (roughness_sq - 1.0) + 1.0)
                            * (n_dot_hsq * (roughness_sq - 1.0) + 1.0));

                    let specular: Vector3D = light.specular.multiply(
                        f * g * d / (4.0 * n_dot_l * n_dot_v) * light_intensity * attenuation,
                    );

                    let mut shading: Vector3D = ambient
                        .multiply(k_d)
                        .add_vector(&diffuse.multiply(k_d))
                        .add_vector(&specular.multiply(k_s));

                    if shading.x.is_nan() || shading.y.is_nan() || shading.z.is_nan() {
                        shading = Vector3D::new(0.0, 0.0, 0.0);
                    }
                    triangle.shader = RGBA::from_vector(shading);
                }
            }
        }
        mesh
    }
}
