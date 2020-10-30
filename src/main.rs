use glam::Vec3A;
use pixel_canvas::{input::MouseState, Canvas, Color, XY};
use rayon::prelude::*;
use std::ops::IndexMut;

mod ray;
mod intersection;
mod camera;
mod sphere;

use ray::Ray;
use intersection::Intersection;
use camera::Camera;
use sphere::Sphere;

struct Light {
    position: Vec3A,
    color: Vec3A,
}

#[inline]
fn to_color(vec: Vec3A) -> Color {
    let (x, y, z) = vec.into();
    let red = x.min(1.0).max(0.0) * 255.0;
    let green = y.min(1.0).max(0.0) * 255.0;
    let blue = z.min(1.0).max(0.0) * 255.0;

    Color {
        r: red as u8,
        g: green as u8,
        b: blue as u8,
    }
}

//Used for shadows only
fn dirty_intersects(ray: Ray, object: &Sphere) -> bool {
    let diff = object.center - ray.position;
    let v = diff.dot(ray.direction);
    if v < 0.0 {
        false
    } else {
        let distance_squared = object.radius_squared - (diff.dot(diff) - v.powi(2));
        distance_squared >= 0.0
    }
}

fn any_intersection(ray: Ray, objects: &[Sphere]) -> bool {
    objects.iter().any(|object| { object.intersects(ray).is_some() })
}

fn nearest_intersection(ray: Ray, objects: &[Sphere]) -> Option<Intersection> {
    objects.iter().filter_map(|object| { object.intersects(ray) }).min()
}

fn apply_light(
    position: Vec3A,
    normal: Vec3A,
    objects: &[Sphere],
    light: &Light,
    ray_direction: Vec3A,
    base_color: Vec3A,
) -> Vec3A {
    let light_dir = (light.position - position).normalize();
    let ray = Ray {
        position,
        direction: light_dir,
    };
    let is_in_shadow = any_intersection(ray, objects);
    if is_in_shadow {
        Vec3A::zero()
    } else {
        let illum = light_dir.dot(normal);
        let diffuse_color = if illum > 0.0 {
            light.color * illum * base_color
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
    objects: &[Sphere],
    lights: &[Light],
    ray_direction: Vec3A,
    base_color: Vec3A,
) -> Vec3A {
    //lights.iter().map(|light| apply_light(position, normal, objects, &light, ray_direction, base_color)).sum()
    
    let mut color = Vec3A::zero();
    for light in lights {
        color += apply_light(position, normal, objects, &light, ray_direction, base_color)
    }
    color
    //lights.iter().fold(Vec3A::zero(), |color, light| {
    //    color + apply_light(position, normal, objects, &light, ray_direction, base_color)
    //})
}

fn trace(ray: Ray, objects: &[Sphere], lights: &[Light], depth: i32) -> Vec3A {
    let intersection = nearest_intersection(ray, objects);
    match intersection {
        Some(intersection) => {
            let hit_point =
                intersection.ray.position + (intersection.ray.direction * intersection.distance());

            let normal = intersection.normal();
            let color = Vec3A::new(0.5, 0.5, 0.5);
            let color = apply_lighting(
                hit_point,
                normal, // intersection.normal,
                objects,
                lights,
                intersection.ray.direction,
                color,
            );
            if depth < 3 {
                let ray = Ray {
                    position: hit_point,
                    direction: normal,
                };
                let newcolor = trace(ray, objects, lights, depth + 1);
                color + newcolor
            } else {
                color
            }
        }
        None => Vec3A::zero(),
    }
}

fn trace_region(
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    camera: &Camera,
    objects: &[Sphere],
    lights: &[Light],
) -> Vec<(usize, usize, Vec3A)> {
    let mut result = Vec::with_capacity((max_x - min_x) * (max_y - min_y));
    for y in min_y..max_y {
        let yf = y as f32;
        for x in min_x..max_x {
            let ray = camera.get_ray(x as f32, yf);

            result.push((x, y, trace(ray, objects, lights, 0)));
        }
    }

    result
}

fn main() {
    let width = 800;
    let height = 600;
    let inverse_height = 1.0f32 / height as f32;
    let half_height = height as f32 / 2.0f32;
    let half_width = width as f32 / 2.0f32;
    let position = Vec3A::zero();

    let canvas = Canvas::new(width, height)
        .title("Raytrace")
        .state(MouseState::new())
        .show_ms(true)
        .input(MouseState::handle_input);

    let objects = vec![
        Sphere::new(
            Vec3A::new(0.0, 2.0, -5.0),
            1.0),
        Sphere::new(
            Vec3A::new(2.0, 0.0, -5.0),
            1.0),
        Sphere::new(
            Vec3A::new(0.0, -1003.0, 0.0),
            1000.0),
    ];

    let lights = vec![
        Light {
            position: Vec3A::new(-3.0, 3.0, -1.0),
            color: Vec3A::new(0.5, 0.0, 0.0),
        },
        Light {
            position: Vec3A::new(3.0, 3.0, -1.0),
            color: Vec3A::new(0.7, 0.7, 0.7),
        },
    ];

    let work: Vec<(usize, usize, usize, usize)> = vec![
            (0, 0, 800, 150),
            (0, 150, 800, 300),
            (0, 300, 800, 450),
            (0, 450, 800, 600),
        ];

    canvas.render(move |mouse, image| {
        let look_x = (half_width - mouse.x as f32) / 200f32;
        let look_y = (half_height - mouse.y as f32) / 200f32;
        let look_at = Vec3A::new(look_x, look_y, -1f32);

        let camera = Camera::create_camera(position, look_at, inverse_height, half_width, half_height);
        
        let result = work
            .par_iter()
            .map(|(min_x, min_y, max_x, max_y)| {
                trace_region(*min_x, *min_y, *max_x, *max_y, &camera, &objects, &lights)
            })
            .collect::<Vec<_>>();

        for r in result {
            for (x, y, col) in r {
                let color = image.index_mut(XY(x, y));
                *color = to_color(col);
            }
        }
    });
}
