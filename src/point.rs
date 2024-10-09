use std::{f128, fmt::Display, ops::{Add, Deref, DerefMut, Mul}};

use crate::matrix::Matrix3x3;

pub mod fourd;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point(pub [f128; 3]);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {{\nx: {:.32?},\ny: {:.32?},\nx: {:.32?}\n}}", self[0] as f64, self[1] as f64, self[2] as f64)
    }
}

impl Deref for Point {
    type Target = [f128; 3];
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
        Point([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Self) -> Self::Output {
        Point([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
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

    /// Rotate point around the x axis theta radians
    pub fn rotate_x_theta(self, theta: f128) -> Point {
        Matrix3x3::rotate_x_theta(theta) * self
    }

    /// Rotate point around the y axis theta radians
    pub fn rotate_y_theta(self, theta: f128) -> Point {
        Matrix3x3::rotate_y_theta(theta) * self
    }
    
    /// Rotate point around the z axis theta radians
    pub fn rotate_z_theta(self, theta: f128) -> Point {
        Matrix3x3::rotate_z_theta(theta) * self
    }

    pub fn to_screen_xy(&self) -> (f128, f128) {
        (self[0] * 2. / (4. + self[1]), self[2] * 2. / (4. + self[1]))
    }
}

