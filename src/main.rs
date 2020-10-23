use pixel_canvas::{Canvas, Color, input::MouseState};
use glam::Vec3;

#[derive(Copy, Clone)]
struct Ray { 
    position : Vec3,
    direction : Vec3
}

struct Intersection { 
    ray : Ray,
    distance : f32
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
    position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
}

fn create_camera(position: Vec3, look_at: Vec3, inverse_height: f32) -> Camera {
    let forward = (look_at - position).normalize();
    let down = Vec3::new(0.0, -1.0,0.0);
    let right = forward.cross(down).normalize() * 1.5f32 * inverse_height;
    let up = forward.cross(right).normalize() * 1.5f32 * inverse_height;

    Camera {
        position: position,
        forward: forward,
        right: right,
        up: up,
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
               camera: &Camera)
               -> Ray {
    let right = camera.right * recenter_x(x, half_width);
    let up = camera.up * recenter_y(y, half_height);
    Ray {
        position,
        direction: (right + up + camera.forward).normalize(),
    }
}

fn make_ray(position : Vec3, direction : Vec3) -> Ray { 
    Ray { position, direction }
}

fn prime_ray(x : usize, y : usize) -> Ray {
    let point = Vec3::new(0.0, 0.0, 0.0);
    make_ray(point, point)
}

fn to_color(v : Vec3) -> Color {
    let (x, y, z) = v.into();
    let r = x.min(1.0).max(0.0) * 255.0;
    let g = y.min(1.0).max(0.0) * 255.0;
    let b = z.min(1.0).max(0.0) * 255.0;

    Color {
        r: r as u8,
        g: g as u8,
        b: b as u8,
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
                ray: ray,
                distance: distance,
                //object: object,
            })
            // Normal = Vector3.Normalize(ray.Position + (ray.Direction * distance) - position); Object = s })
        }
    }

}

fn any_intersection(ray: Ray, objects: &Vec<Sphere>) -> bool {
    objects.iter().any(|object| object_intersects(ray, object).is_some())
}

fn nearest_intersection(ray: Ray, objects: &Vec<Sphere>) -> Option<Intersection> {
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


fn trace(ray: Ray, objects: &Vec<Sphere>, lights: &Vec<Light>, depth: i32) -> Vec3 {
    let intersection = nearest_intersection(ray, objects);
    match intersection {
        Some(intersection) => {
            let _hit_point = intersection.ray.position +
                            (intersection.ray.direction * intersection.distance);

            //let normal = intersection.object.normal(&intersection);

            let color = Vec3::new(1.0,1.0,1.0); //intersection.object intersection
            //let color = apply_lighting(hit_point,
            //                           normal, // intersection.normal,
            //                           objects,
            //                           lights,
            //                           intersection.ray.direction,
            //                           color);
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
    let width = 1280;
    let height = 720;
    let inverse_height = 1.0f32 / 720.0f32;
    let half_height = 720.0f32 / 2.0f32;
    let half_width = 720.0f32 / 2.0f32;
    let position = Vec3::zero();

    let canvas = Canvas::new(width, height)
        .title("Raytrace")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    let spheres = vec!(
        Sphere { center : Vec3::new(0.0, 1.0, 5.0), radius : 1.0 }
    );

    let lights = vec! (Light {
        position: Vec3::new(-3.0, 3.0,-1.0), color: Vec3::new(0.5, 0.0, 0.0),
        },
    );


    canvas.render(move |mouse, image| {
        //Move  
        let spheres = vec!(
            Sphere { center : Vec3::new(0.0, 1.0, 5.0), radius : 1.0 },            
            Sphere { center : Vec3::new(1.0, 0.0, 5.0), radius : 1.0 }
        );
    
        let lights = vec! (Light {
            position: Vec3::new(-3.0, 3.0,-1.0), color: Vec3::new(0.5, 0.0, 0.0),
            },
        );

        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let look_x = (half_width - mouse.x as f32) / 1000f32;
                let look_y = (half_height - mouse.y as f32 ) / 1000f32;

                let look_at = Vec3::new(look_x, -look_y, 1f32);

                let camera = create_camera(position, look_at, inverse_height);

                let ray = get_ray(position,
                    x as f32,
                    y as f32,
                    half_width,
                    half_height,
                    &camera);

                let color = trace(ray, &spheres, &lights, 0);

                *pixel = to_color(color);
            }
        }
    });
}