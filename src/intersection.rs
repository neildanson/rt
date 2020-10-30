use std::cmp::Ordering;
use glam::Vec3A;

use crate::{Ray, Shape, Sphere};

#[derive(Copy, Clone)]
pub struct Intersection {
    pub ray: Ray,
    distance_squared: f32,
    v:f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(ray:Ray, distance_squared : f32, v:f32, object:Sphere) -> Intersection {
        Intersection {
            ray,
            distance_squared,
            v,
            object
        }
    }

    fn distance(&self) -> f32 {
        self.v - self.distance_squared.sqrt()
    }

    pub fn normal(&self, hit_point:Vec3A) -> Vec3A {
        self.object.normal(hit_point)
    }

    pub fn hit_point(&self) -> Vec3A {
        self.ray.position + (self.ray.direction * self.distance())

    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance_squared <= other.distance_squared {
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
        self.distance_squared == other.distance_squared
    }
}