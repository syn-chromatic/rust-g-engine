use crate::components::color::RGBA;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::vectors::Vector3D;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub position: Vector3D,
    pub target: Vector3D,
    pub ambient: Vector3D,
    pub diffuse: Vector3D,
    pub specular: Vector3D,
    pub lumens: f64,
}

impl Light {
    pub fn new(
        position: Vector3D,
        target: Vector3D,
        ambient: Vector3D,
        diffuse: Vector3D,
        specular: Vector3D,
        lumens: f64,
    ) -> Self {
        Light {
            position,
            target,
            ambient,
            diffuse,
            specular,
            lumens,
        }
    }
    pub fn get_light() -> Self {
        let position: Vector3D = Vector3D::new(50.0, 5_000.0, 20000.0);
        let target: Vector3D = Vector3D::new(5000.0, 0.0, 0.0);
        let ambient: Vector3D = Vector3D::new(0.2, 0.2, 0.2);
        let diffuse: Vector3D = Vector3D::new(0.95, 0.6, 0.6);
        let specular: Vector3D = Vector3D::new(0.95, 0.6, 0.6);
        let lumens: f64 = 20_000.0;

        let light: Light = Light::new(position, target, ambient, diffuse, specular, lumens);
        light
    }

    pub fn get_light_from_position(position: Vector3D, target: Vector3D) -> Light {
        let ambient: Vector3D = Vector3D::new(0.2, 0.2, 0.2);
        let diffuse: Vector3D = Vector3D::new(0.6, 0.6, 0.6);
        let specular: Vector3D = Vector3D::new(0.6, 0.6, 0.6);
        let lumens: f64 = 800_000.0;

        let light: Light = Light::new(position, target, ambient, diffuse, specular, lumens);
        light
    }
}

#[derive(Clone, Debug)]
pub struct Shaders {
    roughness: f64,
    metallic: f64,
    albedo: f64,
    f0: f64,
    constant_attenuation: f64,
    linear_attenuation: f64,
    quadratic_attenuation: f64,
}

impl Shaders {
    pub fn new() -> Shaders {
        let roughness = 0.5;
        let metallic = 0.1;
        let albedo = 1.0 - metallic;
        let f0 = 0.04;
        let constant_attenuation = 1.0;
        let linear_attenuation = 0.09;
        let quadratic_attenuation = 0.032;

        Shaders {
            roughness,
            metallic,
            albedo,
            f0,
            constant_attenuation,
            linear_attenuation,
            quadratic_attenuation,
        }
    }

    fn get_attenuation(&self, distance: f64) -> f64 {
        let attenuation = self.constant_attenuation
            + (self.linear_attenuation * distance)
            + (self.quadratic_attenuation * distance.powi(2));
        let attenuation = 1.0 / attenuation;
        attenuation
    }

    fn get_reference_attenuation(&self, distance: f64) -> f64 {
        let ref_attenuation = 0.5;
        let ref_distance = distance.sqrt();

        let linear_attenuation = 2.0 * (1.0 - ref_attenuation) / ref_distance;
        let quadratic_attenuation = (1.0 - ref_attenuation) / (ref_distance.powi(2));

        let attenuation = self.constant_attenuation
            + (linear_attenuation * distance)
            + (quadratic_attenuation * distance.powi(2));

        let attenuation = 1.0 / attenuation;
        attenuation
    }

    fn get_schlick_approximation(&self, n_dot_v: f64) -> f64 {
        self.f0 + (1.0 - self.f0) * (1.0 - n_dot_v).powi(5)
    }

    fn get_ggx_distribution(&self, n_dot_h: f64) -> f64 {
        let alpha_sq = self.roughness.powi(2);
        let n_dot_h_sq = n_dot_h.powi(2);
        let denom = n_dot_h_sq * (alpha_sq - 1.0) + 1.0;
        (alpha_sq) / (PI * denom.powi(2))
    }

    fn get_ggx_smith_geometry(&self, n_dot_v: f64, n_dot_l: f64) -> f64 {
        let alpha_sq = self.roughness.powi(2);

        let g1_v = n_dot_v + ((1.0 - alpha_sq) * n_dot_v.powi(2) + alpha_sq).sqrt();
        let g1_l = n_dot_l + ((1.0 - alpha_sq) * n_dot_l.powi(2) + alpha_sq).sqrt();

        let g_v = 2.0 * n_dot_v / g1_v;
        let g_l = 2.0 * n_dot_l / g1_l;

        g_v * g_l
    }

    fn get_specular_term(&self, f: f64, g: f64, d: f64, n_dot_l: f64, n_dot_v: f64) -> f64 {
        let numerator: f64 = f * g * d - f64::MIN_POSITIVE;
        let denominator: f64 = 4.0 * n_dot_l * n_dot_v + f64::MIN_POSITIVE;
        numerator / denominator
    }

    pub fn is_occluded(
        &self,
        mesh: &Mesh,
        polygon: &Polygon,
        centroid: Vector3D,
        ray_direction: Vector3D,
    ) -> bool {
        let intersecting_polygons = mesh.bvh_node.traverse(&centroid, &ray_direction);

        for intersecting_polygon in &intersecting_polygons {
            if std::ptr::eq(&intersecting_polygon, &polygon) {
                continue;
            }
            if self.intersect_ray(&intersecting_polygon, &centroid, &ray_direction) {
                return true;
            }
        }
        false
    }

    fn intersect_ray_distance(
        &self,
        polygon: &Polygon,
        origin: &Vector3D,
        direction: &Vector3D,
    ) -> Option<f64> {
        let vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };

        let v1 = &vertices[0];
        let v2 = &vertices[1];
        let v3 = &vertices[2];

        let edge1 = v2.subtract_vector(v1);
        let edge2 = v3.subtract_vector(v1);

        let h = direction.cross_product(&edge2);
        let a = edge1.dot_product(&h);
        let epsilon: f64 = 1e-5;

        if -epsilon < a && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = origin.subtract_vector(v1);
        let u = f * s.dot_product(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross_product(&edge1);
        let v = f * direction.dot_product(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot_product(&q);

        if t > epsilon {
            return Some(t);
        }
        None
    }

    pub fn intersect_ray(
        &self,
        polygon: &Polygon,
        origin: &Vector3D,
        direction: &Vector3D,
    ) -> bool {
        let vertices: &[Vector3D] = match polygon {
            Polygon::Triangle(triangle) => &triangle.vertices,
            Polygon::Quad(quad) => &quad.vertices,
        };

        let v1 = &vertices[0];
        let v2 = &vertices[1];
        let v3 = &vertices[2];

        let edge1 = v2.subtract_vector(v1);
        let edge2 = v3.subtract_vector(v1);

        let h = direction.cross_product(&edge2);
        let a = edge1.dot_product(&h);
        let epsilon: f64 = 1e-5;

        if -epsilon < a && a < epsilon {
            return false;
        }

        let f = 1.0 / a;
        let s = origin.subtract_vector(v1);
        let u = f * s.dot_product(&h);

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = s.cross_product(&edge1);
        let v = f * direction.dot_product(&q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * edge2.dot_product(&q);

        if t > epsilon {
            return true;
        }

        false
    }

    fn get_pbr_shader(
        &self,
        light: &Light,
        polygon: &Polygon,
        viewer_position: &Vector3D,
        mesh: &Mesh,
    ) -> Vector3D {
        let light_dir: Vector3D = light.target.subtract_vector(&light.position);
        let light_dir: Vector3D = light_dir.normalize();

        let centroid: Vector3D = polygon.get_centroid();
        let normal: Vector3D = polygon.get_normal();

        let ray_vector = light.position.subtract_vector(&centroid);
        let ray_direction = ray_vector.normalize();
        let distance = ray_vector.get_length();
        let attenuation = self.get_reference_attenuation(distance);

        let light_normal = ray_direction.dot_product(&light_dir);
        let light_dir: Vector3D = light_dir.multiply(light_normal);
        let viewer_dir: Vector3D = viewer_position.subtract_vector(&centroid);
        let viewer_dir: Vector3D = viewer_dir.normalize();

        let halfway: Vector3D = light_dir.add_vector(&viewer_dir);
        let halfway: Vector3D = halfway.normalize();

        let diffuse_angle: f64 = normal.dot_product(&light_dir);
        let diffuse_angle: f64 = diffuse_angle.max(0.0);

        let n_dot_v: f64 = normal.dot_product(&viewer_dir).max(0.0);
        let n_dot_l: f64 = normal.dot_product(&light_dir).max(0.0);
        let n_dot_h: f64 = normal.dot_product(&halfway).max(0.0);

        let mut light_intensity = diffuse_angle * light.lumens;
        light_intensity = light_intensity * attenuation;

        // let is_occluded = self.is_occluded(mesh, polygon, centroid, ray_direction);

        // if is_occluded {
        //     let scaling_factor = 0.5;
        //     let shadow_term = 1.0 - scaling_factor * (1.0 - diffuse_angle);
        //     light_intensity *= shadow_term;
        // }

        let ambient: Vector3D = light.ambient.multiply(light_intensity);
        let diffuse: Vector3D = light.diffuse.multiply(light_intensity);

        let f: f64 = self.get_schlick_approximation(n_dot_v);
        let g: f64 = self.get_ggx_smith_geometry(n_dot_v, n_dot_l);
        let d: f64 = self.get_ggx_distribution(n_dot_h);

        let specular_term = self.get_specular_term(f, g, d, n_dot_l, n_dot_v);
        let specular_term = specular_term * attenuation;
        let specular: Vector3D = light.specular.multiply(specular_term);

        let shader_vec: Vector3D = ambient.multiply(self.albedo);
        let shader_vec: Vector3D = shader_vec.add_vector(&diffuse.multiply(self.albedo));
        let shader_vec: Vector3D = shader_vec.add_vector(&specular.multiply(self.metallic));
        return shader_vec;
    }

    pub fn apply_pbr_lighting(
        &self,
        mut mesh: Mesh,
        light: &Light,
        viewer_position: &Vector3D,
    ) -> Mesh {
        for i in 0..mesh.polygons.len() {
            let shader_vec = self.get_pbr_shader(light, &mesh.polygons[i], viewer_position, &mesh);
            let shader = RGBA::from_vector(shader_vec);
            mesh.polygons[i].set_shader(shader);
        }

        mesh
    }
}
