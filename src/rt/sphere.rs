use super::{Bounds, Intersection, Ray, Shape};
use glam::Vec3A;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3A,
    radius_squared: f32,
    mins: Vec3A,
    maxs: Vec3A,
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32) -> Self {
        let radius_vec = Vec3A::splat(radius);

        Sphere {
            center,
            radius_squared: radius.powi(2),
            mins: center - radius_vec,
            maxs: center + radius_vec,
        }
    }
}

impl Shape for Sphere {
    fn normal(&self, position: Vec3A) -> Vec3A {
        (position - self.center).normalize()
    }

    fn intersects(&self, ray: Ray) -> Option<Intersection> {
        //if  !self.intersects_bounds(ray) {
        //    return None;
        //}

        let diff = self.center - ray.position;
        let v = diff.dot(ray.direction);
        if v < 0.0 {
            None
        } else {
            let discriminant = self.radius_squared - (diff.dot(diff) - v.powi(2));
            if discriminant < 0.0 {
                None
            } else {
                Some(Intersection::new(ray, v - discriminant.sqrt(), *self))
            }
        }
    }
}

impl Bounds for Sphere {
    fn mins(&self) -> Vec3A {
        self.mins
    }

    fn maxs(&self) -> Vec3A {
        self.maxs
    }
}
