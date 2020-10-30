use crate::{Intersection, Ray};
use glam::Vec3A;
use std::mem::swap;

#[derive(Copy, Clone)]
pub struct AABB {
    mins: Vec3A,
    maxs: Vec3A,
}

impl AABB {
    pub fn new(mins: Vec3A, maxs: Vec3A) -> Self {
        AABB { mins, maxs }
    }

    pub fn intersects(self, ray: Ray) -> bool {
        let min = (self.mins - ray.position) / ray.direction;
        let max = (self.maxs - ray.position) / ray.direction;

        let (mut minx,mut miny,mut minz) = min.into();
        let (mut maxx,mut maxy,mut maxz) = max.into();
        if minx > maxx {
            swap(&mut minx, &mut maxx);
        }

        if miny > maxy {
            swap(&mut miny, &mut maxy);
        }


        
        if minx > maxy || miny > maxx {
            return false
        }
        
        if miny > minx {
            minx = miny;
        }
        if maxy <maxx { 
            maxx = maxy;
        }

        if minz > maxz {
            swap(&mut minz, &mut maxz);
        }

        if minx > maxz || minz > maxx {
            return false
        }
        

        true
    }
}