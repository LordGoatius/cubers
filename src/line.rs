use std::ops::{Deref, DerefMut};

use crate::point::Point;

#[derive(Debug, Default, Clone, Copy)]
pub struct Line([Point; 2]);

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
}
