use glam::{const_vec3a, Vec3A};
use rayon::prelude::*;
use std::ops::IndexMut;
use bvh::bvh::BVH;
use bvh::nalgebra::{Point3, Vector3};

pub mod camera;
pub mod intersection;
pub mod ray;
pub mod shape;
pub mod sphere;

pub use camera::Camera;
pub use intersection::Intersection;
pub use ray::Ray;
pub use shape::Shape;
pub use sphere::Sphere;

const AMBIENT_LIGHT: Vec3A = const_vec3a!([0.5, 0.5, 0.5]);
const RELECTION_DEPTH: u32 = 1;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const INVERSE_HEIGHT: f32 = 1.0f32 / HEIGHT as f32;
const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.0f32;
const HALF_WIDTH: f32 = WIDTH as f32 / 2.0f32;
const CAMERA_POSITION: Vec3A = const_vec3a!([0.0, 2.0, 0.0]);

pub struct Light {
    position: Vec3A,
    color: Vec3A,
}

struct Scene {
    bvh:BVH, 
    shapes : Vec<Sphere>
}

fn any_intersection(ray: Ray, scene: &Scene) -> bool {
    let origin : [f32;3] = ray.position.into();
    let direction : [f32;3] = ray.direction.into();
    let ray2 = bvh::ray::Ray::new(Point3::from(origin), Vector3::from(direction));

    scene.bvh.traverse(&ray2, &scene.shapes).iter().any(|node| node.intersects(ray).is_some())
}

fn nearest_intersection(ray: Ray, scene: &Scene) -> Option<Intersection> {
    //nodes.iter().filter_map(|node| node.intersects(ray)).min()
    let origin : [f32;3] = ray.position.into();
    let direction : [f32;3] = (ray.direction).into();
    let ray2 = bvh::ray::Ray::new(Point3::from(origin), Vector3::from(direction));    
    scene.bvh.traverse(&ray2, &scene.shapes).iter().filter_map(|node| node.intersects(ray)).min()
}

fn apply_light(
    position: Vec3A,
    normal: Vec3A,
    scene: &Scene,
    light: &Light,
    ray_direction: Vec3A,
) -> Vec3A {
    let light_dir = (light.position - position).normalize();
    let ray = Ray {
        position,
        direction: light_dir,
    };
    let is_in_shadow = any_intersection(ray, scene);
    if is_in_shadow {
        Vec3A::zero()
    } else {
        let illum = light_dir.dot(normal);
        let diffuse_color = if illum > 0.0 {
            light.color * illum * AMBIENT_LIGHT
        } else {
            Vec3A::zero()
        };

        let dot = normal.dot(ray_direction);
        let ray_direction = (ray_direction - (normal * (2.0 * dot))).normalize();
        let specular = light_dir.dot(ray_direction);
        let specular_result = if specular > 0.0 {
            light.color * specular.powi(50)
        } else {
            Vec3A::zero()
        };
        diffuse_color + specular_result
    }
}

fn apply_lighting(
    position: Vec3A,
    normal: Vec3A,
    scene: &Scene,
    lights: &[Light],
    ray_direction: Vec3A,
) -> Vec3A {
    let mut color = Vec3A::zero();
    for light in lights {
        color += apply_light(position, normal, scene, &light, ray_direction)
    }
    color
}

fn trace(ray: Ray, scene: &Scene, lights: &[Light], depth: u32) -> Vec3A {
    let intersection = nearest_intersection(ray, scene);
    match intersection {
        Some(intersection) => {
            let hit_point = intersection.hit_point();

            let normal = intersection.normal(hit_point);
            let color = apply_lighting(
                hit_point,
                normal, // intersection.normal,
                scene,
                lights,
                intersection.ray.direction,
            );
            if depth < RELECTION_DEPTH {
                let ray = Ray {
                    position: hit_point,
                    direction: normal,
                };
                let newcolor = trace(ray, scene, lights, depth + 1);
                color + newcolor
            } else {
                color
            }
        }
        None => Vec3A::zero(),
    }
}

fn trace_region(
    minmax: &(usize, usize, usize),
    camera: &Camera,
    scene: &Scene,
    lights: &[Light],
) -> Vec<(usize, usize, Vec3A)> {
    let mut result = Vec::with_capacity(minmax.0 * (minmax.2 - minmax.1));
    for y in minmax.1..minmax.2 {
        let yf = y as f32;
        for x in 0..minmax.0 {
            let ray = camera.get_ray(x as f32, yf);

            result.push((x, y, trace(ray, scene, lights, 0)));
        }
    }

    result
}

fn get_nodes() -> Scene {
    let mut shapes = Vec::new();
    for x in 3.. 16 {
        for z in 3 ..16 {
            let x = x as f32;
            let z = z as f32;
            let shape = Sphere::new(Vec3A::new(x * 2.0, 3.0, z * 2.0), 1.0);
            shapes.push(shape);
        }
    }
    //let mut shapes = vec![
    //        Sphere::new(Vec3A::new(0.0, 3.0, 5.0), 1.0),
    //        Sphere::new(Vec3A::new(2.0, 1.0, 5.0), 1.0),
    //        Sphere::new(Vec3A::new(2.0, 1.0, 8.0), 1.0),
    //];
    shapes.push(
        Sphere::new(Vec3A::new(0.0, -1003.0, 0.0), 1000.0));

    let bvh = BVH::build(&mut shapes);
    
    Scene { bvh, shapes }
}

fn get_lights() -> Vec<Light> {
    vec![
        Light {
            position: Vec3A::new(-3.0, 3.0, 1.0),
            color: Vec3A::new(0.5, 0.0, 0.0),
        },
        Light {
            position: Vec3A::new(3.0, 3.0, 1.0),
            color: Vec3A::new(0.0, 0.4, 0.0),
        },
        Light {
            position: Vec3A::new(0.0, 3.0, -1.0),
            color: Vec3A::new(0.0, 0.0, 0.5),
        },
    ]
}


#[inline]
fn to_pb_color(vec: Vec3A) -> pixel_canvas::Color {
    let rgb = vec.min(Vec3A::one()).max(Vec3A::zero()) * 255.0;
    let (red, green, blue) = rgb.into();
    pixel_canvas::Color {
        r: red as u8,
        g: green as u8,
        b: blue as u8,
    }
}

fn run_pixel_canvas() {
    use pixel_canvas::{input::MouseState, Canvas, XY};
    let canvas = Canvas::new(WIDTH, HEIGHT)
        .title("Raytrace")
        .state(MouseState::new())
        .show_ms(true)
        .input(MouseState::handle_input);
    let num_cpus = num_cpus::get();
    let fragment_height = HEIGHT / num_cpus;
    let mut work: Vec<(usize, usize, usize)> = Vec::new();
    let nodes = get_nodes();
    let lights = get_lights();
    for frag in 0..num_cpus {
        work.push((WIDTH, frag * fragment_height, (frag + 1) * fragment_height));
    }

    canvas.render(move |mouse, image| {
        let look_x = (HALF_WIDTH - mouse.x as f32) / 200f32;
        let look_y = (HALF_HEIGHT - mouse.y as f32) / 200f32;
        let look_at = Vec3A::new(look_x, look_y, 1f32);

        let camera = Camera::create_camera(
            CAMERA_POSITION,
            look_at,
            INVERSE_HEIGHT,
            HALF_WIDTH,
            HALF_HEIGHT,
        );

        let result = work
            .par_iter()
            .map(|minmax| trace_region(minmax, &camera, &nodes, &lights))
            .collect::<Vec<_>>();

        for r in result {
            for (x, y, col) in r {
                let color = image.index_mut(XY(x, y));
                *color = to_pb_color(col);
            }
        }
    });
}

#[inline]
fn to_fb_color(vec: Vec3A) -> u32 {
    let rgb = vec.min(Vec3A::one()).max(Vec3A::zero()) * 255.0;
    let (red, green, blue) = rgb.into();
    (255 << 24) | ((red as u32) << 16) | ((green as u32) << 8) | ((blue as u32) )  
}

fn run_minifb() {
    use minifb::{Key, Window, WindowOptions, MouseMode};
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let num_cpus = num_cpus::get();
    let fragment_height = HEIGHT / num_cpus;
    let mut work: Vec<(usize, usize, usize)> = Vec::new();
    let nodes = get_nodes();
    let lights = get_lights();

    for frag in 0..num_cpus {
        work.push((WIDTH, frag * fragment_height, (frag + 1) * fragment_height));
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (mouse_x, mouse_y) = window.get_mouse_pos(MouseMode::Pass).unwrap_or((0.0f32,0.0f32));
        let look_x = (HALF_WIDTH - mouse_x) / 200f32;
        let look_y = (HALF_HEIGHT - mouse_y) / 200f32;
        let look_at = Vec3A::new(look_x, look_y, 1f32);

        let camera = Camera::create_camera(
            CAMERA_POSITION,
            look_at,
            INVERSE_HEIGHT,
            HALF_WIDTH,
            HALF_HEIGHT,
        );

        let result = work
            .par_iter()
            .map(|minmax| trace_region(minmax, &camera, &nodes, &lights))
            .collect::<Vec<_>>();

        for r in result {
            for (x, y, col) in r {
                let color = buffer.index_mut(WIDTH * (HEIGHT - 1 - y) + x);
                *color = to_fb_color(col);
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}


pub fn run() {
    run_pixel_canvas();
}
