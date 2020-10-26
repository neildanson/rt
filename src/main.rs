use glam::Vec3A;
use pixel_canvas::{input::MouseState, Canvas, Color, XY};
use rayon::prelude::*;
use std::ops::IndexMut;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct Ray {
    position: Vec3A,
    direction: Vec3A,
}

#[derive(Copy, Clone)]
struct Intersection {
    ray: Ray,
    distance: f32,
    normal: Vec3A,
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance <= other.distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Intersection {}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

struct Sphere {
    center: Vec3A,
    radius: f32,
}

struct Light {
    position: Vec3A,
    color: Vec3A,
}

struct Camera {
    position: Vec3A,
    forward: Vec3A,
    right: Vec3A,
    up: Vec3A,

    //private
    half_width: f32,
    half_height: f32,
}

impl Camera {
    fn get_ray(&self, x: f32, y: f32) -> Ray {
        let right = self.right * recenter_x(x, self.half_width);
        let up = self.up * recenter_y(y, self.half_height);
        Ray {
            position: self.position,
            direction: (right + up + self.forward).normalize(),
        }
    }
}

fn normal(sphere: &Sphere, position: Vec3A) -> Vec3A {
    (position - sphere.center).normalize()
}

fn create_camera(
    position: Vec3A,
    look_at: Vec3A,
    inverse_height: f32,
    half_width: f32,
    half_height: f32,
) -> Camera {
    let forward = (look_at - position).normalize();
    let down = Vec3A::unit_y();
    let right = forward.cross(down).normalize() * 1.5f32 * inverse_height;
    let up = forward.cross(right).normalize() * 1.5f32 * inverse_height;

    Camera {
        position,
        forward,
        right,
        up,
        half_width,
        half_height,
    }
}

fn recenter_x(x: f32, half_width: f32) -> f32 {
    x - half_width
}

fn recenter_y(y: f32, half_height: f32) -> f32 {
    half_height - y
}

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

fn dirty_intersects(ray: Ray, object: &Sphere) -> bool {
    let diff = object.center - ray.position;
    let v = diff.dot(ray.direction);
    if v < 0.0 {
        false
    } else {
        let distance_squared = object.radius.powi(2) - (diff.dot(diff) - v.powi(2));
        distance_squared >= 0.0
    }
}

fn object_intersects(ray: Ray, object: &Sphere) -> Option<Intersection> {
    let diff = object.center - ray.position;
    let v = diff.dot(ray.direction);
    if v < 0.0 {
        None
    } else {
        let distance_squared = object.radius.powi(2) - (diff.dot(diff) - v.powi(2));
        if distance_squared < 0.0 {
            None
        } else {
            let distance = v - distance_squared.sqrt();
            Some(Intersection {
                ray,
                distance,
                normal: normal(object, ray.position + (ray.direction * distance)), //object: object,
            })
            // Normal = Vector3.Normalize(ray.Position + (ray.Direction * distance) - position); Object = s })
        }
    }
}

fn any_intersection(ray: Ray, objects: &[Sphere]) -> bool {
    objects.iter().any(|object| dirty_intersects(ray, object))
}

fn nearest_intersection(ray: Ray, objects: &[Sphere]) -> Option<Intersection> {
    objects.iter().filter_map(|object| { object_intersects(ray, object) }).min()
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
        let lcolor = if illum > 0.0 {
            light.color * illum
        } else {
            Vec3A::zero()
        };
        let diffuse_color = lcolor * base_color;
        let dot = normal.dot(ray_direction);
        let ray_direction = (ray_direction - (normal * (2.0 * dot))).normalize();
        let specular = light_dir.dot(ray_direction);
        let specular_result = if specular > 0.0 {
            light.color * (specular.powi(50))
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
    lights.iter().fold(Vec3A::zero(), |color, light| {
        color + apply_light(position, normal, objects, &light, ray_direction, base_color)
    })
}

fn trace(ray: Ray, objects: &[Sphere], lights: &[Light], depth: i32) -> Vec3A {
    let intersection = nearest_intersection(ray, objects);
    match intersection {
        Some(intersection) => {
            let hit_point =
                intersection.ray.position + (intersection.ray.direction * intersection.distance);

            let normal = intersection.normal;
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
        for x in min_x..max_x {
            let ray = camera.get_ray(x as f32, y as f32);

            result.push((x, y, trace(ray, objects, lights, 0)));
        }
    }

    result
}

fn main() {
    let width = 1024;
    let height = 768;
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
        Sphere {
            center: Vec3A::new(0.0, 2.0, -5.0),
            radius: 1.0,
        },
        Sphere {
            center: Vec3A::new(2.0, 0.0, -5.0),
            radius: 1.0,
        },
        Sphere {
            center: Vec3A::new(0.0, -1003.0, 0.0),
            radius: 1000.0,
        },
    ];

    let lights = vec![
        Light {
            position: Vec3A::new(-3.0, 3.0, -1.0),
            color: Vec3A::new(0.5, 0.0, 0.0),
        },
        Light {
            position: Vec3A::new(3.0, 3.0, -1.0),
            color: Vec3A::new(0.5, 0.5, 0.5),
        },
    ];

    canvas.render(move |mouse, image| {
        let look_x = (half_width - mouse.x as f32) / 200f32;
        let look_y = (half_height - mouse.y as f32) / 200f32;
        let look_at = Vec3A::new(look_x, look_y, -1f32);

        let camera = create_camera(position, look_at, inverse_height, half_width, half_height);
        let work: Vec<(usize, usize, usize, usize)> = vec![
            (0, 0, 1024, 191),
            (0, 191, 1024, 384),
            (0, 384, 1024, 573),
            (0, 573, 1024, 768),
        ];
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
