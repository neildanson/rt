use pixel_canvas::{Canvas, Color, input::MouseState};
use glam::Vec3;

#[derive(Copy, Clone)]
struct Ray { 
    position : Vec3,
    direction : Vec3
}

struct Intersection { 
    ray : Ray,
    distance : f32,
    normal : Vec3
}

struct Sphere {
    center : Vec3,
    radius : f32
}

struct Light {
    position: Vec3,
    color: Vec3,
}

struct Camera {
    forward: Vec3,
    right: Vec3,
    up: Vec3,
}

fn normal(sphere : &Sphere, position : Vec3) -> Vec3 {
    (position - sphere.center).normalize()
}

fn create_camera(position: Vec3, look_at: Vec3, inverse_height: f32) -> Camera {
    let forward = (look_at - position).normalize();
    let down = Vec3::unit_y();
    let right = forward.cross(down).normalize() * 1.5f32 * inverse_height;
    let up = forward.cross(right).normalize() * 1.5f32 * inverse_height;

    Camera {
        forward,
        right,
        up,
    }
}

fn recenter_x(x: f32, half_width: f32) -> f32 {
    x - half_width
}

fn recenter_y(y: f32, half_height: f32) -> f32 {
    -(y - half_height)
}

fn get_ray(position: Vec3,
               x: f32,
               y: f32,
               half_width: f32,
               half_height: f32,
               camera: Camera)
               -> Ray {
    let right = camera.right * recenter_x(x, half_width);
    let up = camera.up * recenter_y(y, half_height);
    Ray {
        position,
        direction: (right + up + camera.forward).normalize(),
    }
}

fn to_color(vec : Vec3) -> Color {
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
                normal : normal(object, ray.position + (ray.direction * distance))
                //object: object,
            })
            // Normal = Vector3.Normalize(ray.Position + (ray.Direction * distance) - position); Object = s })
        }
    }

}

fn any_intersection(ray: Ray, objects: &[Sphere]) -> bool {
    objects.iter().any(|object| object_intersects(ray, object).is_some())
}

fn nearest_intersection(ray: Ray, objects: &[Sphere]) -> Option<Intersection> {
    objects.iter().fold(None, |intersection, object| {
        let i = object_intersects(ray, object);
        match (intersection, i) {
            (Some(intersection), None) => Some(intersection),
            (None, Some(i)) => Some(i),
            (Some(a), Some(b)) => {
                if a.distance < b.distance {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            (_, _) => None,

        }
    })
}

fn apply_light(position: Vec3,
               normal: Vec3,
               objects: &[Sphere],
               light: &Light,
               ray_direction: Vec3,
               base_color: Vec3)
               -> Vec3 {

    let light_dir = (light.position - position).normalize();
    let ray = Ray {
        position,
        direction: light_dir,
    };
    let is_in_shadow = any_intersection(ray, objects);
    if is_in_shadow {
        Vec3::zero()
    } else {
        let illum = light_dir.dot(normal);
        let lcolor = if illum > 0.0 {
            light.color * illum
        } else {
            Vec3::zero()
        };
        let diffuse_color = lcolor * base_color;
        let dot = normal.dot(ray_direction);
        let ray_direction = (ray_direction - (normal * (2.0 * dot))).normalize();
        let specular = light_dir.dot(ray_direction);
        let specular_result = if specular > 0.0 {
            light.color * (specular.powi(50))
        } else {
            Vec3::zero()
        };
        diffuse_color + specular_result
    }
}


fn apply_lighting(position: Vec3,
                  normal: Vec3,
                  objects: &[Sphere],
                  lights: &[Light],
                  ray_direction: Vec3,
                  base_color: Vec3)
                  -> Vec3 {
    lights.iter().fold(Vec3::zero(), |color, light| {
        color + apply_light(position, normal, objects, &light, ray_direction, base_color)
    })
}


fn trace(ray: Ray, objects: &[Sphere], lights: &[Light], _depth: i32) -> Vec3 {
    let intersection = nearest_intersection(ray, objects);
    match intersection {
        Some(intersection) => {
            let hit_point = intersection.ray.position +
                            (intersection.ray.direction * intersection.distance);

            let normal = intersection.normal;
            let color = Vec3::new(0.5,0.5,0.5);
            let color = apply_lighting(hit_point,
                                       normal, // intersection.normal,
                                       objects,
                                       lights,
                                       intersection.ray.direction,
                                       color);
            //if depth < 3 {
            //   let ray = Ray {
            //        position: hit_point,
            //        direction: normal,
            //    };
            //    let newcolor = trace(ray, objects, lights, depth + 1);
            //    color + newcolor
            //} else {
                color
            //}
        }
        None => Vec3::zero(),
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let inverse_height = 1.0f32 / height as f32;
    let half_height = height as f32 / 2.0f32;
    let half_width = width as f32 / 2.0f32;
    let position = Vec3::zero();

    let canvas = Canvas::new(width, height)
        .title("Raytrace")
        .state(MouseState::new())
        .show_ms(true)
        .input(MouseState::handle_input);


    let spheres = vec!(
            Sphere { center : Vec3::new(0.0, 2.0, -5.0), radius : 1.0 },            
            Sphere { center : Vec3::new(2.0, 0.0, -5.0), radius : 1.0 },
            Sphere { center : Vec3::new(0.0, -1003.0, 0.0), radius : 1000.0 }
    );
    
   let lights = vec! (
       Light { position: Vec3::new(-3.0, 3.0,-1.0), color: Vec3::new(0.5, 0.0, 0.0) },
       Light { position: Vec3::new(3.0, 3.0,-1.0), color: Vec3::new(0.5, 0.5, 0.5) },
    );

    canvas.render(move |mouse, image| {
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let look_x = (half_width - mouse.x as f32) / 1000f32;
                let look_y = (half_height - mouse.y as f32 ) / 1000f32;

                let look_at = Vec3::new(look_x, look_y, -1f32);

                let camera = create_camera(position, look_at, inverse_height);

                let ray = get_ray(position,
                    x as f32,
                    y as f32,
                    half_width,
                    half_height,
                    camera);

                let color = trace(ray, &spheres, &lights, 0);

                *pixel = to_color(color);
            }
        }
    });
}