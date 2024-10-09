use std::ops::{Deref, DerefMut, Mul};

use super::Line as Line3d;
use super::Point as Point3d;
use crate::point::fourd::Point;

#[derive(Debug, Default, Clone, Copy)]
pub struct Line(pub [Point; 2]);

impl Deref for Line {
    type Target = [Point; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Mul<f128> for Line {
    type Output = Line;

    fn mul(self, rhs: f128) -> Self::Output {
        Line(self.map(|pt| pt * rhs))
    }
}

impl Line {
    /// Turns 4d line into 3d line
    pub fn to_line_3d(&self) -> Line3d {
        Line3d(self.map(|point| Point3d(point.to_cube_xyz().into())))
    }

    /// Rotate line around the zw plane theta radians
    pub fn rotate_zw_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_zw_theta(theta)))
    }

    /// Rotate line around the yw plane theta radians
    pub fn rotate_yw_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_yw_theta(theta)))
    }

    /// Rotate line around the yz plane theta radians
    pub fn rotate_yz_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_yz_theta(theta)))
    }

    /// Rotate line around the xw plane theta radians
    pub fn rotate_xw_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_xw_theta(theta)))
    }

    /// Rotate line around the xy plane theta radians
    pub fn rotate_xy_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_xy_theta(theta)))
    }

    /// Rotate line around the xz plane theta radians
    pub fn rotate_xz_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_xz_theta(theta)))
    }
}
