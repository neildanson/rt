use std::cmp::Ordering;
use glam::Vec3A;

use crate::Ray;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub ray: Ray,
    pub distance: f32,
    pub normal: Vec3A,
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