use super::{Intersection, Ray};
use glam::Vec3A;

pub trait Shape {
    fn normal(&self, position: Vec3A) -> Vec3A;
    fn intersects(&self, ray: Ray) -> Option<Intersection>;
}
