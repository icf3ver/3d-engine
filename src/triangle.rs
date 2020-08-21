use ggez;              // Graphics
use ggez::nalgebra::geometry::Point2;

use rand::prelude::*;
use std::cmp::Ordering;

use crate::vec3d::Vec3d;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub normal: Vec3d,
    pub verticies: (Vec3d, Vec3d, Vec3d),
    pub color: (f32, f32, f32),
    pub center: Vec3d,
}

impl Triangle{
    pub fn new(vertex1: Vec3d, vertex2: Vec3d, vertex3: Vec3d) -> Triangle {
        let normal = Triangle::calculate_normal((vertex1, vertex2, vertex3));
        let center = Triangle::calculate_center((vertex1, vertex2, vertex3));
        
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        Triangle{normal: normal, verticies: (vertex1, vertex2, vertex3), color: (r, g, b), center: center}

    }

    pub fn new_with_normal(normal: Vec3d, vertex1: Vec3d, vertex2: Vec3d, vertex3: Vec3d) -> Triangle {
        let center = Triangle::calculate_center((vertex1, vertex2, vertex3));

        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        Triangle{normal: normal, verticies: (vertex1, vertex2, vertex3), color: (r, g, b), center: center}
    }
    
    pub fn calculate_normal(verticies: (Vec3d, Vec3d, Vec3d)) -> Vec3d{
        let u: Vec3d = verticies.1 - verticies.0;
        let v: Vec3d = verticies.2 - verticies.0;

        let n_x: f32 = (u.y * v.z) - (u.z * v.y);
        let n_y: f32 = (u.z * v.x) - (u.x * v.z);
        let n_z: f32 = (u.x * v.y) - (u.y * v.x);

        let look_len = f32::sqrt(n_x * n_x + n_y * n_y + n_z * n_z);

        Vec3d::new(n_x / look_len, n_y / look_len, n_z / look_len)
    }
    
    pub fn x_axis_rotation(&mut self, r: f32, origin_y: f32, origin_z: f32) {
        self.verticies.0.x_axis_rotation(r, origin_y, origin_z);
        self.verticies.1.x_axis_rotation(r, origin_y, origin_z);
        self.verticies.2.x_axis_rotation(r, origin_y, origin_z);
        self.normal = Triangle::calculate_normal(self.verticies);
    }

    pub fn y_axis_rotation(&mut self, r: f32, origin_x: f32, origin_z: f32) {
        self.verticies.0.y_axis_rotation(r, origin_x, origin_z);
        self.verticies.1.y_axis_rotation(r, origin_x, origin_z);
        self.verticies.2.y_axis_rotation(r, origin_x, origin_z);
        self.normal = Triangle::calculate_normal(self.verticies);
    }

    pub fn z_axis_rotation(&mut self, r: f32, origin_x: f32, origin_y: f32) {
        self.verticies.0.z_axis_rotation(r, origin_x, origin_y);
        self.verticies.1.z_axis_rotation(r, origin_x, origin_y);
        self.verticies.2.z_axis_rotation(r, origin_x, origin_y);
        self.normal = Triangle::calculate_normal(self.verticies);
    }

    pub fn calculate_center(verticies: (Vec3d, Vec3d, Vec3d)) -> Vec3d{
        let center: Vec3d = Vec3d::new(
            (verticies.0.x + verticies.1.x + verticies.2.x)/3.0,
            (verticies.0.y + verticies.1.y + verticies.2.y)/3.0,
            (verticies.0.z + verticies.1.z + verticies.2.z)/3.0);
        center
    }

    pub fn increment_x(&mut self, x: f32){
        self.verticies.0.x += x;
        self.verticies.1.x += x;
        self.verticies.2.x += x;
    }

    pub fn increment_y(&mut self, y: f32){
        self.verticies.0.y += y;
        self.verticies.1.y += y;
        self.verticies.2.y += y;
    }

    pub fn increment_z(&mut self, z: f32){
        self.verticies.0.z += z;
        self.verticies.1.z += z;
        self.verticies.2.z += z;
    }

    pub fn form_pointlist(&mut self) -> [Point2<f32>; 3] {
        let list: [Point2<f32>; 3] = [ 
            self.verticies.0.form_point2(),
            self.verticies.1.form_point2(),
            self.verticies.2.form_point2(),
        ];
        list
    }

    pub fn clip (&mut self, x_bound: f32, y_bound: f32) -> Vec<Triangle>{
        if (self.verticies.0.x > x_bound) &&
            (self.verticies.1.x > x_bound) &&
            (self.verticies.2.x > x_bound) && 
            (self.verticies.0.x < 0.0) && 
            (self.verticies.1.x < 0.0) && 
            (self.verticies.2.x < 0.0) &&
            (self.verticies.0.y > y_bound) &&
            (self.verticies.1.y > y_bound) &&
            (self.verticies.2.y > y_bound) &&
            (self.verticies.0.y < 0.0) &&
            (self.verticies.1.y < 0.0) &&
            (self.verticies.2.y < 0.0) {
                let na: Vec<Triangle> = Vec::new();
                return na;      //return
            }
        

        // X
        if (self.verticies.0.x > x_bound) {
            if (self.verticies.1.x > x_bound) {
                if !(self.verticies.2.x > x_bound){
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

                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound);// return
                }
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = (x_bound - self.verticies.1.x) / a_b_pt.x;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if (self.verticies.2.x > x_bound) {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = (x_bound - self.verticies.1.x) / b_c_pt.x;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = (x_bound - self.verticies.2.x) / c_a_pt.x;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2).clip(x_bound, y_bound);
                    c.append(&mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt).clip(x_bound, y_bound));

                    return c;
                }
            }
        }

        if (self.verticies.1.x > x_bound) {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = (x_bound - self.verticies.0.x) / a_b_pt.x;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if (self.verticies.2.x > x_bound){
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = (x_bound - self.verticies.0.x) / c_a_pt.x;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt).clip(x_bound, y_bound);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = (x_bound - self.verticies.2.x) / b_c_pt.x;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2).clip(x_bound, y_bound);
                c.append(&mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound));

                return c;
            }
        }

        if (self.verticies.2.x > x_bound){
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

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt).clip(x_bound, y_bound);
            c.append(&mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound));

            return c;
        }

        // < 0
        if (self.verticies.0.x < 0.0) {
            if (self.verticies.1.x < 0.0) {
                if !(self.verticies.2.x < 0.0){
                    
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
                    
                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound);// return
                }
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = -self.verticies.1.x / a_b_pt.x;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if (self.verticies.2.x < 0.0) {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = -self.verticies.1.x / b_c_pt.x;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = -self.verticies.2.x / c_a_pt.x;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2).clip(x_bound, y_bound);
                    c.append(&mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt).clip(x_bound, y_bound));

                    return c;
                }
            }
        }

        if (self.verticies.1.x < 0.0) {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = -self.verticies.0.x / a_b_pt.x;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if (self.verticies.2.x < 0.0){
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = -self.verticies.0.x / c_a_pt.x;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt).clip(x_bound, y_bound);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = -self.verticies.2.x / b_c_pt.x;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2).clip(x_bound, y_bound);
                c.append(&mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound));

                return c;
            }
        }

        if (self.verticies.2.x < 0.0){
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

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt).clip(x_bound, y_bound);
            c.append(&mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound));

            return c;
        }
        // --------------------------------------------------------------------------------------------------------
        // Y ------------------------------------------------------------------------------------------------------
        if (self.verticies.0.y > y_bound) {
            if (self.verticies.1.y > y_bound) {
                if !(self.verticies.2.y > y_bound){
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

                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound);
                } // else \/\/\/// return
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = (y_bound - self.verticies.1.y) / a_b_pt.y;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if (self.verticies.2.y > y_bound) {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = (y_bound - self.verticies.1.y) / b_c_pt.y;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = (y_bound - self.verticies.2.y) / c_a_pt.y;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2).clip(x_bound, y_bound);
                    c.append(&mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt).clip(x_bound, y_bound));

                    return c;
                }
            }
        }

        if (self.verticies.1.y > y_bound) {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = (y_bound - self.verticies.0.y) / a_b_pt.y;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if (self.verticies.2.y > y_bound){
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = (y_bound - self.verticies.0.y) / c_a_pt.y;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt).clip(x_bound, y_bound);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = (y_bound - self.verticies.2.y) / b_c_pt.y;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2).clip(x_bound, y_bound);
                c.append(&mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound));

                return c;
            }
        }

        if (self.verticies.2.y > y_bound){
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

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt).clip(x_bound, y_bound);
            c.append(&mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound));

            return c;
        }

        // < 0
        if (self.verticies.0.y < 0.0) {
            if (self.verticies.1.y < 0.0) {
                if !(self.verticies.2.y < 0.0){

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
    
                    return Triangle::new(c_a_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound);

                }
            } else {
                let mut a_b_pt = self.verticies.0 - self.verticies.1;
                let f = -self.verticies.1.y / a_b_pt.y;
                a_b_pt.x *= f;
                a_b_pt.y *= f;
                a_b_pt.z *= f;
                a_b_pt = a_b_pt + self.verticies.1;
                if (self.verticies.2.y < 0.0) {
                    let mut b_c_pt = self.verticies.2 - self.verticies.1;
                    let f = -self.verticies.1.y / b_c_pt.y;
                    b_c_pt.x *= f;
                    b_c_pt.y *= f;
                    b_c_pt.z *= f;
                    b_c_pt = b_c_pt + self.verticies.1;

                    return Triangle::new(a_b_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound); // return
                } else {
                    let mut c_a_pt = self.verticies.0 - self.verticies.2;
                    let f = -self.verticies.2.y / c_a_pt.y;
                    c_a_pt.x *= f;
                    c_a_pt.y *= f;
                    c_a_pt.z *= f;
                    c_a_pt = c_a_pt + self.verticies.2;

                    let mut c = Triangle::new(a_b_pt, self.verticies.1, self.verticies.2).clip(x_bound, y_bound);
                    c.append(&mut Triangle::new(a_b_pt, self.verticies.2, c_a_pt).clip(x_bound, y_bound));

                    return c;
                }
            }
        }

        if (self.verticies.1.y < 0.0) {
            let mut a_b_pt = self.verticies.1 - self.verticies.0;
            let f = -self.verticies.0.y / a_b_pt.y;
            a_b_pt.x *= f;
            a_b_pt.y *= f;
            a_b_pt.z *= f;
            a_b_pt = a_b_pt + self.verticies.0;

            if (self.verticies.2.y < 0.0){
                let mut c_a_pt = self.verticies.2 - self.verticies.0;
                let f = -self.verticies.0.y / c_a_pt.y;
                c_a_pt.x *= f;
                c_a_pt.y *= f;
                c_a_pt.z *= f;
                c_a_pt = c_a_pt + self.verticies.0;

                return Triangle::new(self.verticies.0, a_b_pt, c_a_pt).clip(x_bound, y_bound);
            } else {
                let mut b_c_pt = self.verticies.1 - self.verticies.2;
                let f = -self.verticies.2.y / b_c_pt.y;
                b_c_pt.x *= f;
                b_c_pt.y *= f;
                b_c_pt.z *= f;
                b_c_pt = b_c_pt + self.verticies.2;

                let mut c = Triangle::new(self.verticies.0, a_b_pt, self.verticies.2).clip(x_bound, y_bound);
                c.append(&mut Triangle::new(a_b_pt, b_c_pt, self.verticies.2).clip(x_bound, y_bound));

                return c;
            }
        }

        if (self.verticies.2.y < 0.0){
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

            let mut c = Triangle::new(self.verticies.0, self.verticies.1, c_a_pt).clip(x_bound, y_bound);
            c.append(&mut Triangle::new(c_a_pt, self.verticies.1, b_c_pt).clip(x_bound, y_bound));

            return c;
        }

        vec![*self]
    }

    pub fn painters_algorithm(v: &Vec<Triangle>) -> Vec<Triangle>{
        let mut out: Vec<Triangle> = v.clone();
        out.sort_by(|b, a| a.center.z.partial_cmp(&b.center.z).unwrap());
        out
    }
}