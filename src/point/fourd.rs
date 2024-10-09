use std::{
    f128,
    fmt::Display,
    ops::{Add, Deref, DerefMut, Mul},
};

use crate::matrix::fourd::Matrix4x4;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point(pub [f128; 4]);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point {{\nx: {:.32?},\ny: {:.32?},\nx: {:.32?},\nw: {:.32?}\n}}",
            self[0] as f64, self[1] as f64, self[2] as f64, self[3] as f64
        )
    }
}

impl Deref for Point {
    type Target = [f128; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Self) -> Self::Output {
        Point([
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        ])
    }
}

impl Mul<f128> for Point {
    type Output = Point;

    fn mul(self, rhs: f128) -> Self::Output {
        Point(self.map(|x| x * rhs))
    }
}

impl Point {
    pub fn l1_norm(&self) -> f128 {
        self[0].abs() + self[1].abs() + self[2].abs()
    }

    pub fn l2_norm(&self) -> f128 {
        (self[0].powi(2) + self[1].powi(2) + self[2].powi(2)).sqrt()
    }

    /// Rotate point around the zw plane theta radians
    pub fn rotate_zw_theta(self, theta: f128) -> Point {
        Matrix4x4::rotate_zw_theta(theta) * self
    }

    /// Rotate point around the yw plane theta radians
    pub fn rotate_yw_theta(self, theta: f128) -> Point {
        Matrix4x4::rotate_yw_theta(theta) * self
    }

    /// Rotate point around the yz plane theta radians
    pub fn rotate_yz_theta(self, theta: f128) -> Point {
        Matrix4x4::rotate_yz_theta(theta) * self
    }

    /// Rotate point around the xw plane theta radians
    pub fn rotate_xw_theta(self, theta: f128) -> Point {
        Matrix4x4::rotate_xw_theta(theta) * self
    }

    /// Rotate point around the xy plane theta radians
    pub fn rotate_xy_theta(self, theta: f128) -> Point {
        Matrix4x4::rotate_xy_theta(theta) * self
    }

    /// Rotate point around the xz plane theta radians
    pub fn rotate_xz_theta(self, theta: f128) -> Point {
        Matrix4x4::rotate_xz_theta(theta) * self
    }

    // NOTE: Maybe?
    pub fn to_cube_xyz(&self) -> (f128, f128, f128) {
        (self[0] * 2. / (4. + self[3]), 
         self[1] * 2. / (4. + self[3]),
         self[2] * 2. / (4. + self[3]))
    }

    // NOTE: Maybe?
    pub fn to_cube_xyz_2(&self) -> (f128, f128, f128) {
        (self[0] / self[3], 
         self[1] / self[3],
         self[2] / self[3])
    }
}
