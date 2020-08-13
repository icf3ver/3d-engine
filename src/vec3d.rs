use ggez;              // Graphics
use ggez::nalgebra as na;
use ggez::nalgebra::geometry::Point2;

use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d{
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d{
        Vec3d{x: x, y: y, z: z}
    }

    pub fn x_axis_rotation(&mut self, r: f32, origin_y: f32, origin_z: f32) {
        let tmp_y = self.y;
        let tmp_z = self.z;
        let angle: f32 = (r) * (3.14159265/180.0) ;
        self.y = (tmp_y - origin_y) * f32::cos(angle) - (tmp_z - origin_z) * f32::sin(angle) + origin_y;
        self.z = (tmp_y - origin_y) * f32::sin(angle) + (tmp_z - origin_z) * f32::cos(angle) + origin_z;
    }

    pub fn y_axis_rotation(&mut self, r: f32, origin_x: f32, origin_z: f32) {
        let tmp_x = self.x;
        let tmp_z = self.z;
        let angle: f32 = (r) * (3.14159265/180.0);
        self.x = (tmp_x - origin_x) * f32::cos(angle) - (tmp_z - origin_z) * f32::sin(angle) + origin_x;
        self.z = (tmp_x - origin_x) * f32::sin(angle) + (tmp_z - origin_z) * f32::cos(angle) + origin_z;
    }

    pub fn z_axis_rotation (&mut self, r: f32, origin_x: f32, origin_y: f32) {
        let tmp_x = self.x;
        let tmp_y = self.y;
        let angle: f32 = (r) * (3.14159265/180.0);
        self.x = (tmp_x - origin_x) * f32::cos(angle) - (tmp_y - origin_y) * f32::sin(angle) + origin_x;
        self.y = (tmp_x - origin_x) * f32::sin(angle) + (tmp_y - origin_y) * f32::cos(angle) + origin_y;
    }

    pub fn form_point2(&mut self) -> Point2<f32> {
        na::Point2::new(self.x, self.y)
    }
}

impl Sub for Vec3d{
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}