use ggez;              // Graphics
use ggez::nalgebra::geometry::Point2;

use crate::vec3d::Vec3d;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub normal: Vec3d,
    pub verticies: (Vec3d, Vec3d, Vec3d),
}

impl Triangle{
    pub fn new(vertex1: Vec3d, vertex2: Vec3d, vertex3: Vec3d) -> Triangle {
        let normal = Triangle::calculate_normal((vertex1, vertex2, vertex3));
        Triangle{normal: normal, verticies: (vertex1, vertex2, vertex3)}
    }

    pub fn new_with_normal(normal: Vec3d, vertex1: Vec3d, vertex2: Vec3d, vertex3: Vec3d) -> Triangle {
        Triangle{normal: normal, verticies: (vertex1, vertex2, vertex3)}
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

    pub fn get_rayback_pt(&mut self, point: Vec3d){
        // todo
    }

    pub fn check_point_overlap_plane(){

    }
}