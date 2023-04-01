use crate::components::color::RGBA;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::polygons::Triangle;
use crate::components::vectors::Vector3D;

pub struct Ray {
    pub orig: Vector3D,
    pub dir: Vector3D,
}

impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vector3D {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vector3D {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> Vector3D {
        self.orig.add_vector(&&self.dir.multiply(t))
    }
}

pub fn ray_color(r: &Ray) -> Vector3D {
    let unit_direction = &r.direction().normalize();

    let t = 0.5 * (unit_direction.y + 1.0);
    Vector3D::new(1.0, 1.0, 1.0)
        .multiply(1.0 - t)
        .add_vector(&Vector3D::new(0.5, 0.7, 1.0).multiply(t))
}

#[derive(Clone, Debug)]
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
        let position: Vector3D = Vector3D::new(50.0, 5_000.0, 10000.0);
        let target: Vector3D = Vector3D::new(5000.0, 0.0, 0.0);
        let ambient: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
        let diffuse: Vector3D = Vector3D::new(0.2, 0.2, 0.2);
        let specular: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let lumens: f64 = 20_000.0;

        let light: Light = Light::new(position, target, ambient, diffuse, specular, lumens);
        light
    }

    pub fn get_light_from_position(position: Vector3D, target: Vector3D) -> Light {
        let ambient: Vector3D = Vector3D::new(0.8, 0.8, 0.8);
        let diffuse: Vector3D = Vector3D::new(0.5, 0.5, 0.5);
        let specular: Vector3D = Vector3D::new(0.2, 0.2, 0.2);
        let lumens: f64 = 1000.0;

        let light: Light = Light::new(position, target, ambient, diffuse, specular, lumens);
        light
    }
}
#[derive(Clone, Debug)]
pub struct Shaders;

impl Shaders {
    fn trace_light_ray(
        &self,
        mesh: &Mesh,
        light: &Light,
        viewer_position: Vector3D,
        ray_origin: Vector3D,
        ray_direction: Vector3D,
        max_bounces: u32,
        current_bounce: u32,
    ) -> Vector3D {
        if current_bounce > max_bounces {
            return Vector3D::new(0.0, 0.0, 0.0);
        }
        let mut closest_intersection: Option<(f64, &Triangle)> = None;

        for (idx, polygon) in mesh.polygons.iter().enumerate() {
            if let Polygon::Triangle(triangle) = polygon {
                if let Some(distance) = self.ray_intersection(triangle, ray_origin, ray_direction) {
                    if closest_intersection.is_none() || distance < closest_intersection.unwrap().0
                    {
                        closest_intersection = Some((distance, triangle));
                    }
                }
            }
        }

        if let Some((_, triangle)) = closest_intersection {
            let direct_illumination = self.get_pbr_shader(light, triangle, viewer_position);

            let normal = self.normal(triangle);
            let reflection = ray_direction
                .subtract_vector(&normal.multiply(2.0 * normal.dot_product(&ray_direction)));

            let indirect_illumination = self.trace_light_ray(
                mesh,
                light,
                viewer_position,
                self.centroid(triangle),
                reflection,
                max_bounces,
                current_bounce + 1,
            );

            direct_illumination.add_vector(&indirect_illumination)
        } else {
            Vector3D::new(0.0, 0.0, 0.0)
        }
    }
    pub fn apply_ray_traced_lighting(
        &self,
        mut mesh: Mesh,
        light: &Light,
        viewer_position: Vector3D,
        max_bounces: u32,
    ) -> Mesh {
        let light_dir = light.target.subtract_vector(&light.position);
        let light_dir = light_dir.normalize();

        let mesh_clone = mesh.clone();

        for (index, polygon) in mesh.polygons.iter_mut().enumerate() {
            if let Polygon::Triangle(triangle) = polygon {
                let normal = self.normal(triangle);
                let facing_light = normal.dot_product(&light_dir) < 0.0;

                if facing_light {
                    let shader_vec = self.trace_light_ray(
                        &mesh_clone,
                        light,
                        viewer_position,
                        light.position.clone(),
                        light_dir.clone(),
                        max_bounces,
                        0,
                    );
                    let shader = RGBA::from_vector(shader_vec);
                    // println!("{:?}", shader);
                    triangle.shader = triangle.shader.average(&shader);
                }
            }
        }

        mesh
    }

    fn random_in_hemisphere(&self, normal: &Vector3D) -> Vector3D {
        let up = if normal.y.abs() < 0.99 {
            Vector3D::new(0.0, 1.0, 0.0)
        } else {
            Vector3D::new(1.0, 0.0, 0.0)
        };

        let tangent = up.cross_product(normal).normalize();
        let bitangent = normal.cross_product(&tangent);

        let r1 = rand::random::<f64>();
        let r2 = rand::random::<f64>();
        let r3 = rand::random::<f64>();

        let sin_theta = (1.0 - r1 * r1).sqrt();
        let phi = 2.0 * std::f64::consts::PI * r2;

        let x = phi.cos() * sin_theta;
        let y = phi.sin() * sin_theta;
        let z = r1;

        let new_dir = tangent
            .multiply(x)
            .add_vector(&bitangent.multiply(y))
            .add_vector(&normal.multiply(z));

        new_dir.normalize()
    }

    fn ray_intersection(
        &self,
        triangle: &Triangle,
        ray_origin: Vector3D,
        ray_direction: Vector3D,
    ) -> Option<f64> {
        let epsilon = 1e-6;
        let vertices = triangle.vertices;
        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];

        let edge1 = v1.subtract_vector(&v0);
        let edge2 = v2.subtract_vector(&v0);

        let h = ray_direction.cross_product(&edge2);
        let a = edge1.dot_product(&h);

        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray_origin.subtract_vector(&v0);
        let u = f * s.dot_product(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross_product(&edge1);
        let v = f * ray_direction.dot_product(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot_product(&q);

        if t > epsilon {
            Some(t)
        } else {
            None
        }
    }

    fn centroid(&self, triangle: &Triangle) -> Vector3D {
        let vertices = triangle.vertices;
        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];

        v0.add_vector(&v1).add_vector(&v2).divide(3.0)
    }

    fn normal(&self, triangle: &Triangle) -> Vector3D {
        let vertices = triangle.vertices;
        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];

        let edge1 = v1.subtract_vector(&v0);
        let edge2 = v2.subtract_vector(&v0);
        let normal = edge1.cross_product(&edge2);

        normal.normalize()
    }

    fn get_pbr_shader(
        &self,
        light: &Light,
        triangle: &Triangle,
        viewer_position: Vector3D,
    ) -> Vector3D {
        let roughness: f64 = 0.2;
        let metallic: f64 = 0.8;
        let k_s: f64 = metallic;
        let k_d: f64 = 1.0 - k_s;
        let f0: f64 = 0.04;
        let constant_attenuation: f64 = 1.0;
        let linear_attenuation: f64 = 0.09;
        let quadratic_attenuation: f64 = 0.032;

        let light_dir = light.target.subtract_vector(&light.position);
        let light_dir = light_dir.normalize();

        let vertices: [Vector3D; 3] = triangle.vertices;
        let v0: Vector3D = vertices[0];
        let v1: Vector3D = vertices[1];
        let v2: Vector3D = vertices[2];

        let centroid: Vector3D = v0.add_vector(&v1).add_vector(&v2).divide(3.0);

        let edge1: Vector3D = v1.subtract_vector(&v0);
        let edge2: Vector3D = v2.subtract_vector(&v0);
        let normal: Vector3D = edge1.cross_product(&edge2);
        let normal: Vector3D = normal.normalize();

        let distance_vector = centroid.subtract_vector(&light.position);
        let distance = distance_vector.get_length();
        let distance_vector = distance_vector.normalize();

        let light_intensity = light.lumens / (distance * distance);
        let light_dir = light_dir.multiply(distance_vector.dot_product(&light_dir));
        let light_dir = light_dir.multiply(-1.0);

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
        let n_dot_vsq = n_dot_v * n_dot_v;
        let n_dot_lsq = n_dot_l * n_dot_l;
        let n_dot_hsq = n_dot_h * n_dot_h;

        let g1_denom = n_dot_v + ((1.0 - roughness_sq) * n_dot_vsq + roughness_sq).sqrt();
        let g2_denom = n_dot_l + ((1.0 - roughness_sq) * n_dot_lsq + roughness_sq).sqrt();

        let g1: f64 = 2.0 * n_dot_v / g1_denom;
        let g2: f64 = 2.0 * n_dot_l / g2_denom;

        let attenuation: f64 = 1.0
            / (constant_attenuation
                + linear_attenuation * distance
                + quadratic_attenuation * distance * distance);

        let ambient: Vector3D = light
            .ambient
            .multiply(light.lumens)
            .multiply(light_intensity);

        let diffuse: Vector3D = light
            .diffuse
            .multiply(diffuse_angle * light.lumens * attenuation)
            .multiply(light_intensity);

        let f: f64 = f0 + (1.0 - f0) * (1.0 - n_dot_h).powi(5);
        let g: f64 = g1 * g2;
        let d: f64 = roughness_sq
            / ((n_dot_hsq * (roughness_sq - 1.0) + 1.0) * (n_dot_hsq * (roughness_sq - 1.0) + 1.0));

        let specular: Vector3D = light
            .specular
            .multiply(f * g * d / (4.0 * n_dot_l * n_dot_v) * light_intensity * attenuation);

        let mut shader_vec: Vector3D = ambient
            .multiply(k_d)
            .add_vector(&diffuse.multiply(k_d))
            .add_vector(&specular.multiply(k_s));

        if shader_vec.x.is_nan() || shader_vec.y.is_nan() || shader_vec.z.is_nan() {
            shader_vec = Vector3D::new(0.0, 0.0, 0.0);
        }
        shader_vec
    }

    pub fn apply_pbr_lighting(&self, mesh: Mesh, light: &Light, viewer_position: Vector3D) -> Mesh {
        let mut mesh = mesh;

        let light_dir = light.target.subtract_vector(&light.position);
        let light_dir = light_dir.normalize();

        for polygon in &mut mesh.polygons {
            match polygon {
                Polygon::Quad(_) => continue,
                Polygon::Triangle(triangle) => {
                    let shader_vec = self.get_pbr_shader(light, triangle, viewer_position);
                    let shader = RGBA::from_vector(shader_vec);
                    triangle.shader = triangle.shader.average(&shader);
                }
            }
        }
        mesh
    }
}

// #[derive(Clone, Debug)]
// pub struct Shaders;

// impl Shaders {
//     pub fn apply_path_tracing(
//         mesh: &mut Mesh,
//         light: &Light,
//         viewer_position: Vector3D,
//         max_bounces: u32,
//     ) {
//         let constant_attenuation: f64 = 0.8;

//         for polygon in &mut mesh.polygons {
//             match polygon {
//                 Polygon::Quad(_) => continue,
//                 Polygon::Triangle(triangle) => {
//                     let mut accumulated_color = Vector3D::new(0.0, 0.0, 0.0);
//                     let mut current_energy = 1.0;

//                     for bounce in 0..max_bounces {
//                         let mut ray_origin = if bounce == 0 {
//                             viewer_position
//                         } else {
//                             triangle.vertices[0]
//                         };
//                         let mut ray_direction = if bounce == 0 {
//                             Self::triangle_centroid(triangle)
//                                 .subtract_vector(&ray_origin)
//                                 .normalize()
//                         } else {
//                             Self::reflect(ray_direction, &normal).normalize()
//                         };

//                         let t = Self::triangle_intersect(triangle, ray_origin, ray_direction);
//                         if t.is_none() {
//                             break;
//                         }

//                         let intersection =
//                             ray_origin.add_vector(&ray_direction.multiply(t.unwrap()));
//                         let normal = Self::triangle_normal(triangle);
//                         let distance = light.position.get_distance(&intersection);
//                         let light_intensity = light.lumens / (distance * distance);

//                         let light_to_intersection =
//                             light.position.subtract_vector(&intersection).normalize();
//                         let facing_light = normal.dot_product(&light_to_intersection) > 0.0;

//                         if facing_light {
//                             let color_contribution = light
//                                 .diffuse
//                                 .multiply(light.lumens)
//                                 .multiply(light_intensity)
//                                 .multiply(current_energy);
//                             accumulated_color = accumulated_color.add_vector(&color_contribution);
//                         }

//                         current_energy *= constant_attenuation;
//                     }

//                     if accumulated_color.x.is_nan()
//                         || accumulated_color.y.is_nan()
//                         || accumulated_color.z.is_nan()
//                     {
//                         accumulated_color = Vector3D::new(0.0, 0.0, 0.0);
//                     }

//                     let shader = RGBA::from_vector(accumulated_color);
//                     triangle.shader = triangle.shader.average(&shader);
//                 }
//             }
//         }
//     }

//     // pub fn apply_path_tracing(
//     //     mesh: Mesh,
//     //     light: &Light,
//     //     viewer_position: Vector3D,
//     //     max_bounces: u32,
//     // ) -> Mesh {
//     //     let mut mesh = mesh;

//     //     let constant_attenuation: f64 = 0.8;

//     //     for polygon in &mut mesh.polygons {
//     //         match polygon {
//     //             Polygon::Quad(_) => continue,
//     //             Polygon::Triangle(triangle) => {
//     //                 let mut accumulated_color = Vector3D::new(0.0, 0.0, 0.0);
//     //                 let mut current_energy = 1.0;

//     //                 for _ in 0..max_bounces {
//     //                     let mut ray_origin = viewer_position;
//     //                     let mut ray_direction = Self::triangle_centroid(triangle)
//     //                         .subtract_vector(&ray_origin)
//     //                         .normalize();

//     //                     let t = Self::triangle_intersect(triangle, ray_origin, ray_direction);
//     //                     if t.is_none() {
//     //                         break;
//     //                     }

//     //                     let intersection =
//     //                         ray_origin.add_vector(&ray_direction.multiply(t.unwrap()));
//     //                     let normal = Self::triangle_normal(triangle);
//     //                     let distance = light.position.get_distance(&intersection);
//     //                     let light_intensity = light.lumens / (distance * distance);

//     //                     let color_contribution = light
//     //                         .diffuse
//     //                         .multiply(light.lumens)
//     //                         .multiply(light_intensity)
//     //                         .multiply(current_energy);
//     //                     accumulated_color = accumulated_color.add_vector(&color_contribution);

//     //                     let next_ray_direction = Self::reflect(ray_direction, &normal);
//     //                     if next_ray_direction.dot_product(&normal) <= 0.0 {
//     //                         break;
//     //                     }

//     //                     ray_origin = intersection;
//     //                     ray_direction = next_ray_direction;
//     //                     current_energy *= constant_attenuation;
//     //                 }

//     //                 if accumulated_color.x.is_nan()
//     //                     || accumulated_color.y.is_nan()
//     //                     || accumulated_color.z.is_nan()
//     //                 {
//     //                     accumulated_color = Vector3D::new(0.0, 0.0, 0.0);
//     //                 }

//     //                 let shader = RGBA::from_vector(accumulated_color);
//     //                 triangle.shader = triangle.shader.average(&shader);
//     //             }
//     //         }
//     //     }
//     //     mesh
//     // }

//     // pub fn apply_path_tracing(
//     //     mesh: Mesh,
//     //     light: &Light,
//     //     viewer_position: Vector3D,
//     //     max_bounces: u32,
//     // ) -> Mesh {
//     //     let mut mesh = mesh;
//     //     let constant_attenuation: f64 = 0.8;

//     //     for polygon in &mut mesh.polygons {
//     //         match polygon {
//     //             Polygon::Quad(_) => continue,
//     //             Polygon::Triangle(triangle) => {
//     //                 let mut accumulated_color = Vector3D::new(0.0, 0.0, 0.0);
//     //                 let mut current_energy = 1.0;

//     //                 for _ in 0..max_bounces {
//     //                     let mut ray_origin = viewer_position;
//     //                     let mut ray_direction = Self::triangle_centroid(triangle)
//     //                         .subtract_vector(&ray_origin)
//     //                         .normalize();

//     //                     let t = Self::triangle_intersect(triangle, ray_origin, ray_direction);
//     //                     if t.is_none() {
//     //                         break;
//     //                     }

//     //                     let intersection =
//     //                         ray_origin.add_vector(&ray_direction.multiply(t.unwrap()));
//     //                     let normal = Self::triangle_normal(triangle);
//     //                     let light_dir = light.target.subtract_vector(&light.position).normalize();

//     //                     let distance = light.position.get_distance(&intersection);
//     //                     let light_intensity = light.lumens / (distance * distance);

//     //                     let light_angle: f64 = normal.dot_product(&light_dir.multiply(-1.0));
//     //                     let light_angle: f64 = light_angle.max(0.0);
//     //                     let color_contribution = light
//     //                         .diffuse
//     //                         .multiply(light.lumens * light_angle)
//     //                         .multiply(light_intensity)
//     //                         .multiply(current_energy);

//     //                     accumulated_color = accumulated_color.add_vector(&color_contribution);

//     //                     let next_ray_direction = Self::reflect(ray_direction, &normal);
//     //                     if next_ray_direction.dot_product(&normal) <= 0.0 {
//     //                         break;
//     //                     }

//     //                     ray_origin = intersection;
//     //                     ray_direction = next_ray_direction;
//     //                     current_energy *= constant_attenuation;
//     //                 }

//     //                 if accumulated_color.x.is_nan()
//     //                     || accumulated_color.y.is_nan()
//     //                     || accumulated_color.z.is_nan()
//     //                 {
//     //                     accumulated_color = Vector3D::new(0.0, 0.0, 0.0);
//     //                 }

//     //                 let shader = RGBA::from_vector(accumulated_color);
//     //                 triangle.shader = triangle.shader.average(&shader);
//     //             }
//     //         }
//     //     }
//     //     mesh
//     // }

//     fn triangle_centroid(triangle: &Triangle) -> Vector3D {
//         let (v0, v1, v2) = (
//             triangle.vertices[0],
//             triangle.vertices[1],
//             triangle.vertices[2],
//         );
//         v0.add_vector(&v1).add_vector(&v2).divide(3.0)
//     }

//     fn triangle_intersect(
//         triangle: &Triangle,
//         ray_origin: Vector3D,
//         ray_direction: Vector3D,
//     ) -> Option<f64> {
//         let edge1 = triangle.vertices[1].subtract_vector(&triangle.vertices[0]);
//         let edge2 = triangle.vertices[2].subtract_vector(&triangle.vertices[0]);

//         let pvec = ray_direction.cross_product(&edge2);
//         let det = edge1.dot_product(&pvec);

//         if det.abs() < std::f64::EPSILON {
//             return None;
//         }

//         let inv_det = 1.0 / det;
//         let tvec = ray_origin.subtract_vector(&triangle.vertices[0]);
//         let u = tvec.dot_product(&pvec) * inv_det;

//         if u < 0.0 || u > 1.0 {
//             return None;
//         }

//         let qvec = tvec.cross_product(&edge1);
//         let v = ray_direction.dot_product(&qvec) * inv_det;

//         if v < 0.0 || u + v > 1.0 {
//             return None;
//         }

//         Some(edge2.dot_product(&qvec) * inv_det)
//     }

//     fn triangle_normal(triangle: &Triangle) -> Vector3D {
//         let v0 = triangle.vertices[0];
//         let v1 = triangle.vertices[1];
//         let v2 = triangle.vertices[2];

//         let edge1 = v1.subtract_vector(&v0);
//         let edge2 = v2.subtract_vector(&v0);

//         edge1.cross_product(&edge2).normalize()
//     }

//     fn reflect(vector: Vector3D, normal: &Vector3D) -> Vector3D {
//         let dot = vector.dot_product(normal);
//         vector.subtract_vector(&normal.multiply(2.0 * dot))
//     }
