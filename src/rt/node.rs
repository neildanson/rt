use super::{Bounds, Intersection, Ray, Shape, Sphere, AABB};
use glam::Vec3A;
use std::f32;

pub struct Node {
    aabb: AABB,
    objects: Vec<Sphere>,
}

impl Node {
    //fn any_intersection(&self, ray: Ray) -> bool {
    //    self.objects
    //        .iter()
    //        .any(|object| object.intersects(ray).is_some())
    //}

    fn nearest_intersection(&self, ray: Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|object| object.intersects(ray))
            .min()
    }

    pub fn new(objects: Vec<Sphere>) -> Self {
        let mut mins = Vec3A::splat(f32::MAX);
        let mut maxs = Vec3A::splat(f32::MIN);

        for object in objects.iter() {
            mins = mins.min(object.mins());
            maxs = maxs.max(object.maxs());
        }

        let aabb = AABB::new(mins, maxs);

        Node { aabb, objects }
    }

    pub fn intersects(&self, ray: Ray) -> Option<Intersection> {
        if self.aabb.intersects_bounds(ray) {
            self.nearest_intersection(ray)
        } else {
            None
        }
    }
}
