use crate::vec3d::Vec3d;

#[derive(Clone)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4{
    //bad
    pub fn multiply_vector(&mut self, i: &Vec3d) -> Vec3d {
        let mut o: Vec3d = Vec3d::new(0.0, 0.0, 0.0);
        o.x = i.x * self.m[0][0] + i.y * self.m[1][0] + i.z * self.m[2][0] + self.m[3][0];
        o.y = i.x * self.m[0][1] + i.y * self.m[1][1] + i.z * self.m[2][1] + self.m[3][1];
        o.z = i.x * self.m[0][2] + i.y * self.m[1][2] + i.z * self.m[2][2] + self.m[3][2];
        let w: f32 = i.x * self.m[0][3] + i.y * self.m[1][3] + i.z * self.m[2][3] + self.m[3][3];
    
        if w != 0.0 {
            o.x /= w;
            o.y /= w;
            o.z /= w;
        }
    
        o
    }
}
