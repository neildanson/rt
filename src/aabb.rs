
use crate::Bounds;
use glam::Vec3A;

#[derive(Copy, Clone)]
pub struct AABB {
    mins: Vec3A,
    maxs: Vec3A,
}



impl AABB {
    pub fn new(mins: Vec3A, maxs: Vec3A) -> Self {
        AABB { mins, maxs }
    }
}

impl Bounds for AABB {
    fn mins(&self) -> Vec3A {
        self.mins
    }

    fn maxs(&self) -> Vec3A {
        self.maxs
    }
}