use std::ops::{Deref, DerefMut};

use crate::line::Line;

#[derive(Debug, Default, Clone)]
pub struct Shape(Vec<Line>);

impl Deref for Shape {
    type Target = Vec<Line>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Shape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Shape {
    /// Rotate shape around the x axis theta radians
    pub fn rotate_x_theta(&mut self, theta: f128) {
        let _ = self.0.iter_mut().map(|x| x.rotate_x_theta(theta)).collect::<Vec<_>>();
    }

    /// Rotate shape around the y axis theta radians
    pub fn rotate_y_theta(&mut self, theta: f128) {
        let _ = self.0.iter_mut().map(|x| x.rotate_y_theta(theta)).collect::<Vec<_>>();
    }
    
    /// Rotate shape around the z axis theta radians
    pub fn rotate_z_theta(&mut self, theta: f128) {
        let _ = self.0.iter_mut().map(|x| x.rotate_z_theta(theta)).collect::<Vec<_>>();
    }
}


