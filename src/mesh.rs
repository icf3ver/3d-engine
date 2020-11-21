use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::{self, File}, io::prelude::*};
use crate::{triangle::Triangle, vec3d::Vec3d};

/// A Mesh is a 3D object made up of triangles.
/// It also has a camera that is looking at it.
///
/// NOTE: Structure is subject to change
///
#[derive(Clone)]
pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub is_over: bool,
    pub is_held: bool,
    pub pos: Vec3d,
}

impl Mesh {
    /// Creates a new Mesh with a camera
    ///
    /// # Arguments
    /// * `camera` - The Camera viewing the Mesh.
    ///
    /// # Return
    /// A GameRusult<Mesh> object
    ///
    pub fn new(pos: Vec3d) -> Mesh {
        Mesh {
            tris: Vec::new(),
            is_over: false,
            is_held: false,
            pos: pos,
        }
    }

    /// Form a cube and add set it to the Mesh.
    ///
    /// # Argumetns
    /// * `self` - The Mesh the function was called for.
    ///
    #[allow(dead_code)]
    pub fn form_cube(&mut self) {
        self.tris = vec![
            // FRONT
            Triangle::new(
                Vec3d::new(-0.5, -0.5, -0.5),
                Vec3d::new(-0.5, 0.5, -0.5),
                Vec3d::new(0.5, 0.5, -0.5),
            ) + self.pos,
            Triangle::new(
                Vec3d::new(-0.5, -0.5, -0.5),
                Vec3d::new(0.5, 0.5, -0.5),
                Vec3d::new(0.5, -0.5, -0.5),
            ) + self.pos,
            // RIGHT
            Triangle::new(
                Vec3d::new(0.5, -0.5, -0.5),
                Vec3d::new(0.5, 0.5, -0.5),
                Vec3d::new(0.5, 0.5, 0.5),
            ) + self.pos,
            Triangle::new(
                Vec3d::new(0.5, -0.5, -0.5),
                Vec3d::new(0.5, 0.5, 0.5),
                Vec3d::new(0.5, -0.5, 0.5),
            ) + self.pos,
            // BACK
            Triangle::new(
                Vec3d::new(0.5, -0.5, 0.5),
                Vec3d::new(0.5, 0.5, 0.5),
                Vec3d::new(-0.5, 0.5, 0.5),
            ) + self.pos,
            Triangle::new(
                Vec3d::new(0.5, -0.5, 0.5),
                Vec3d::new(-0.5, 0.5, 0.5),
                Vec3d::new(-0.5, -0.5, 0.5),
            ) + self.pos,
            // LEFT
            Triangle::new(
                Vec3d::new(-0.5, -0.5, 0.5),
                Vec3d::new(-0.5, 0.5, 0.5),
                Vec3d::new(-0.5, 0.5, -0.5),
            ) + self.pos,
            Triangle::new(
                Vec3d::new(-0.5, -0.5, 0.5),
                Vec3d::new(-0.5, 0.5, -0.5),
                Vec3d::new(-0.5, -0.5, -0.5),
            ) + self.pos,
            // TOP
            Triangle::new(
                Vec3d::new(-0.5, 0.5, -0.5),
                Vec3d::new(-0.5, 0.5, 0.5),
                Vec3d::new(0.5, 0.5, 0.5),
            ) + self.pos,
            Triangle::new(
                Vec3d::new(-0.5, 0.5, -0.5),
                Vec3d::new(0.5, 0.5, 0.5),
                Vec3d::new(0.5, 0.5, -0.5),
            ) + self.pos,
            // BOTTOM
            Triangle::new(
                Vec3d::new(0.5, -0.5, 0.5),
                Vec3d::new(-0.5, -0.5, 0.5),
                Vec3d::new(-0.5, -0.5, -0.5),
            ) + self.pos,
            Triangle::new(
                Vec3d::new(0.5, -0.5, 0.5),
                Vec3d::new(-0.5, -0.5, -0.5),
                Vec3d::new(0.5, -0.5, -0.5),
            ) + self.pos,
        ];
    }

    /// Rotates the Mesh arround the x-axis at the origin point.
    ///
    /// # Arguments
    /// * `self` - The Mesh the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_y` - The y position of the origin point.
    /// * `origin_z` - The z position of the origin point.
    ///
    #[allow(dead_code)]
    pub fn x_axis_rotation(&mut self, r: f32, origin_y: f32, origin_z: f32) {
        for i in 0..self.tris.len() {
            self.tris[i].x_axis_rotation(r, origin_y, origin_z);
        }
    }

    /// Rotates the Mesh arround the y-axis at the origin point.
    ///
    /// # Arguments
    /// * `self` - The Mesh the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_x` - The x position of the origin point.
    /// * `origin_z` - The z position of the origin point.
    ///
    #[allow(dead_code)]
    pub fn y_axis_rotation(&mut self, r: f32, origin_x: f32, origin_z: f32) {
        for i in 0..self.tris.len() {
            self.tris[i].y_axis_rotation(r, origin_x, origin_z);
        }
    }

    /// Rotates the Mesh arround the z-axis at the origin point.
    ///
    /// # Arguments
    /// * `self` - The Mesh the function was called for.
    /// * `r` - The Amount rotated by.
    /// * `origin_x` - The x position of the origin point.
    /// * `origin_y` - The y position of the origin point.
    ///
    #[allow(dead_code)]
    pub fn z_axis_rotation(&mut self, r: f32, origin_x: f32, origin_y: f32) {
        for i in 0..self.tris.len() {
            self.tris[i].z_axis_rotation(r, origin_x, origin_y);
        }
    }

    /// Increment the Mesh x position by a number.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `inc_x` - The number the x position will be incremented by.
    ///
    #[allow(dead_code)]
    pub fn increment_x(&mut self, inc_x: f32) {
        self.pos.x += inc_x;
        for i in 0..self.tris.len() {
            self.tris[i].increment_x(inc_x);
        }
    }

    /// Increment the Mesh y position by a number.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `inc_y` - The number the y position will be incremented by.
    ///
    #[allow(dead_code)]
    pub fn increment_y(&mut self, inc_y: f32) {
        self.pos.y += inc_y;
        for i in 0..self.tris.len() {
            self.tris[i].increment_y(inc_y);
        }
    }

    /// Increment the Mesh z position by a number.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `inc_z` - The number the z position will be incremented by.
    ///
    #[allow(dead_code)]
    pub fn increment_z(&mut self, inc_z: f32) {
        self.pos.z += inc_z;
        for i in 0..self.tris.len() {
            self.tris[i].increment_z(inc_z);
        }
    }

    /// Make a Mesh from a file.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `filename` - The filename of the file containing the data.
    ///
    #[allow(dead_code)]
    pub fn from_file(&mut self, filename: &str) {
        println!("In file {}", filename);
        let file_type = filename.split('.').last().unwrap();
        if file_type == "obj" {
            self.from_obj(filename);
        } else if file_type == "stl" {
            match self.from_stl_ascii(filename) {
                Ok(_) => (),
                Err(_e) => self.from_stl_bin(filename),
            }
        }
    }

    /// Make a Mesh from an obj file.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `filename` - The filename of the obj file containing the data.
    ///
    pub fn from_obj(&mut self, filename: &str) {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let mut points: Vec<Vec3d> = Vec::new();
        for line in contents.lines() {
            if line.chars().next().unwrap() == 'v' {
                let mut e = line.split_whitespace();
                e.next();
                points.push(Vec3d {
                    x: e.next().unwrap().parse::<f32>().unwrap(),
                    y: e.next().unwrap().parse::<f32>().unwrap(),
                    z: e.next().unwrap().parse::<f32>().unwrap(),
                });
            } else if line.chars().next().unwrap() == 'f' {
                let mut e = line.split_whitespace();
                e.next();
                self.tris.push(Triangle::new(
                    points[e.next().unwrap().parse::<usize>().unwrap() - 1_usize],
                    points[e.next().unwrap().parse::<usize>().unwrap() - 1_usize],
                    points[e.next().unwrap().parse::<usize>().unwrap() - 1_usize],
                ));
            }
        }
    }

    /// Make a Mesh from an stl binary file.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `filename` - The filename of the stl binary file containing the data.
    ///
    pub fn from_stl_bin(&mut self, filename: &str) {
        let mut file = File::open(filename).expect("file not found");

        // header
        let mut header_buf = [0; 80];
        let header = file
            .read(&mut header_buf[..])
            .expect("Something went wrong reading the file");
        let _contents = String::from_utf8_lossy(&header_buf[..header]);

        // number of triangles
        let mut n_tris_buf = [0; 4];
        let n_tris_raw = file
            .read(&mut n_tris_buf[..])
            .expect("Something went wrong reading the file");
        let mut n_tris_dat = &n_tris_buf[..n_tris_raw];
        let n_tris = *&n_tris_dat.read_u32::<LittleEndian>().unwrap() as i32;
        println!("{:?}", n_tris);

        let mut tri_buf = [0; 50];
        for _i in 0..n_tris {
            let tri_raw = file
                .read(&mut tri_buf[..])
                .expect("Something went wrong reading the file");
            let mut data = &tri_buf[..tri_raw];
            self.tris.push(Triangle::new_with_normal(
                Vec3d {
                    // Normal
                    x: *&data.read_f32::<LittleEndian>().unwrap(),
                    y: *&data.read_f32::<LittleEndian>().unwrap(),
                    z: *&data.read_f32::<LittleEndian>().unwrap(),
                },
                Vec3d {
                    // Verticies
                    x: *&data.read_f32::<LittleEndian>().unwrap(),
                    y: *&data.read_f32::<LittleEndian>().unwrap(),
                    z: *&data.read_f32::<LittleEndian>().unwrap(),
                },
                Vec3d {
                    x: *&data.read_f32::<LittleEndian>().unwrap(),
                    y: *&data.read_f32::<LittleEndian>().unwrap(),
                    z: *&data.read_f32::<LittleEndian>().unwrap(),
                },
                Vec3d {
                    x: *&data.read_f32::<LittleEndian>().unwrap(),
                    y: *&data.read_f32::<LittleEndian>().unwrap(),
                    z: *&data.read_f32::<LittleEndian>().unwrap(),
                },
            ));

            // Attribute byte count
            &data.read_u16::<LittleEndian>().unwrap();
        }
    }

    /// Make a Mesh from an stl ascii file.
    ///
    /// # Arguments
    /// * `self` - The Mesh this function was called for.
    /// * `filename` - The filename of the stl ascii file containing the data.
    ///
    /// # Return
    /// Success status
    ///
    pub fn from_stl_ascii(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let contents = fs::read_to_string(filename)?;

        let mut points: Vec<Vec3d> = Vec::new();
        let mut normal: Vec3d = Vec3d::new(0.0, 0.0, 0.0);
        for line in contents.lines() {
            let mut e = line.split_whitespace();
            let s = e.next().unwrap();
            if s == "facet" {
                if e.next().unwrap() == "normal" {
                    normal = Vec3d::new(
                        e.next().unwrap().parse::<f32>().unwrap(),
                        e.next().unwrap().parse::<f32>().unwrap(),
                        e.next().unwrap().parse::<f32>().unwrap(),
                    );
                }
            } else if s == "vertex" {
                points.push(Vec3d::new(
                    e.next().unwrap().parse::<f32>().unwrap(),
                    e.next().unwrap().parse::<f32>().unwrap(),
                    e.next().unwrap().parse::<f32>().unwrap(),
                ));
            } else if s == "endfacet" {
                self.tris.push(Triangle::new_with_normal(
                    normal, points[0], points[1], points[2],
                ));
                normal = Vec3d::new(0.0, 0.0, 0.0);
                points = Vec::new();
            }
        }
        Ok(())
    }
}
