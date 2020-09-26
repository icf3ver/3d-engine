use crate::matrix3x3::Matrix3x3;
use crate::mesh::Mesh;
use crate::triangle::Triangle;
use crate::vec3d::Vec3d;

use std::sync::mpsc;
use std::thread;

/// A Camera has a position and a rotation
/// as well as forward up and right vectors 
/// used for moving the camera quickly.
#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Vec3d,
    pub rotation: Vec3d,

    // movement vectors
    pub forward: Vec3d,
    pub right: Vec3d,
    pub up: Vec3d,
}

impl Camera{
    /// Creates a new camera object with a position and a roation.
    /// 
    /// # Arguments
    /// * `pos` - a Vec3d containing the position of the camera
    /// * `rotation` - a Vec3d containing the rotation of the camera
    /// 
    /// # Return
    /// 
    /// A new Camera Object
    /// 
    pub fn new (pos: Vec3d, rotation: Vec3d) -> Camera {
        Camera{
            position: pos,
            rotation: rotation,
            forward: Vec3d::new(0.0, 0.0, 1.0),
            right: Vec3d::new(1.0, 0.0, 0.0),
            up: Vec3d::new(0.0, 1.0, 0.0),
        }
    }

    /// Creates a vec and populates it with of all the projected triangles
    /// implements multithreading
    /// defines a rotation matrix
    /// 
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the camera the function was called for
    /// * `real_mesh` - The real mesh (not projected)
    /// * `size` - The dimensions of the screen
    /// 
    /// # Return
    /// 
    /// A vec containing the projected triangles
    /// 
    pub fn get_projected_triangles (&mut self,real_mesh: Mesh, size: (f32, f32)) -> Vec<Triangle> {
        // rotation martrix
        let rotation = self.rotation * (3.14159265/180.0);
        
        let r_x = Matrix3x3{m:[[1.0, 0.0, 0.0], [0.0, f32::cos(rotation.x), f32::sin(rotation.x)], [0.0, -f32::sin(rotation.x), f32::cos(rotation.x)]]};
        let r_y = Matrix3x3{m:[[f32::cos(rotation.y), 0.0, f32::sin(rotation.y)], [0.0, 1.0, 0.0], [-f32::sin(rotation.y), 0.0, f32::cos(rotation.y)]]};

        let r = r_x.clone() * r_y.clone();
        
        // directions
        self.forward = r.clone() * Vec3d::new(0.0, 0.0, 1.0);
        self.right = r.clone() * Vec3d::new(1.0, 0.0, 0.0);
        self.up = r.clone() * Vec3d::new(0.0, 1.0, 0.0);
        
        // deal with the triangles
        let this = self.clone();
        let mut tris: Vec<Triangle> = Vec::new();
        let s = real_mesh.tris.len();
        let (tx, rx) = mpsc::channel();
        for i in 0..s {
            let tri = real_mesh.tris[i];
            let tx = tx.clone();

            let n_r = r.clone();
            thread::spawn(move || {
                let look: Vec3d = Vec3d::new(
                    (tri.verticies.0.x + tri.verticies.1.x + tri.verticies.2.x)/3.0 - this.position.x,
                    (tri.verticies.0.y + tri.verticies.1.y + tri.verticies.2.y)/3.0 - this.position.y,
                    (tri.verticies.0.z + tri.verticies.1.z + tri.verticies.2.z)/3.0 - this.position.z);
                let look_len = f64::sqrt((look.x * look.x + look.y * look.y + look.z * look.z).into());
                let lx = (look.x as f64) / look_len;
                let ly = (look.y as f64) / look_len;
                let lz = (look.z as f64) / look_len;
                    
                let dot_product_normals: f64 = (tri.normal.x as f64 * lx) + (tri.normal.y as f64 * ly) + (tri.normal.z as f64 * lz);
                let dot_product_look: f64 = -(this.forward.x as f64 * lx) - (this.forward.y as f64 * ly) + (this.forward.z as f64 * lz);
                if dot_product_normals < 0.0 && dot_product_look > 0.4 {
                    //println!("{}", angle);
                    let mut tri_projected: Triangle = Triangle::new(
                        this.get_point_projection(tri.verticies.0, size, n_r.clone()),
                        this.get_point_projection(tri.verticies.1, size, n_r.clone()),
                        this.get_point_projection(tri.verticies.2, size, n_r.clone()));

                    tri_projected.color = tri.color;
                    tri_projected.center = Triangle::calculate_center(tri_projected.verticies);
                    
                    tri_projected.verticies.0.x += 1.0; tri_projected.verticies.0.y += 1.0;
                    tri_projected.verticies.1.x += 1.0; tri_projected.verticies.1.y += 1.0;
                    tri_projected.verticies.2.x += 1.0; tri_projected.verticies.2.y += 1.0;

                    tri_projected.verticies.0.x *= 0.5 * size.0; 
                    tri_projected.verticies.0.y *= 0.5 * size.1;
                    tri_projected.verticies.1.x *= 0.5 * size.0;
                    tri_projected.verticies.1.y *= 0.5 * size.1;
                    tri_projected.verticies.2.x *= 0.5 * size.0;
                    tri_projected.verticies.2.y *= 0.5 * size.1;

                    let mut projected_tris = tri_projected.clip(size.0, size.1);

                    for tri_n in 0..projected_tris.len(){
                        projected_tris[tri_n].dist = look_len as f32;
                    }
                    
                    tx.send(projected_tris).unwrap();
                }else {
                    tx.send(Vec::new()).unwrap();
                }
            });
        }

        // Collect results
        for _ in 0..s {
            tris.append(&mut rx.recv().unwrap());
        }

        Triangle::painters_algorithm(&tris)
    }
    
    /// Projection logic explained:
    ///
    ///               x
    ///             /
    ///  (.)e_2   /
    ///   o-----.------> e_1
    ///   s   / g
    ///     /
    ///   e
    ///
    /// note all variables shown are vectors
    ///
    /// x = real location of the cordinate denoted (x, y, z)
    /// e = the position of your eye directly behind the screen
    /// s = the position of the screen
    /// g = the projected point on the screen denoted (x', y', λ)
    /// e_1 = the x axis defined by your rotation
    /// e_2 = the y axis defined by your rotation. note: points up
    ///
    /// let v: vec3d = x - e; // vector from e to x
    /// // maths time
    /// f(λ) = e + λ(v) // so that  f(0) = e
    ///     //or        // and      f(1) = x
    /// f(λ) = e + λ(x - e)
    ///
    /// // plane/screen
    ///
    /// f(ς_1, ς_2) = s + ς_1 * e_1 + ς_2 * e_2
    /// e + λ(v) = s + ς_1 * e_1 + ς_2 * e_2
    /// 0 = ς_1 * e_1 + ς_2 * e_2 - λ(v) + s - e
    ///
    /// M = (e_1|e_2|e-x)
    ///               /ς_1\
    /// (e_1|e_2|e-x) |ς_2| = (e - s)
    ///               \ λ /
    /// // so
    ///   /ς_1\
    /// M |ς_2| = (e - s)
    ///   \ λ /
    /// // soooooo...
    /// /ς_1\
    /// |ς_2| = M^-1 (e - s)
    /// \ λ /   ^^^^ implemented in matrix3x3 
    ///
    /// // (: done
    ///
    /// note: since e is positioned directly behind s which I define as the center of the screen
    ///       e-s is realy just the opposite of the forwards vector.
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the camera the function was called for
    /// * `real` - The real position
    /// * `size` - The dimensions of the screen
    /// * `r`    - The rotation matrix
    /// 
    /// # Return
    /// 
    /// The projected Vec3d
    /// 
    pub fn get_point_projection (self, real: Vec3d, size: (f32, f32), r: Matrix3x3) -> Vec3d {
        let rotated_real = r.clone() * (real - self.position);

        let mut v_e1 = r.clone() * Vec3d::new(1.0, 0.0, 0.0);
        let mut v_e2 = r.clone() * Vec3d::new(0.0, 1.0, 0.0);
        let     v_f  = r.clone() * Vec3d::new(0.0, 0.0, 1.0);
        let     v_v  = r.clone() * rotated_real;

        // size the screen cord plane aspect ratio and screen limits
        v_e1.set_length(0.5 * (size.0/size.1));
        v_e2.set_length(0.5);
        
        let mut mat = Matrix3x3::from_vec3ds(v_e1, v_e2, v_v);
        let inverse_mat = mat.calculate_inverse();

        inverse_mat * (Vec3d::new(0.0, 0.0, 0.0) - v_f)
    }
}