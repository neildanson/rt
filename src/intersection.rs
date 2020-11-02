use glam::Vec3A;
use std::cmp::Ordering;

use crate::{Ray, Shape, Sphere};

#[derive(Copy, Clone)]
pub struct Intersection {
    pub ray: Ray,
    distance: f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(ray: Ray, distance: f32, object: Sphere) -> Self {
        Intersection {
            ray,
            distance,
            object,
        }
    }

    pub fn normal(&self, hit_point: Vec3A) -> Vec3A {
        self.object.normal(hit_point)
    }

    pub fn hit_point(&self) -> Vec3A {
        self.ray.position + (self.ray.direction * self.distance)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
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
