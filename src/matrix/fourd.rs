use std::ops::{Add, Deref, DerefMut, Mul};

use crate::point::fourd::Point;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Matrix4x4(pub [[f128; 4]; 4]);

impl Matrix4x4 {
    /// Rotation matrix for rotation across the zw plane. Theta is in radians
    pub fn rotate_zw_theta(theta: f128) -> Matrix4x4 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix4x4([
            [cos, -sin, 0., 0.],
            [sin, cos,  0., 0.],
            [0., 0.,    1., 0.],
            [0., 0.,    0., 1.],
        ])
    }

    /// Rotation matrix for rotation across the yw plane. Theta is in radians
    pub fn rotate_yw_theta(theta: f128) -> Matrix4x4 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix4x4([
            [cos, 0., -sin, 0.],
            [0.,  1.,   0., 0.],
            [sin, 0.,  cos, 0.],
            [0.,  0.,   0., 1.],
        ])
    }

    /// Rotation matrix for rotation across the yz plane. Theta is in radians
    pub fn rotate_yz_theta(theta: f128) -> Matrix4x4 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix4x4([
            [cos, 0., 0., -sin],
            [0.,  1., 0., 0.  ],
            [0.,  0., 1., 0.  ],
            [sin, 0., 0., cos ],
        ])
    }

    /// Rotation matrix for rotation across the xw plane. Theta is in radians
    pub fn rotate_xw_theta(theta: f128) -> Matrix4x4 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix4x4([
            [1., 0.,    0., 0.],
            [0., cos, -sin, 0.],
            [0., sin,  cos, 0.],
            [0., 0.,    0., 1.],
        ])
    }

    /// Rotation matrix for rotation across the xz plane. Theta is in radians
    pub fn rotate_xz_theta(theta: f128) -> Matrix4x4 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix4x4([
            [1., 0.,  0.,   0.],
            [0., cos, 0., -sin],
            [0., 0.,  1.,   0.],
            [0., sin, 0.,  cos],
        ])
    }

    /// Rotation matrix for rotation across the xy plane. Theta is in radians
    pub fn rotate_xy_theta(theta: f128) -> Matrix4x4 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix4x4([
            [1., 0., 0.,    0.],
            [0., 1., 0.,    0.],
            [0., 0., cos, -sin],
            [0., 0., sin,  cos],
        ])
    }
}

impl Deref for Matrix4x4 {
    type Target = [[f128; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[[f128; 4]; 4]> for Matrix4x4 {
    fn from(value: [[f128; 4]; 4]) -> Self {
        Matrix4x4(value)
    }
}

impl DerefMut for Matrix4x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//impl Mul<Point> for Matrix3x3 {
//    type Output = Point;
//
//    fn mul(self, rhs: Point) -> Self::Output {
//        let mut prod = Point::default();
//        for i in 0..9 {
//            prod[i / 3] += self[i / 3][i % 3] * rhs[i % 3];
//        }
//        prod
//    }
//}
impl Mul<Point> for Matrix4x4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let mut prod = Point::default();
        for i in 0..16 {
            prod[i / 4] += self[i / 4][i % 4] * rhs[i % 4];
        }
        prod
    }
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4x4([
            [
                self[0][0] + rhs[0][0],
                self[0][1] + rhs[0][1],
                self[0][2] + rhs[0][2],
                self[0][3] + rhs[0][3],
            ],
            [
                self[1][0] + rhs[1][0],
                self[1][1] + rhs[1][1],
                self[1][2] + rhs[1][2],
                self[1][3] + rhs[1][3],
            ],
            [
                self[2][0] + rhs[2][0],
                self[2][1] + rhs[2][1],
                self[2][2] + rhs[2][2],
                self[2][3] + rhs[2][3],
            ],
            [
                self[3][0] + rhs[3][0],
                self[3][1] + rhs[3][1],
                self[3][2] + rhs[3][2],
                self[3][3] + rhs[3][3],
            ],
        ])
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut prod = Matrix4x4::default();

        for i in 0..16 {
            let mut sum = 0.;
            for j in 0..4 {
                sum += self[i / 4][j] * rhs[j][i % 4];
            }
            prod[i / 4][i % 4] = sum;
        }

        prod
    }
}
