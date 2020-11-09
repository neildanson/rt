use super::{Intersection, Ray, Shape};
use glam::Vec3A;
use bvh::nalgebra::Point3;
use bvh::bounding_hierarchy::BHShape;
use bvh::aabb::{AABB, Bounded};

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3A,
    radius : f32,
    radius_squared: f32,
    mins: Vec3A,
    maxs: Vec3A,
    index : usize
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32) -> Self {
        let radius_vec = Vec3A::splat(radius);

        Sphere {
            center,
            radius,
            radius_squared: radius.powi(2),
            mins: center - radius_vec,
            maxs: center + radius_vec,
            index : 0
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

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let (minx,miny,minz) = self.mins.into();
        let (maxx,maxy,maxz) = self.maxs.into();
        AABB::with_bounds(Point3::new(minx,miny,minz), Point3::new(maxx,maxy,maxz))
    }
}

impl BHShape for Sphere {
    fn set_bh_node_index(&mut self, index: usize) {
        self.index = index;
    }


    fn bh_node_index(&self) -> usize {
        self.index
    }
}
