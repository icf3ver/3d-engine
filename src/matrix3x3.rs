use crate::vec3d::Vec3d;
use std::ops::Mul;

/// A 3 x 3 Matrix
#[derive(Clone)]
pub struct Matrix3x3 {
    pub m: [[f32; 3]; 3],
}

impl Matrix3x3 {
    #[allow(dead_code)]
    pub fn new(matrix: [[f32; 3]; 3]) -> Matrix3x3 {
        Matrix3x3 { m: matrix }
    }
    /// Form a Matrix 3x3 from 3 Vec3d's
    ///
    /// # Argments
    /// * `col1` - column 1
    /// * `col2` - column 2
    /// * `col3` - column 3
    ///
    /// # Return
    /// A new Matrix3x3.
    ///
    pub fn from_vec3ds(col1: Vec3d, col2: Vec3d, col3: Vec3d) -> Matrix3x3 {
        Matrix3x3 {
            m: [
                [col1.x, col1.y, col1.z],
                [col2.x, col2.y, col2.z],
                [col3.x, col3.y, col3.z],
            ],
        }
    }

    #[allow(dead_code)]
    pub fn determinant(&mut self) -> f32 {
        let det: f32 = self.m[0][0]
            * ((self.m[1][1] * self.m[2][2]) - (self.m[1][2] * self.m[2][1]))
            - self.m[0][1] * ((self.m[1][0] * self.m[2][2]) - (self.m[1][2] * self.m[2][0]))
            + self.m[0][2] * ((self.m[1][0] * self.m[2][1]) - (self.m[1][1] * self.m[2][0]));

        det
    }

    /// Get the inverse of a 3x3 matrix.
    ///
    /// # Arguments
    /// * `self` - The matrix the function is being called for
    ///
    /// # Return
    /// The inverse Matrix
    ///
    pub fn calculate_inverse(&mut self) -> Matrix3x3 {
        // matrix (a, b, c)
        //        (d, e, f)
        //        (g, h, i)
        // determinant of a 3x3 matrix is |A| = a(ei − fh)
        //                                    − b(di − fg)
        //                                    + c(dh − eg)

        let det: f32 = self.m[0][0]
            * ((self.m[1][1] * self.m[2][2]) - (self.m[1][2] * self.m[2][1]))
            - self.m[0][1] * ((self.m[1][0] * self.m[2][2]) - (self.m[1][2] * self.m[2][0]))
            + self.m[0][2] * ((self.m[1][0] * self.m[2][1]) - (self.m[1][1] * self.m[2][0]));

        // transpose
        // matrix (a, b, c)
        //        (d, e, f)
        //        (g, h, i)
        //
        // matrix (a, d, g)
        //        (b, e, h)
        //        (c, f, i)

        let transposed = Matrix3x3 {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2]],
            ],
        };

        // matrix (a, b, c)
        //        (d, e, f)
        //        (g, h, i)

        // Adjugate matrix
        // matrix ( |e, f|,  |d, f|,  |d, e|)
        //        ( |h, i|  -|g, i|   |g, h|)
        //
        //        ( |b, c|,  |a, c|,  |a, b|)
        //        (-|h, i|   |g, i|  -|g, h|)
        //
        //        ( |b, c|,  |a, c|,  |a, b|)
        //        ( |e, f|  -|d, f|   |d, e|)
        //
        // matrix (a, b)
        //        (c, d)
        // determinant of 2x2 matrix if |A| = (a * d) - (b * c)
        //
        // so
        //
        // matrix ( ((e * i) - (f * h)), -((d * i) - (f * g)),  ((d * h) - (e * g)))
        //        (-((b * i) - (c * h)),  ((a * i) - (c * g)), -((a * h) - (b * g)))
        //        ( ((b * f) - (c * e)), -((a * f) - (c * d)),  ((a * e) - (b * d)))

        // matrix (a, b, c)
        //        (d, e, f)
        //        (g, h, i)

        let a = transposed.m[0][0];
        let b = transposed.m[0][1];
        let c = transposed.m[0][2];
        let d = transposed.m[1][0];
        let e = transposed.m[1][1];
        let f = transposed.m[1][2];
        let g = transposed.m[2][0];
        let h = transposed.m[2][1];
        let i = transposed.m[2][2];

        let a0_0 = (e * i) - (f * h);
        let b1_0 = -((d * i) - (f * g));
        let c2_0 = (d * h) - (e * g);
        let d0_1 = -((b * i) - (c * h));
        let e1_1 = (a * i) - (c * g);
        let f2_1 = -((a * h) - (b * g));
        let g0_2 = (b * f) - (c * e);
        let h1_2 = -((a * f) - (c * d));
        let i2_2 = (a * e) - (b * d);
        let adjugate_mat = Matrix3x3 {
            m: [[a0_0, d0_1, g0_2], [b1_0, e1_1, h1_2], [c2_0, f2_1, i2_2]],
        };

        adjugate_mat * (1.00000 / det)
    }
}

/// Matrix3x3 * Matrix3x3 = Matrix3x3
impl Mul for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            m: [
                [
                    self.m[0][0] * rhs.m[0][0]
                        + self.m[0][1] * rhs.m[1][0]
                        + self.m[0][2] * rhs.m[2][0],
                    self.m[0][0] * rhs.m[0][1]
                        + self.m[0][1] * rhs.m[1][1]
                        + self.m[0][2] * rhs.m[2][1],
                    self.m[0][0] * rhs.m[0][2]
                        + self.m[0][1] * rhs.m[1][2]
                        + self.m[0][2] * rhs.m[2][2],
                ],
                [
                    self.m[1][0] * rhs.m[0][0]
                        + self.m[1][1] * rhs.m[1][0]
                        + self.m[1][2] * rhs.m[2][0],
                    self.m[1][0] * rhs.m[0][1]
                        + self.m[1][1] * rhs.m[1][1]
                        + self.m[1][2] * rhs.m[2][1],
                    self.m[1][0] * rhs.m[0][2]
                        + self.m[1][1] * rhs.m[1][2]
                        + self.m[1][2] * rhs.m[2][2],
                ],
                [
                    self.m[2][0] * rhs.m[0][0]
                        + self.m[2][1] * rhs.m[1][0]
                        + self.m[2][2] * rhs.m[2][0],
                    self.m[2][0] * rhs.m[0][1]
                        + self.m[2][1] * rhs.m[1][1]
                        + self.m[2][2] * rhs.m[2][1],
                    self.m[2][0] * rhs.m[0][2]
                        + self.m[2][1] * rhs.m[1][2]
                        + self.m[2][2] * rhs.m[2][2],
                ],
            ],
        }
    }
}

/// Matrix3x3 * Vec3d = Vec3d
impl Mul<Vec3d> for Matrix3x3 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(
            rhs.x * self.m[0][0] + rhs.y * self.m[0][1] + rhs.z * self.m[0][2],
            rhs.x * self.m[1][0] + rhs.y * self.m[1][1] + rhs.z * self.m[1][2],
            rhs.x * self.m[2][0] + rhs.y * self.m[2][1] + rhs.z * self.m[2][2],
        )
    }
}

/// Matrix3x3 * f32 = Matrix3x3
impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix3x3 {
            m: [
                [self.m[0][0] * rhs, self.m[0][1] * rhs, self.m[0][2] * rhs],
                [self.m[1][0] * rhs, self.m[1][1] * rhs, self.m[1][2] * rhs],
                [self.m[2][0] * rhs, self.m[2][1] * rhs, self.m[2][2] * rhs],
            ],
        }
    }
}
