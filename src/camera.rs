use gamemath::Mat4;

use crate::resolution;
use crate::resolution::Resolution;

pub struct Camera {
    transform: Mat4,
    fov: f32,
    focal_length: f32,
    resolution: Resolution,
}