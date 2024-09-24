use std::ops::{Deref, DerefMut, Mul};

use crate::point::Point;

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
    /// Rotate line around the x axis theta radians
    pub fn rotate_x_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_x_theta(theta)))
    }

    /// Rotate line around the y axis theta radians
    pub fn rotate_y_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_y_theta(theta)))
    }

    /// Rotate line around the z axis theta radians
    pub fn rotate_z_theta(self, theta: f128) -> Line {
        Line(self.map(|x| x.rotate_z_theta(theta)))
    }

    pub fn find_78_points_in_between(&self) -> Vec<Point> {
        let delta_x = self[0][0] - self[1][0];
        let delta_y = self[0][1] - self[1][1];
        let delta_z = self[0][2] - self[1][2];
        (0..=78).map(|num| {
            Point([
                self[0][0] - (num as f128 * delta_x / 78.),
                self[0][1] - (num as f128 * delta_y / 78.),
                self[0][2] - (num as f128 * delta_z / 78.),
            ])
        }).collect()
    }
}
