use crate::matrix3x3::Matrix3x3;
use crate::mesh::Mesh;
use crate::triangle::Triangle;
use crate::vec3d::Vec3d;

use ggez;

#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Vec3d,
    pub rotation: Vec3d,

    pub forward: Vec3d,
    pub right: Vec3d,
    pub up: Vec3d,
}

impl Camera {
    pub fn new (pos: Vec3d, rotation: Vec3d, f_far: f32, f_near: f32, f_fov: f32, ctx: &ggez::Context) -> Camera {
        let size: (f32, f32) = ggez::graphics::drawable_size(ctx);

        let f_aspect_ratio: f32 = size.1 / size.0;
        let f_fov_rad = 1.0/ f32::tan((f_fov * 0.5)/(180.0 * 3.14159));

        Camera{
            position: pos,
            rotation: rotation,
            forward: Vec3d::new(0.0, 0.0, 1.0),
            right: Vec3d::new(1.0, 0.0, 0.0),
            up: Vec3d::new(0.0, 1.0, 0.0),
        }
    }

    pub fn get_projected_triangles(&mut self, real_mesh: &Mesh, size: (f32, f32)) -> Vec<Triangle> {
        let mut tris: Vec<Triangle> = Vec::new();
        for tri in &real_mesh.tris{
            let look: Vec3d = Vec3d::new(
                (tri.verticies.0.x + tri.verticies.1.x + tri.verticies.2.x)/3.0 - self.position.x,
                (tri.verticies.0.y + tri.verticies.1.y + tri.verticies.2.y)/3.0 - self.position.y,
                (tri.verticies.0.z + tri.verticies.1.z + tri.verticies.2.z)/3.0 - self.position.z);
            let look_len = f64::sqrt((look.x * look.x + look.y * look.y + look.z * look.z).into());
            let lx = (look.x as f64) / look_len;
            let ly = (look.y as f64) / look_len;
            let lz = (look.z as f64) / look_len;
                
            let dot_product: f64 = (tri.normal.x as f64 * lx) + (tri.normal.y as f64 * ly) + (tri.normal.z as f64 * lz);
            if dot_product < 0.0 {
                let mut tri_projected: Triangle = Triangle::new(
                    self.get_point_projection(tri.verticies.0, size),
                    self.get_point_projection(tri.verticies.1, size),
                    self.get_point_projection(tri.verticies.2, size));

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

                let mut tmp = vec![tri_projected];//.clip(size.0, size.1);
                tris.append(&mut tmp);
            }
        }

        Triangle::painters_algorithm(&tris)
    }
    /**
     * Projection logic explained:
     * 
     *                x
     *              /
     *   (.)e_2   /
     *    o-----.------> e_1
     *    s   / g
     *      /
     *    e
     * 
     * note all variables shown are vectors
     * 
     * x = real location of the cordinate denoted (x, y, z)
     * e = the position of your eye directly behind the screen
     * s = the position of the screen
     * g = the projected point on the screen denoted (x', y', λ)
     * e_1 = the x axis defined by your rotation
     * e_2 = the y axis defined by your rotation. note: points up
     * 
     * let v: vec3d = x - e; // vector from e to x
     * // maths time
     * f(λ) = e + λ(v) // so that  f(0) = e
     *     //or        // and      f(1) = x
     * f(λ) = e + λ(x - e)
     * 
     * // plane/screen
     * 
     * f(ς_1, ς_2) = s + ς_1 * e_1 + ς_2 * e_2
     * e + λ(v) = s + ς_1 * e_1 + ς_2 * e_2
     * 0 = ς_1 * e_1 + ς_2 * e_2 - λ(v) + s - e
     * 
     * M = (e_1|e_2|e-x)
     *               /ς_1\
     * (e_1|e_2|e-x) |ς_2| = (e - s)
     *               \ λ /
     * // so
     *   /ς_1\
     * M |ς_2| = (e - s)
     *   \ λ /
     * // soooooo...
     * /ς_1\
     * |ς_2| = M^-1 (e - s)
     * \ λ /   ^^^^ implemented in matrix3x3 
     * 
     * // (: done
     * 
     * note: since e is positioned directly behind s which i define as the center of the screen
     *       e-s is realy just the opposite of the forwards vector.
     * 
     */
    pub fn get_point_projection (&mut self, real: Vec3d, size: (f32, f32)) -> Vec3d {
        let rotation = self.rotation * (3.14159265/180.0);
        
        let R_x = Matrix3x3{m:[[1.0, 0.0, 0.0], [0.0, f32::cos(rotation.x), f32::sin(rotation.x)], [0.0, -f32::sin(rotation.x), f32::cos(rotation.x)]]};
        let R_y = Matrix3x3{m:[[f32::cos(rotation.y), 0.0, f32::sin(rotation.y)], [0.0, 1.0, 0.0], [-f32::sin(rotation.y), 0.0, f32::cos(rotation.y)]]};

        let R = R_x.clone() * R_y.clone();
        
        let rotated_real = R.clone() * (real - self.position);

        let mut v_f  = R.clone() * Vec3d::new(0.0, 0.0, 1.0);
        let mut v_e1 = R.clone() * Vec3d::new(1.0, 0.0, 0.0);
        let mut v_e2 = R.clone() * Vec3d::new(0.0, 1.0, 0.0);
        let mut v_v  = R.clone() * rotated_real;

        // size the screen cord plane aspect ratio and screen limits
        v_e1.limit(0.5 * (size.0/size.1));
        v_e2.limit(0.5);
        

        let mut mat = Matrix3x3::from_vec3ds(v_e1, v_e2, v_v);
        let mut inverse_mat = mat.calculate_inverse();

        inverse_mat * (Vec3d::new(0.0, 0.0, 0.0) - v_f)
    }
}