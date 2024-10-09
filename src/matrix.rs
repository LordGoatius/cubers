use std::ops::{Add, Deref, DerefMut, Mul};

use crate::point::Point;
pub mod fourd;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Matrix3x3(pub [[f128; 3]; 3]);

impl Matrix3x3 {
    /// Rotation matrix for rotation across the x axis. Theta is in radians
    pub fn rotate_x_theta(theta: f128) -> Matrix3x3 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix3x3([[1., 0., 0.], [0., cos, -sin], [0., sin, cos]])
    }
    
    /// Rotation matrix for rotation across the y axis. Theta is in radians
    pub fn rotate_y_theta(theta: f128) -> Matrix3x3 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix3x3([[cos, 0., sin], [0., 1., 0.], [-sin, 0., cos]])
    }

    /// Rotation matrix for rotation across the z axis. Theta is in radians
    pub fn rotate_z_theta(theta: f128) -> Matrix3x3 {
        let cos = theta.cos();
        let sin = theta.sin();
        Matrix3x3([[cos, -sin, 0.], [sin, cos, 0.], [0., 0., 1.]])
    }
}

impl Deref for Matrix3x3 {
    type Target = [[f128; 3]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[[f128; 3]; 3]> for Matrix3x3 {
    fn from(value: [[f128; 3]; 3]) -> Self {
        Matrix3x3(value)
    }
}

impl DerefMut for Matrix3x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix3x3(
            [[self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2]],
             [self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2]],
             [self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2]]]
        )
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut prod = Matrix3x3::default();

        for i in 0..9 {
            let mut sum = 0.;
            for j in 0..3 {
                sum += self[i / 3][j] * rhs[j][i % 3];
            }
            prod[i / 3][i % 3] = sum;
        }

        prod
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|row| row.map(|val| val * rhs as f128)).into()
    }
}

impl Mul<f64> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|row| row.map(|val| val * rhs as f128)).into()
    }
}

impl Mul<Point> for Matrix3x3 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let mut prod = Point::default();
        for i in 0..9 {
            prod[i / 3] += self[i / 3][i % 3] * rhs[i % 3];
        }
        prod
    }
}

impl Add<f32> for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, rhs: f32) -> Self::Output {
        self.map(|row| row.map(|val| val + rhs as f128)).into()
    }
}

impl Add<f64> for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, rhs: f64) -> Self::Output {
        self.map(|row| row.map(|val| val + rhs as f128)).into()
    }
}
