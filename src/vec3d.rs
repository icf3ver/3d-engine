use ggez;
use ggez::nalgebra as na;
use ggez::nalgebra::geometry::Point2;

use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;

/// A Vec3d is a point in 3D space
#[derive(Copy, Clone)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d{
    /// Creates a Vec3d from an x, a y, and a z.
    /// 
    /// # Arguments
    /// * `x` - The x position of the Vec3d.
    /// * `y` - The y position of the Vec3d.
    /// * `z` - The z position of the Vec3d.
    /// 
    /// # Return
    /// A new Vec3d
    /// 
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d{
        Vec3d{x: x, y: y, z: z}
    }

    /// Rotates the Vec3d around the x-axis at the origin point.
    /// 
    /// # Arguments
    /// * `self` - The Vec3d the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_y` - The y position of the origin point.
    /// * `origin_z` - The z position of the origin point.
    /// 
    pub fn x_axis_rotation(&mut self, r: f32, origin_y: f32, origin_z: f32) {
        let tmp_y = self.y;
        let tmp_z = self.z;
        let angle: f32 = (r) * (3.14159265/180.0) ;
        self.y = (tmp_y - origin_y) * f32::cos(angle) - (tmp_z - origin_z) * f32::sin(angle) + origin_y;
        self.z = (tmp_y - origin_y) * f32::sin(angle) + (tmp_z - origin_z) * f32::cos(angle) + origin_z;
    }

    /// Rotates the Vec3d around the y-axis at the origin point.
    /// 
    /// # Arguments
    /// * `self` - The Vec3d the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_x` - The x position of the origin point.
    /// * `origin_z` - The z position of the origin point.
    /// 
    pub fn y_axis_rotation(&mut self, r: f32, origin_x: f32, origin_z: f32) {
        let tmp_x = self.x;
        let tmp_z = self.z;
        let angle: f32 = (r) * (3.14159265/180.0);
        self.x = (tmp_x - origin_x) * f32::cos(angle) - (tmp_z - origin_z) * f32::sin(angle) + origin_x;
        self.z = (tmp_x - origin_x) * f32::sin(angle) + (tmp_z - origin_z) * f32::cos(angle) + origin_z;
    }

    /// Rotates the Vec3d around the z-axis at the origin point.
    /// 
    /// # Arguments
    /// * `self` - The Vec3d the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_x` - The x position of the origin point.
    /// * `origin_y` - The y position of the origin point.
    /// 
    pub fn z_axis_rotation (&mut self, r: f32, origin_x: f32, origin_y: f32) {
        let tmp_x = self.x;
        let tmp_y = self.y;
        let angle: f32 = (r) * (3.14159265/180.0);
        self.x = (tmp_x - origin_x) * f32::cos(angle) - (tmp_y - origin_y) * f32::sin(angle) + origin_x;
        self.y = (tmp_x - origin_x) * f32::sin(angle) + (tmp_y - origin_y) * f32::cos(angle) + origin_y;
    }

    /// Normalize the Vec3d
    /// 
    /// # Arguments
    /// * `self` - The Vec3d the function was called for.
    /// 
    /// # Return
    /// self
    /// 
    pub fn normalize(&mut self) -> Vec3d{
        let len = f32::sqrt((self.x * self.x + self.y * self.y + self.z * self.z).into());
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
        *self
    }

    /// Set the length of the vector to a number.
    /// 
    /// # Arguments
    /// * `self` - The Vec3d the function was called for.
    /// * `len` - The desired length.
    /// 
    /// # Return
    /// self
    /// 
    pub fn set_length(&mut self, len: f32) -> Vec3d{
        self.normalize();
        self.x = self.x * len;
        self.y = self.y * len;
        self.z = self.z * len;
        *self
    }

    /// Remove the z value from the Vec3d.
    /// 
    /// # Arguments 
    /// * `self` - The Vec3d the function was called for.
    /// 
    /// # Return
    /// Point2 containing the x and y positions of the Vec3d
    /// 
    pub fn form_point2(&mut self) -> Point2<f32> {
        na::Point2::new(self.x, self.y)
    }
}

/// Vec3d - Vec3d = Vec3d
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

/// Vec3d + Vec3d = Vec3d
impl Add for Vec3d{
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

/// Vec3d * Vec3d = Vec3d
impl Mul for Vec3d{
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
        )
    }
}


/// Vec3d * f32 = Vec3d or f32 * Vec3d = Vec3d
impl Mul<f32> for Vec3d{
    type Output = Vec3d;

    fn mul(self, other: f32) -> Vec3d {
        Vec3d::new(
            self.x * other,
            self.y * other,
            self.z * other,
        )
    }
}