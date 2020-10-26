use glam::Vec3A;

#[derive(Copy, Clone)]
pub struct Ray {
    pub position: Vec3A,
    pub direction: Vec3A,
}