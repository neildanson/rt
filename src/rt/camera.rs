use super::Ray;
use glam::Vec3A;

#[inline]
fn recenter_x(x: f32, half_width: f32) -> f32 {
    x - half_width
}

#[inline]
fn recenter_y(y: f32, half_height: f32) -> f32 {
    y - half_height
}

pub struct Camera {
    position: Vec3A,
    forward: Vec3A,
    right: Vec3A,
    up: Vec3A,

    half_width: f32,
    half_height: f32,
}

impl Camera {
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let right = self.right * recenter_x(x, self.half_width);
        let up = self.up * recenter_y(y, self.half_height);
        Ray {
            position: self.position,
            direction: (right + up + self.forward).normalize(),
        }
    }

    pub fn create_camera(
        position: Vec3A,
        look_at: Vec3A,
        inverse_height: f32,
        half_width: f32,
        half_height: f32,
    ) -> Self {
        let forward = (look_at - position).normalize();
        let down = -Vec3A::Y; //this is actually inverted due to pixel-canvas coord system starting y=0 at the bottom of the window instead of top
        let right = forward.cross(down).normalize() * 1.5f32 * inverse_height;
        let up = forward.cross(right).normalize() * 1.5f32 * inverse_height;

        Camera {
            position,
            forward,
            right,
            up,
            half_width,
            half_height,
        }
    }
}
