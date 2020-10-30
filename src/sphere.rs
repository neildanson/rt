use crate::{Intersection, Ray, Shape};
use glam::Vec3A;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub radius_squared: f32,
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
            radius_squared: radius.powi(2),
        }
    }
}

impl Shape for Sphere {
    fn normal(&self, position: Vec3A) -> Vec3A {
        (position - self.center).normalize()
    }

    fn intersects(&self, ray: Ray) -> Option<Intersection> {
        let diff = self.center - ray.position;
        let v = diff.dot(ray.direction);
        if v < 0.0 {
            None
        } else {
            let distance_squared = self.radius_squared - (diff.dot(diff) - v.powi(2));
            if distance_squared < 0.0 {
                None
            } else {
                Some(Intersection::new(ray, distance_squared, v, *self))
            }
        }
    }
}