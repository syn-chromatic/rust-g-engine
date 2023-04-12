use crate::abstracts::body::Body;
use crate::abstracts::body::BodyType;
use crate::components::color::RGBA;
use crate::components::model::OBJModelFormat;
use crate::components::shaders::Light;
use crate::components::shape::Shape;
use crate::components::vectors::Vector3D;
use crate::components::vertices::GridHorizontal;
use crate::components::vertices::Sphere;

use rand::rngs::ThreadRng;
use rand::Rng;

pub fn get_grid() -> BodyType {
    let mut grid = GridHorizontal::new(50, 50, 200.0);

    grid.set_offset(-1000.0, -100.0, -1000.0);
    let mesh = grid.get_triangle_polygons();

    let body = Shape::new(mesh);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_obj(file_path: &str) -> BodyType {
    let mut obj = OBJModelFormat::new(file_path, 200.0);
    let position = (2000.0, -500_000.0, 500_000.0);
    obj.set_offset(position.0, position.1, position.2);
    // obj.set_offset(2000.0, -100.0, 4000.0);
    obj.set_rotation(-90.0, 0.0, 0.0);
    // obj.set_rotation(0.0, -110.0, 0.0);
    // obj.set_offset(800.0, 800.0, 0.0);

    let mesh = obj.get_polygons();

    let mut body = Shape::new(mesh);
    body.physics().set_mass(500_000_000.0);
    body.physics()
        .set_position(position.0, position.1, position.2);
    body.physics().set_velocity(2500.0, 1.0, 500.0);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere() -> BodyType {
    let mut sphere = Sphere::new(500.0, 50, 50);
    let offset = 1000.0;
    sphere.set_offset(50.0 + offset, 5_000.0 + offset, 20000.0 + offset);

    sphere.set_color(RGBA::from_rgb(1.0, 0.2, 0.2));
    let mut mesh = sphere.get_triangle_mesh();

    let mut body = Shape::new(mesh);
    body.physics().set_velocity(20_000.0, 0.0, -1000.0);
    // body.physics().set_acceleration(100.0, 0.0, 100.0);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere_light_highmass() -> BodyType {
    let position: Vector3D = Vector3D::new(2000.0, -100.0, 4000.0);

    let mut light = Light::get_light();
    light.position = position;
    let light_dir: Vector3D = light.target.subtract_vector(&light.position);
    let light_dir: Vector3D = light_dir.normalize();

    let (lx, ly, lz) = light.position.to_tuple();
    let (dx, dy, dz) = light_dir.to_tuple();
    let offset = 5000.0;

    let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
    let mass = 5_000_000_000_000.0;

    let mut sphere = Sphere::new(80_000.0, 50, 50);
    sphere.set_offset(position.0, position.1, position.2);
    sphere.set_color(RGBA::from_rgb(1.0, 0.2, 0.2));
    let mut mesh = sphere.get_triangle_mesh();
    // mesh.add_light(light);

    let mut body = Shape::new(mesh);
    // body.physics().set_velocity(50_000.0, 10_000.0, 100_000.0);
    // body.physics().set_velocity(2500.0, 1.0, 5000.0);
    body.physics()
        .set_position(position.0, position.1, position.2);
    body.physics().set_mass(mass);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere_light1() -> BodyType {
    let position: Vector3D = Vector3D::new(50_000.0, 5_000.0, 20_000.0);

    let mut light = Light::get_light();
    light.position = position;
    let light_dir: Vector3D = light.target.subtract_vector(&light.position);
    let light_dir: Vector3D = light_dir.normalize();

    let (lx, ly, lz) = light.position.to_tuple();
    let (dx, dy, dz) = light_dir.to_tuple();
    let offset = 5000.0;

    let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
    let mass = 500_000_000.0;

    let mut sphere = Sphere::new(5_000.0, 50, 50);
    sphere.set_offset(position.0, position.1, position.2);
    // let mut mesh = sphere.get_triangle_mesh(RGBA::from_rgb(1.0, 0.2, 0.2));
    sphere.set_color(RGBA::from_random());
    let mut mesh = sphere.get_triangle_mesh();
    // mesh.add_light(light);

    let mut body = Shape::new(mesh);
    body.physics().set_velocity(100_000.0, 20_000.0, 200_000.0);
    // body.physics().set_velocity(2500.0, 1.0, 5000.0);
    body.physics()
        .set_position(position.0, position.1, position.2);
    body.physics().set_mass(mass);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere_light2() -> BodyType {
    let position: Vector3D = Vector3D::new(5000.0, 0.0, 0.0);

    let mut light = Light::get_light();
    light.position = position;
    let light_dir: Vector3D = light.target.subtract_vector(&light.position);
    let light_dir: Vector3D = light_dir.normalize();

    let (lx, ly, lz) = light.position.to_tuple();
    let (dx, dy, dz) = light_dir.to_tuple();
    let offset = 5000.0;

    let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
    let mass = 5_000.0;

    let mut sphere = Sphere::new(5_000.0, 50, 50);
    sphere.set_offset(position.0, position.1, position.2);
    // let mut mesh = sphere.get_triangle_mesh(RGBA::from_rgb(1.0, 0.2, 0.2));
    sphere.set_color(RGBA::from_random());
    let mut mesh = sphere.get_triangle_mesh();
    // mesh.add_light(light);

    let mut body = Shape::new(mesh);
    // body.physics().set_velocity(500_000.0, 20_000.0, 200_000.0);
    body.physics().set_velocity(-2500.0, 0.0, 10_000.0);
    body.physics()
        .set_position(position.0, position.1, position.2);
    body.physics().set_mass(mass);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn get_sphere_light3() -> BodyType {
    let position: Vector3D = Vector3D::new(-500_000.0, 0.0, 0.0);

    let mut light = Light::get_light();
    light.position = position;
    let light_dir: Vector3D = light.target.subtract_vector(&light.position);
    let light_dir: Vector3D = light_dir.normalize();

    let (lx, ly, lz) = light.position.to_tuple();
    let (dx, dy, dz) = light_dir.to_tuple();
    let offset = 5000.0;

    let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
    // let mass = 2_000.0;
    let mut rng: ThreadRng = rand::thread_rng();
    let mass: f64 = rng.gen_range(100.0..6_000.0);

    let radius = (mass / 1000.0) * 2000.0;

    // let mut sphere = Sphere::new(5_000.0, 10, 10);
    let mut sphere = Sphere::new(radius, 10, 10);
    sphere.set_offset(position.0, position.1, position.2);
    // let mut mesh = sphere.get_triangle_mesh(RGBA::from_rgb(1.0, 0.2, 0.2));
    sphere.set_color(RGBA::from_random());
    let mut mesh = sphere.get_triangle_mesh();
    // mesh.add_light(light);

    let mut body = Shape::new(mesh);
    // body.physics().set_velocity(-500_000.0, -20_000.0, 200_000.0);
    body.physics().set_velocity(250.0, 1.0, 2000.0);
    body.physics()
        .set_position(position.0, position.1, position.2);
    body.physics().set_mass(mass);
    let body_type = BodyType::Shape(body);
    body_type
}

pub fn orbiting_system(position: Vector3D) -> Vec<BodyType> {
    pub fn get_sphere_light_highmass(mut position: Vector3D) -> BodyType {
        position.x += 2000.0;
        position.y += -100.0;
        position.z += 4000.0;

        let mut light = Light::get_light();
        light.position = position;
        let light_dir: Vector3D = light.target.subtract_vector(&light.position);
        let light_dir: Vector3D = light_dir.normalize();

        let (lx, ly, lz) = light.position.to_tuple();
        let (dx, dy, dz) = light_dir.to_tuple();
        let offset = 5000.0;

        let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
        let mass = 8_000_000_000_000.0;

        let mut sphere = Sphere::new(150_000.0, 20, 20);
        sphere.set_offset(position.0, position.1, position.2);
        // sphere.set_color(RGBA::from_rgb(1.0, 0.2, 0.2));
        sphere.set_color(RGBA::from_random());
        sphere.set_shader(RGBA::from_rgb(0.5, 0.5, 0.5));
        let mut mesh = sphere.get_triangle_mesh();
        // mesh.add_light(light);
        let mut body = Shape::new(mesh);
        body.physics()
            .set_position(position.0, position.1, position.2);
        body.physics().set_mass(mass);
        // body.physics().set_velocity(-250.0, 1.0, -2000.0);
        let body_type = BodyType::Shape(body);
        body_type
    }

    pub fn get_sphere_light3(mut position: Vector3D) -> BodyType {
        position.x += -1_000_000.0;
        position.y += 0.0;
        position.z += 0.0;

        let mut light = Light::get_light();
        light.position = position;
        let light_dir: Vector3D = light.target.subtract_vector(&light.position);
        let light_dir: Vector3D = light_dir.normalize();

        let (lx, ly, lz) = light.position.to_tuple();
        let (dx, dy, dz) = light_dir.to_tuple();
        let offset = 5000.0;

        let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
        let mut rng: ThreadRng = rand::thread_rng();
        let mass: f64 = rng.gen_range(1000.0..8_000.0);

        let radius = (mass / 1000.0) * 2000.0;
        let mut sphere = Sphere::new(radius, 3, 3);
        sphere.set_offset(position.0, position.1, position.2);
        sphere.set_color(RGBA::from_random());
        sphere.set_shader(RGBA::from_rgb(0.5, 0.5, 0.5));
        let mut mesh = sphere.get_triangle_mesh();

        let mut body = Shape::new(mesh);
        body.physics().set_velocity(250.0, 1.0, 2000.0);
        body.physics()
            .set_position(position.0, position.1, position.2);
        body.physics().set_mass(mass);
        let body_type = BodyType::Shape(body);
        body_type
    }

    let mut objects: Vec<BodyType> = Vec::new();
    let high_mass = get_sphere_light_highmass(position.clone());
    objects.push(high_mass);

    for i in 0..250 {
        let sphere = get_sphere_light3(position.clone());
        objects.push(sphere);
    }
    objects
}

pub fn orbiting_system2(position: Vector3D) -> Vec<BodyType> {
    pub fn get_sphere_light_highmass(mut position: Vector3D) -> BodyType {
        position.x += 2000.0;
        position.y += -100.0;
        position.z += 4000.0;

        let mut light = Light::get_light();
        light.position = position;
        let light_dir: Vector3D = light.target.subtract_vector(&light.position);
        let light_dir: Vector3D = light_dir.normalize();

        let (lx, ly, lz) = light.position.to_tuple();
        let (dx, dy, dz) = light_dir.to_tuple();
        let offset = 5000.0;

        let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
        let mass = 8_000_000_000_000.0;

        let mut sphere = Sphere::new(150_000.0, 20, 20);
        sphere.set_offset(position.0, position.1, position.2);
        // sphere.set_color(RGBA::from_rgb(1.0, 0.2, 0.2));
        sphere.set_color(RGBA::from_random());
        sphere.set_shader(RGBA::from_rgb(0.5, 0.5, 0.5));

        let mut mesh = sphere.get_triangle_mesh();
        // mesh.add_light(light);

        let mut body = Shape::new(mesh);
        body.physics()
            .set_position(position.0, position.1, position.2);
        body.physics().set_mass(mass);
        body.physics().set_velocity(0.0, 200.0, 0.0);
        let body_type = BodyType::Shape(body);
        body_type
    }

    pub fn get_sphere_light3(mut position: Vector3D) -> BodyType {
        position.x += -1_000_000.0;
        position.y += 0.0;
        position.z += 0.0;

        let mut light = Light::get_light();
        light.position = position;
        let light_dir: Vector3D = light.target.subtract_vector(&light.position);
        let light_dir: Vector3D = light_dir.normalize();

        let (lx, ly, lz) = light.position.to_tuple();
        let (dx, dy, dz) = light_dir.to_tuple();
        let offset = 5000.0;

        let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
        let mut rng: ThreadRng = rand::thread_rng();
        let mass: f64 = rng.gen_range(1000.0..8_000.0);

        let radius = (mass / 1000.0) * 2000.0;
        let mut sphere = Sphere::new(radius, 3, 3);
        sphere.set_offset(position.0, position.1, position.2);
        sphere.set_color(RGBA::from_random());
        sphere.set_shader(RGBA::from_rgb(0.5, 0.5, 0.5));
        let mut mesh = sphere.get_triangle_mesh();

        let mut body = Shape::new(mesh);
        body.physics().set_velocity(250.0, 1.0, 2000.0);
        body.physics()
            .set_position(position.0, position.1, position.2);
        body.physics().set_mass(mass);
        let body_type = BodyType::Shape(body);
        body_type
    }

    let mut objects: Vec<BodyType> = Vec::new();
    let high_mass = get_sphere_light_highmass(position.clone());
    objects.push(high_mass);

    for i in 0..250 {
        let sphere = get_sphere_light3(position.clone());
        objects.push(sphere);
    }
    objects
}

pub fn highmass_planet(position: Vector3D) -> BodyType {
    pub fn get_sphere_light_highmass(mut position: Vector3D) -> BodyType {
        position.x += 2000.0;
        position.y += -100.0;
        position.z += 4000.0;

        let mut light = Light::get_light();
        light.position = position;
        let light_dir: Vector3D = light.target.subtract_vector(&light.position);
        let light_dir: Vector3D = light_dir.normalize();

        let (lx, ly, lz) = light.position.to_tuple();
        let (dx, dy, dz) = light_dir.to_tuple();
        let offset = 5000.0;

        let position = (lx - (offset * dx), ly - (offset * dy), lz - (offset * dz));
        let mass = 4_000_000_000_000.0;

        let mut sphere = Sphere::new(500_000.0, 20, 20);
        sphere.set_offset(position.0, position.1, position.2);
        // sphere.set_color(RGBA::from_rgb(1.0, 0.2, 0.2));
        sphere.set_color(RGBA::from_random());
        let mut mesh = sphere.get_triangle_mesh();
        mesh.add_light(light);

        let mut body = Shape::new(mesh);
        body.physics()
            .set_position(position.0, position.1, position.2);
        body.physics().set_mass(mass);
        let body_type = BodyType::Shape(body);
        body_type
    }

    let high_mass = get_sphere_light_highmass(position.clone());
    high_mass
}
