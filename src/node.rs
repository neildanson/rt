use crate::{Intersection, Ray, Shape, Sphere, AABB};

pub struct Node {
    aabb: AABB,
    objects: Vec<Sphere>,
}

impl Node {
    fn any_intersection(&self, ray: Ray) -> bool {
        self.objects
            .iter()
            .any(|object| object.intersects(ray).is_some())
    }

    fn nearest_intersection(&self, ray: Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|object| object.intersects(ray))
            .min()
    }

    //TODO - Should this calculate AABB rather than passed in?
    pub fn new(aabb: AABB, objects: Vec<Sphere>) -> Self {
        Node { aabb, objects }
    }

    pub fn intersects(&self, ray: Ray) -> Option<Intersection> {
        if self.aabb.intersects(ray) {
            self.nearest_intersection(ray)
        } else {
            None
        }
    }
}
