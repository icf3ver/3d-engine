use ggez::{self, nalgebra::geometry::Point2};
use rand::prelude::*;
use ggez::graphics::Vertex;
use std::ops::Add;
use crate::vec3d::Vec3d;

/// A Triangle is a triangle with a normal, 3 vertecies, a
/// color, a center point and its distance to the camera.
#[derive(Copy, Clone)]
pub struct Triangle {
    pub normal: Vec3d,
    pub verticies: (Vec3d, Vec3d, Vec3d),
    pub color: (f32, f32, f32),
    pub center: Vec3d,
    pub dist: f32,
}

impl Triangle {
    /// Create a new Triangle from 3 vertecies.
    ///
    /// # Arguments
    /// * `vertex1` - The first vertex of the Triangle.
    /// * `vertex2` - The second vertex of the Triangle.
    /// * `vertex3` - The third vertex of the Triangle.
    ///
    /// # Return
    /// A new Triangle
    ///
    pub fn new(vertex1: Vec3d, vertex2: Vec3d, vertex3: Vec3d) -> Triangle {
        let normal = Triangle::calculate_normal((vertex1, vertex2, vertex3));
        let center = Triangle::calculate_center((vertex1, vertex2, vertex3));

        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        Triangle {
            normal: normal,
            verticies: (vertex1, vertex2, vertex3),
            color: (r, g, b),
            center: center,
            dist: 0.0,
        }
    }

    /// Create a new Triangle from 3 vertecies and a predefined normal.
    ///
    /// # Arguments
    /// * `normal` - The surface normal of the Triangle.
    /// * `vertex1` - The first vertex of the Triangle.
    /// * `vertex2` - The second vertex of the Triangle.
    /// * `vertex3` - The third vertex of the Triangle.
    ///
    /// # Return
    /// A new Triangle
    ///
    pub fn new_with_normal(
        normal: Vec3d,
        vertex1: Vec3d,
        vertex2: Vec3d,
        vertex3: Vec3d,
    ) -> Triangle {
        let center = Triangle::calculate_center((vertex1, vertex2, vertex3));

        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        Triangle {
            normal: normal,
            verticies: (vertex1, vertex2, vertex3),
            color: (r, g, b),
            center: center,
            dist: 0.0,
        }
    }

    /// Calculates the surface normal.
    ///
    /// # Arguments
    /// * `verticies` - A tuple containing the 3 verticies of a Triangle.
    ///
    /// # Return
    /// The surface normal
    ///
    pub fn calculate_normal(verticies: (Vec3d, Vec3d, Vec3d)) -> Vec3d {
        let u: Vec3d = verticies.1 - verticies.0;
        let v: Vec3d = verticies.2 - verticies.0;

        let n_x: f32 = (u.y * v.z) - (u.z * v.y);
        let n_y: f32 = (u.z * v.x) - (u.x * v.z);
        let n_z: f32 = (u.x * v.y) - (u.y * v.x);

        let look_len = f32::sqrt(n_x * n_x + n_y * n_y + n_z * n_z);

        Vec3d::new(n_x / look_len, n_y / look_len, n_z / look_len)
    }

    /// Calculates the center of the Triangle.
    ///
    /// # Arguments
    /// * `verticies` - A tuple containing the 3 verticies of a Triangle.
    ///
    /// # Return
    /// The center
    ///
    pub fn calculate_center(verticies: (Vec3d, Vec3d, Vec3d)) -> Vec3d {
        let center: Vec3d = Vec3d::new(
            (verticies.0.x + verticies.1.x + verticies.2.x) / 3.0,
            (verticies.0.y + verticies.1.y + verticies.2.y) / 3.0,
            (verticies.0.z + verticies.1.z + verticies.2.z) / 3.0,
        );
        center
    }

    /// Rotates the Triangle around the x-axis at the origin point.
    ///
    /// # Arguments
    /// * `self` - The Triangle the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_y` - The y position of the origin point.
    /// * `origin_z` - The z position of the origin point.
    ///
    pub fn x_axis_rotation(&mut self, r: f32, origin_y: f32, origin_z: f32) {
        self.verticies.0.x_axis_rotation(r, origin_y, origin_z);
        self.verticies.1.x_axis_rotation(r, origin_y, origin_z);
        self.verticies.2.x_axis_rotation(r, origin_y, origin_z);
        self.normal = Triangle::calculate_normal(self.verticies);
        self.center = Triangle::calculate_center(self.verticies);
    }

    /// Rotates the Triangle around the y-axis at the origin point.
    ///
    /// # Arguments
    /// * `self` - The Triangle the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_x` - The x position of the origin point.
    /// * `origin_z` - The z position of the origin point.
    ///
    pub fn y_axis_rotation(&mut self, r: f32, origin_x: f32, origin_z: f32) {
        self.verticies.0.y_axis_rotation(r, origin_x, origin_z);
        self.verticies.1.y_axis_rotation(r, origin_x, origin_z);
        self.verticies.2.y_axis_rotation(r, origin_x, origin_z);
        self.normal = Triangle::calculate_normal(self.verticies);
        self.center = Triangle::calculate_center(self.verticies);
    }

    /// Rotates the Triangle around the z-axis at the origin point.
    ///
    /// # Arguments
    /// * `self` - The Triangle the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_x` - The x position of the origin point.
    /// * `origin_y` - The y position of the origin point.
    ///
    pub fn z_axis_rotation(&mut self, r: f32, origin_x: f32, origin_y: f32) {
        self.verticies.0.z_axis_rotation(r, origin_x, origin_y);
        self.verticies.1.z_axis_rotation(r, origin_x, origin_y);
        self.verticies.2.z_axis_rotation(r, origin_x, origin_y);
        self.normal = Triangle::calculate_normal(self.verticies);
        self.center = Triangle::calculate_center(self.verticies);
    }

    /// Increment a Triangle's x position by a number.
    ///
    /// # Arguments
    /// * `self` - The Triangle this function was called for.
    /// * `inc_x` - The number the x position will be incremented by.
    ///
    pub fn increment_x(&mut self, inc_x: f32) {
        self.verticies.0.x += inc_x;
        self.verticies.1.x += inc_x;
        self.verticies.2.x += inc_x;
        self.center = Triangle::calculate_center(self.verticies);
    }

    /// Increment a Triangle's y position by a number.
    ///
    /// # Arguments
    /// * `self` - The Triangle this function was called for.
    /// * `inc_y` - The number the y position will be incremented by.
    ///
    pub fn increment_y(&mut self, inc_y: f32) {
        self.verticies.0.y += inc_y;
        self.verticies.1.y += inc_y;
        self.verticies.2.y += inc_y;
        self.center = Triangle::calculate_center(self.verticies);
    }

    /// Increment a Triangle's z position by a number.
    ///
    /// # Arguments
    /// * `self` - The Triangle this function was called for.
    /// * `inc_z` - The number the z position will be incremented by.
    ///
    pub fn increment_z(&mut self, inc_z: f32) {
        self.verticies.0.z += inc_z;
        self.verticies.1.z += inc_z;
        self.verticies.2.z += inc_z;
        self.center = Triangle::calculate_center(self.verticies);
    }

    /// Clip the Triangle's so that no part of them is being rendered off the screen.
    ///
    /// # Arguments
    /// * `x_bound` - The width of the screen
    /// * `y_bound` - The height of the screen
    ///
    /// # Return
    /// A vec of Triangle's that are on the screen and can replace the original
    ///
    pub fn clip(&mut self, x_bound: f32, y_bound: f32, mut depth: i32) -> Vec<Triangle> {
        depth = depth + 1;
        // if depth > 100 {
        //     println!("{}", depth);
        // }
        if ((self.verticies.0.x > x_bound)
            && (self.verticies.1.x > x_bound)
            && (self.verticies.2.x > x_bound))
            || ((self.verticies.0.y > y_bound)
                && (self.verticies.1.y > y_bound)
                && (self.verticies.2.y > y_bound))
            || ((self.verticies.0.x < 0.0)
                && (self.verticies.1.x < 0.0)
                && (self.verticies.2.x < 0.0))
            || ((self.verticies.0.y < 0.0)
                && (self.verticies.1.y < 0.0)
                && (self.verticies.2.y < 0.0))
        {
            let na: Vec<Triangle> = Vec::new();
            return na; //return
        }

        // X
        if self.verticies.0.x > x_bound {
            if self.verticies.1.x > x_bound {
                if !(self.verticies.2.x > x_bound) {
                    let mut b_c_pt = self.verticies.1 - self.verticies.2;
                    let f = (x_bound - self.verticies.2.x) / b_c_pt.x;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.2;

                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = (x_bound - self.verticies.2.x) / c_a_pt.x;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth); // return
                }
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = (x_bound - self.verticies.1.x) / a_b_pt.x;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if self.verticies.2.x > x_bound {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = (x_bound - self.verticies.1.x) / b_c_pt.x;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt)
                        .clip(x_bound, y_bound, depth); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = (x_bound - self.verticies.2.x) / c_a_pt.x;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2)
                        .clip(x_bound, y_bound, depth);
                    c.append(
                        &mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt)
                            .clip(x_bound, y_bound, depth),
                    );

                    return c;
                }
            }
        }

        if self.verticies.1.x > x_bound {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = (x_bound - self.verticies.0.x) / a_b_pt.x;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if self.verticies.2.x > x_bound {
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = (x_bound - self.verticies.0.x) / c_a_pt.x;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt)
                    .clip(x_bound, y_bound, depth);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = (x_bound - self.verticies.2.x) / b_c_pt.x;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2)
                    .clip(x_bound, y_bound, depth);
                c.append(
                    &mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth),
                );

                return c;
            }
        }

        //// \/\/\/
        if depth > 100 {
            println!("{}", depth);
        }

        if self.verticies.2.x > x_bound {
            let mut c_a_pt = self.verticies.2 - self.verticies.0;
            let f = (x_bound - self.verticies.0.x) / c_a_pt.x;
            c_a_pt.x *= f;
            c_a_pt.y *= f;
            c_a_pt.z *= f;
            c_a_pt = c_a_pt + self.verticies.0;

            let mut b_c_pt = self.verticies.2 - self.verticies.1;
            let f = (x_bound - self.verticies.1.x) / b_c_pt.x;
            b_c_pt.x *= f;
            b_c_pt.y *= f;
            b_c_pt.z *= f;
            b_c_pt = b_c_pt + self.verticies.1;

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt)
                .clip(x_bound, y_bound, depth);
            c.append(
                &mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound, depth),
            );

            return c;
        }

        // /\/\/\
        if depth > 100 {
            println!("NA");
        }

        // < 0
        if self.verticies.0.x < 0.0 {
            if self.verticies.1.x < 0.0 {
                if !(self.verticies.2.x < 0.0) {
                    let mut b_c_pt = self.verticies.1 - self.verticies.2;
                    let f = -self.verticies.2.x / b_c_pt.x;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.2;

                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = -self.verticies.2.x / c_a_pt.x;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth); // return
                }
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = -self.verticies.1.x / a_b_pt.x;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if self.verticies.2.x < 0.0 {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = -self.verticies.1.x / b_c_pt.x;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt)
                        .clip(x_bound, y_bound, depth); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = -self.verticies.2.x / c_a_pt.x;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2)
                        .clip(x_bound, y_bound, depth);
                    c.append(
                        &mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt)
                            .clip(x_bound, y_bound, depth),
                    );

                    return c;
                }
            }
        }

        if self.verticies.1.x < 0.0 {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = -self.verticies.0.x / a_b_pt.x;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if self.verticies.2.x < 0.0 {
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = -self.verticies.0.x / c_a_pt.x;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt)
                    .clip(x_bound, y_bound, depth);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = -self.verticies.2.x / b_c_pt.x;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2)
                    .clip(x_bound, y_bound, depth);
                c.append(
                    &mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth),
                );

                return c;
            }
        }

        if self.verticies.2.x < 0.0 {
            let mut c_a_pt = self.verticies.2 - self.verticies.0;
            let f = -self.verticies.0.x / c_a_pt.x;
            c_a_pt.x *= f;
            c_a_pt.y *= f;
            c_a_pt.z *= f;
            c_a_pt = c_a_pt + self.verticies.0;

            let mut b_c_pt = self.verticies.2 - self.verticies.1;
            let f = -self.verticies.1.x / b_c_pt.x;
            b_c_pt.x *= f;
            b_c_pt.y *= f;
            b_c_pt.z *= f;
            b_c_pt = b_c_pt + self.verticies.1;

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt)
                .clip(x_bound, y_bound, depth);
            c.append(
                &mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound, depth),
            );

            return c;
        }

        // --------------------------------------------------------------------------------------------------------
        // Y ------------------------------------------------------------------------------------------------------
        if self.verticies.0.y > y_bound {
            if self.verticies.1.y > y_bound {
                if !(self.verticies.2.y > y_bound) {
                    let mut b_c_pt = self.verticies.1 - self.verticies.2;
                    let f = (y_bound - self.verticies.2.y) / b_c_pt.y;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.2;

                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = (y_bound - self.verticies.2.y) / c_a_pt.y;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth);
                } // else \/\/\/// return
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = (y_bound - self.verticies.1.y) / a_b_pt.y;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if self.verticies.2.y > y_bound {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = (y_bound - self.verticies.1.y) / b_c_pt.y;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt)
                        .clip(x_bound, y_bound, depth); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = (y_bound - self.verticies.2.y) / c_a_pt.y;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2)
                        .clip(x_bound, y_bound, depth);
                    c.append(
                        &mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt)
                            .clip(x_bound, y_bound, depth),
                    );

                    return c;
                }
            }
        }

        if self.verticies.1.y > y_bound {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = (y_bound - self.verticies.0.y) / a_b_pt.y;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if self.verticies.2.y > y_bound {
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = (y_bound - self.verticies.0.y) / c_a_pt.y;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt)
                    .clip(x_bound, y_bound, depth);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = (y_bound - self.verticies.2.y) / b_c_pt.y;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2)
                    .clip(x_bound, y_bound, depth);
                c.append(
                    &mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth),
                );

                return c;
            }
        }

        if self.verticies.2.y > y_bound {
            let mut c_a_pt = self.verticies.2 - self.verticies.0;
            let f = (y_bound - self.verticies.0.y) / c_a_pt.y;
            c_a_pt.x *= f;
            c_a_pt.y *= f;
            c_a_pt.z *= f;
            c_a_pt = c_a_pt + self.verticies.0;

            let mut b_c_pt = self.verticies.2 - self.verticies.1;
            let f = (y_bound - self.verticies.1.y) / b_c_pt.y;
            b_c_pt.x *= f;
            b_c_pt.y *= f;
            b_c_pt.z *= f;
            b_c_pt = b_c_pt + self.verticies.1;

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt)
                .clip(x_bound, y_bound, depth);
            c.append(
                &mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound, depth),
            );

            return c;
        }

        // < 0
        if self.verticies.0.y < 0.0 {
            if self.verticies.1.y < 0.0 {
                if !(self.verticies.2.y < 0.0) {
                    let mut b_c_pt = self.verticies.1 - self.verticies.2;
                    let f = -self.verticies.2.y / b_c_pt.y;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.2;

                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = -self.verticies.2.y / c_a_pt.y;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth);
                }
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = -self.verticies.1.y / a_b_pt.y;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if self.verticies.2.y < 0.0 {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = -self.verticies.1.y / b_c_pt.y;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt)
                        .clip(x_bound, y_bound, depth); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = -self.verticies.2.y / c_a_pt.y;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2)
                        .clip(x_bound, y_bound, depth);
                    c.append(
                        &mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt)
                            .clip(x_bound, y_bound, depth),
                    );

                    return c;
                }
            }
        }

        if self.verticies.1.y < 0.0 {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = -self.verticies.0.y / a_b_pt.y;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if self.verticies.2.y < 0.0 {
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = -self.verticies.0.y / c_a_pt.y;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt)
                    .clip(x_bound, y_bound, depth);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = -self.verticies.2.y / b_c_pt.y;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2)
                    .clip(x_bound, y_bound, depth);
                c.append(
                    &mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2)
                        .clip(x_bound, y_bound, depth),
                );

                return c;
            }
        }

        if self.verticies.2.y < 0.0 {
            let mut c_a_pt = self.verticies.2 - self.verticies.0;
            let f = -self.verticies.0.y / c_a_pt.y;
            c_a_pt.x *= f;
            c_a_pt.y *= f;
            c_a_pt.z *= f;
            c_a_pt = c_a_pt + self.verticies.0;

            let mut b_c_pt = self.verticies.2 - self.verticies.1;
            let f = -self.verticies.1.y / b_c_pt.y;
            b_c_pt.x *= f;
            b_c_pt.y *= f;
            b_c_pt.z *= f;
            b_c_pt = b_c_pt + self.verticies.1;

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt)
                .clip(x_bound, y_bound, depth);
            c.append(
                &mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound, depth),
            );

            return c;
        }

        vec![*self]
    }

    /// sort an array of Triangle's according to depth
    ///
    /// # Arguments
    /// * `v` - The vec of projected Triangle's
    ///
    pub fn painters_algorithm(projected_triangles: &Vec<Triangle>) -> Vec<Triangle> {
        let mut out: Vec<Triangle> = projected_triangles.clone();
        out.sort_by(|b, a| a.dist.partial_cmp(&b.dist).unwrap());
        out
    }

    /// convert a projected Triangle to an array of 3 ggez Point2<f32>'s
    ///
    /// # Arguments
    /// * `self` - The Triangle the function was called for.
    ///
    /// # Return
    /// An array of 3 Point2<f32>'s representing the projected 3 vertecies
    /// of the Triangle.
    ///
    pub fn form_pointlist(&mut self) -> [Point2<f32>; 3] {
        let list: [Point2<f32>; 3] = [
            //change
            self.verticies.0.form_point2(),
            self.verticies.1.form_point2(),
            self.verticies.2.form_point2(),
        ];
        list
    }

    pub fn form_vertexlist(&mut self) -> [Vertex; 3] {
        let list: [Vertex; 3] = [
            self.verticies.0.form_vertex(),
            self.verticies.1.form_vertex(),
            self.verticies.2.form_vertex(),
        ];
        list
    }
}

impl Add<Vec3d> for Triangle {
    type Output = Triangle;

    fn add(self, other: Vec3d) -> Self {
        let mut new_tri = self.clone();

        new_tri.verticies.0 += other;
        new_tri.verticies.1 += other;
        new_tri.verticies.2 += other;
        new_tri.center = Triangle::calculate_center(new_tri.verticies);

        new_tri
    }
}
