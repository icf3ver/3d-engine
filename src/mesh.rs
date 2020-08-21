use ggez;              // Graphics

use std::fs;           // Read in files
use std::fs::File;
use std::io::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::camera::Camera;
use crate::vec3d::Vec3d;
use crate::triangle::Triangle;

#[derive(Clone)]
pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub camera: Camera,
}

impl Mesh{
    pub fn new(ctx: &mut ggez::Context, camera: Camera) -> ggez::GameResult<Mesh> {
        Ok(Mesh{tris: Vec::new(), camera: camera})
    }

    pub fn form_cube(&mut self){
        self.tris = vec![
            // FRONT
            Triangle::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 1.0, 0.0), Vec3d::new(1.0, 1.0, 0.0)),
            Triangle::new(Vec3d{ x: 0.0, y: 0.0, z: 0.0}, Vec3d{ x: 1.0, y: 1.0, z: 0.0}, Vec3d{ x: 1.0, y: 0.0, z: 0.0}),
            
            // RIGHT
            Triangle::new(Vec3d{ x: 1.0, y: 0.0, z: 0.0}, Vec3d{ x: 1.0, y: 1.0, z: 0.0}, Vec3d{ x: 1.0, y: 1.0, z: 1.0}),
            Triangle::new(Vec3d{ x: 1.0, y: 0.0, z: 0.0}, Vec3d{ x: 1.0, y: 1.0, z: 1.0}, Vec3d{ x: 1.0, y: 0.0, z: 1.0}),
            
            // BACK
            Triangle::new(Vec3d{ x: 1.0, y: 0.0, z: 1.0}, Vec3d{ x: 1.0, y: 1.0, z: 1.0}, Vec3d{ x: 0.0, y: 1.0, z: 1.0}),
            Triangle::new(Vec3d{ x: 1.0, y: 0.0, z: 1.0}, Vec3d{ x: 0.0, y: 1.0, z: 1.0}, Vec3d{ x: 0.0, y: 0.0, z: 1.0}),

            // LEFT
            Triangle::new(Vec3d{ x: 0.0, y: 0.0, z: 1.0}, Vec3d{ x: 0.0, y: 1.0, z: 1.0}, Vec3d{ x: 0.0, y: 1.0, z: 0.0}),
            Triangle::new(Vec3d{ x: 0.0, y: 0.0, z: 1.0}, Vec3d{ x: 0.0, y: 1.0, z: 0.0}, Vec3d{ x: 0.0, y: 0.0, z: 0.0}),

            // TOP
            Triangle::new(Vec3d{ x: 0.0, y: 1.0, z: 0.0}, Vec3d{ x: 0.0, y: 1.0, z: 1.0}, Vec3d{ x: 1.0, y: 1.0, z: 1.0}),
            Triangle::new(Vec3d{ x: 0.0, y: 1.0, z: 0.0}, Vec3d{ x: 1.0, y: 1.0, z: 1.0}, Vec3d{ x: 1.0, y: 1.0, z: 0.0}),

            // BOTTOM
            Triangle::new(Vec3d{ x: 1.0, y: 0.0, z: 1.0}, Vec3d{ x: 0.0, y: 0.0, z: 1.0}, Vec3d{ x: 0.0, y: 0.0, z: 0.0}),
            Triangle::new(Vec3d{ x: 1.0, y: 0.0, z: 1.0}, Vec3d{ x: 0.0, y: 0.0, z: 0.0}, Vec3d{ x: 1.0, y: 0.0, z: 0.0}),
        ];
    }

    pub fn x_axis_rotation(&mut self, r: f32, origin_y: f32, origin_z: f32) {
        for i in 0..self.tris.len() {
            self.tris[i].x_axis_rotation(r, origin_y, origin_z);
        }
    }

    pub fn y_axis_rotation(&mut self, r: f32, origin_x: f32, origin_z: f32) {
        for i in 0..self.tris.len() {
            self.tris[i].y_axis_rotation(r, origin_x, origin_z);
        }
    }

    pub fn z_axis_rotation(&mut self, r: f32, origin_x: f32, origin_y: f32) {
        for i in 0..self.tris.len() {
            self.tris[i].z_axis_rotation(r, origin_x, origin_y);
        }
    }

    pub fn increment_x(&mut self, x: f32){
        for i in 0..self.tris.len() {
            self.tris[i].increment_x(x);
        }
    }

    pub fn increment_y(&mut self, y: f32){
        for i in 0..self.tris.len() {
            self.tris[i].increment_y(y);
        }
    }

    pub fn increment_z(&mut self, z: f32){
        for i in 0..self.tris.len() {
            self.tris[i].increment_z(z);
        }
    }

    pub fn from_file(&mut self, filename: &str) {
        let file_type = filename.split('.').last().unwrap();
        if file_type == "obj" {
            self.from_obj(filename);
        }else if file_type == "stl" {
            match self.from_stl_ascii(filename){
                Ok(_) => (),
                Err(_e) => self.from_stl_bin(filename),
            }

        }
    }

    pub fn from_obj(&mut self, filename: &str) {
        println!("In file {}", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let mut points: Vec<Vec3d> = Vec::new();
        for line in contents.lines(){
            if line.chars().next().unwrap() == 'v' {
                let mut e = line.split_whitespace();
                e.next();
                points.push(Vec3d{x: e.next().unwrap().parse::<f32>().unwrap(),
                                  y: e.next().unwrap().parse::<f32>().unwrap(),
                                  z: e.next().unwrap().parse::<f32>().unwrap()});
            }else if line.chars().next().unwrap() == 'f' {
                let mut e = line.split_whitespace();
                e.next();
                self.tris.push(Triangle::new(points[e.next().unwrap().parse::<usize>().unwrap()-1_usize],
                                                    points[e.next().unwrap().parse::<usize>().unwrap()-1_usize],
                                                    points[e.next().unwrap().parse::<usize>().unwrap()-1_usize]));
            }
        }
    }

    pub fn from_stl_bin(&mut self, filename: &str) {
        println!("In file {}", filename);

        let mut file = File::open(filename).expect("file not found");

        // header
        let mut header_buf = [0; 80];
        let header = file.read(&mut header_buf[..])
            .expect("Something went wrong reading the file");
        let contents = String::from_utf8_lossy(&header_buf[..header]);
        
        // number of triangles
        let mut n_tris_buf = [0; 4];
        let n_tris_raw = file.read(&mut n_tris_buf[..]).expect("Something went wrong reading the file");
        let mut n_tris_dat = &n_tris_buf[..n_tris_raw];
        let n_tris = *&n_tris_dat.read_u32::<LittleEndian>().unwrap() as i32;
        println!("{:?}", n_tris);

        let mut tri_buf = [0; 50];
        for _i in 0..n_tris {
            let tri_raw = file.read(&mut tri_buf[..])
                .expect("Something went wrong reading the file");
            let mut data = &tri_buf[..tri_raw];
            self.tris.push(
                Triangle::new_with_normal(
                    Vec3d{ // Normal
                        x: *&data.read_f32::<LittleEndian>().unwrap(),
                        y: *&data.read_f32::<LittleEndian>().unwrap(),
                        z: *&data.read_f32::<LittleEndian>().unwrap(),
                    },
                    Vec3d{ // Verticies
                        x: *&data.read_f32::<LittleEndian>().unwrap(),
                        y: *&data.read_f32::<LittleEndian>().unwrap(),
                        z: *&data.read_f32::<LittleEndian>().unwrap(),
                    },
                    Vec3d{
                        x: *&data.read_f32::<LittleEndian>().unwrap(),
                        y: *&data.read_f32::<LittleEndian>().unwrap(),
                        z: *&data.read_f32::<LittleEndian>().unwrap(),
                    },
                    Vec3d{
                        x: *&data.read_f32::<LittleEndian>().unwrap(),
                        y: *&data.read_f32::<LittleEndian>().unwrap(),
                        z: *&data.read_f32::<LittleEndian>().unwrap(),
                    },
                )
            );

            // Attribute byte count
            &data.read_u16::<LittleEndian>().unwrap();
        }
    }

    pub fn from_stl_ascii(&mut self, filename: &str) -> Result<(), std::io::Error> {
        println!("In file {}", filename);

        let contents = fs::read_to_string(filename)?;
        
        let mut points: Vec<Vec3d> = Vec::new();
        let mut normal: Vec3d = Vec3d::new(0.0, 0.0, 0.0);
        for line in contents.lines(){
            let mut e = line.split_whitespace();
            let s = e.next().unwrap();
            if s == "facet"{
                if e.next().unwrap() == "normal"{
                    normal = Vec3d::new(e.next().unwrap().parse::<f32>().unwrap(),
                                        e.next().unwrap().parse::<f32>().unwrap(),
                                        e.next().unwrap().parse::<f32>().unwrap());
                }
            }else if s == "vertex" {
                points.push(Vec3d::new(e.next().unwrap().parse::<f32>().unwrap(),
                                       e.next().unwrap().parse::<f32>().unwrap(),
                                       e.next().unwrap().parse::<f32>().unwrap()));
            }else if s == "endfacet" {
                self.tris.push(Triangle::new_with_normal(normal, points[0], points[1], points[2]));
                normal = Vec3d::new(0.0, 0.0, 0.0);
                points = Vec::new();
            }
        }
        Ok(())
    }

    pub fn painters_algorithm(&mut self) {
        self.tris.sort_by(|b, a| a.center.z.partial_cmp(&b.center.z).unwrap());
    }

    pub fn get_mesh_relative_camera(&mut self) -> Mesh{
        let mut transformed_mesh: Mesh = self.clone();
        //transformed_mesh.y_axis_rotation(self.camera.rotation.y, self.camera.position.x, self.camera.position.z - 0.1);
        //transformed_mesh.x_axis_rotation(self.camera.rotation.x, self.camera.position.y, self.camera.position.z - 0.1);
        //transformed_mesh.z_axis_rotation(self.camera.rotation.z, self.camera.position.x, self.camera.position.y);

        //transformed_mesh.increment_x(-self.camera.position.x);
        //transformed_mesh.increment_y(-self.camera.position.y);
        //transformed_mesh.increment_z(-self.camera.position.z);
        transformed_mesh
    }
}