use crate::vec3d::Vec3d;

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3d,
    pub rotation: Vec3d,
}