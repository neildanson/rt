use glam::{const_vec3a, Vec3A};
use pixel_canvas::{input::MouseState, Canvas, Color, XY};
use rayon::prelude::*;
use std::ops::IndexMut;

mod aabb;
mod camera;
mod intersection;
mod node;
mod ray;
mod shape;
mod sphere;
mod bounds;

use aabb::AABB;
use camera::Camera;
use intersection::Intersection;
use node::Node;
use ray::Ray;
use shape::Shape;
use sphere::Sphere;
use bounds::Bounds;

const AMBIENT_LIGHT: Vec3A = const_vec3a!([0.5, 0.5, 0.5]);

struct Light {
    position: Vec3A,
    color: Vec3A,
}

#[inline]
fn to_color(vec: Vec3A) -> Color {
    let rgb = vec.min(Vec3A::one()).max(Vec3A::zero()) * 255.0;
    let (red,green,blue) = rgb.into();
    Color {
        r: red as u8,
        g: green as u8,
        b: blue as u8,
    }
}

fn any_intersection(ray: Ray, nodes: &[Node]) -> bool {
    nodes
        .iter()
        .any(|nodes| nodes.intersects(ray).is_some())
}

fn nearest_intersection(ray: Ray, nodes: &[Node]) -> Option<Intersection> {
    nodes
        .iter()
        .filter_map(|node| node.intersects(ray))
        .min()
}

fn apply_light(
    position: Vec3A,
    normal: Vec3A,
    objects: &[Node],
    light: &Light,
    ray_direction: Vec3A,
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
    objects: &[Node],
    lights: &[Light],
    ray_direction: Vec3A,
) -> Vec3A {
    let mut color = Vec3A::zero();
    for light in lights {
        color += apply_light(position, normal, objects, &light, ray_direction)
    }
    color
}

fn trace(ray: Ray, objects: &[Node], lights: &[Light], depth: i32) -> Vec3A {
    let intersection = nearest_intersection(ray, objects);
    match intersection {
        Some(intersection) => {
            let hit_point = intersection.hit_point();

            let normal = intersection.normal(hit_point);
            let color = apply_lighting(
                hit_point,
                normal, // intersection.normal,
                objects,
                lights,
                intersection.ray.direction,
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
    objects: &[Node],
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
    let position = Vec3A::new(0.0,2.0,0.0);

    #[cfg(vec3a_sse2)]
    {
        println("SSE2");
    }

    let canvas = Canvas::new(width, height)
        .title("Raytrace")
        .state(MouseState::new())
        .show_ms(true)
        .input(MouseState::handle_input);

    let scene = vec![
        Node::new(
            //AABB::new(Vec3A::new(-6.0, 0.0, -5.0), Vec3A::new(6.0, 6.0, 10.0)),
            vec![
                Sphere::new(Vec3A::new(0.0, 3.0, 5.0), 1.0),
                Sphere::new(Vec3A::new(2.0, 1.0, 5.0), 1.0),
                Sphere::new(Vec3A::new(2.0, 1.0, 8.0), 1.0),
            ],
        ),
        Node::new(
            //AABB::new(Vec3A::new(-1000.0, -10.0, -1000.0), Vec3A::new(1000.0, 0.0, 1000.0)),
            vec![Sphere::new(Vec3A::new(0.0, -1003.0, 0.0), 1000.0)],
        ),
    ];

    let lights = vec![
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
    ];

    let fragment_height = height / 4;
    let mut work :Vec<(usize, usize, usize, usize)> = Vec::new();
    for frag in 0 .. 4 {
        work.push((0,(frag)*fragment_height, width, (frag+1)*fragment_height ));
    }

    canvas.render(move |mouse, image| {
        let look_x = (half_width - mouse.x as f32) / 200f32;
        let look_y = (half_height - mouse.y as f32) / 200f32;
        let look_at = Vec3A::new(look_x, look_y, 1f32);

        let camera =
            Camera::create_camera(position, look_at, inverse_height, half_width, half_height);

        let result = work
            .par_iter()
            .map(|(min_x, min_y, max_x, max_y)| {
                trace_region(*min_x, *min_y, *max_x, *max_y, &camera, &scene, &lights)
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
