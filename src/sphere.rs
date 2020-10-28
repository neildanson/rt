use glam::Vec3A;

pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub radius_squared: f32
}

impl Sphere { 
    pub fn new (center : Vec3A, radius : f32) -> Sphere {
        Sphere { center, radius, radius_squared : radius.powi(2) }
    }
    pub fn normal(&self, position: Vec3A) -> Vec3A {
        (position - self.center).normalize()
    }
}