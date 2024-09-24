use std::ops::{Deref, DerefMut, Mul};

use crate::{line::Line, point::Point};

#[derive(Debug, Default, Clone)]
pub struct Shape(pub Vec<Line>, pub char);

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

impl Mul<f128> for Shape {
    type Output = Shape;

    fn mul(self, rhs: f128) -> Self::Output {
        Shape(self.clone().iter().map(|line| *line * rhs).collect(), self.1)
    }
}

impl Shape {
    /// Rotate shape around the x axis theta radians
    pub fn rotate_x_theta(self, theta: f128) -> Shape {
        Shape(self.0.iter().map(|x| x.rotate_x_theta(theta)).collect::<Vec<_>>(), self.1)
    }

    /// Rotate shape around the y axis theta radians
    pub fn rotate_y_theta(&mut self, theta: f128) -> Shape {
        Shape(self.0.iter().map(|x| x.rotate_y_theta(theta)).collect::<Vec<_>>(), self.1)
    }
    
    /// Rotate shape around the z axis theta radians
    pub fn rotate_z_theta(&mut self, theta: f128) -> Shape {
        Shape(self.0.iter().map(|x| x.rotate_z_theta(theta)).collect::<Vec<_>>(), self.1)
    }

    pub fn cube() -> Shape {
        Shape(vec![
            // Pos to self
            Line([Point([ 1.,  1., 1.]), Point([-1.,  1., 1.])]),
            Line([Point([-1.,  1., 1.]), Point([-1., -1., 1.])]),
            Line([Point([-1., -1., 1.]), Point([ 1., -1., 1.])]),
            Line([Point([ 1., -1., 1.]), Point([ 1.,  1., 1.])]),

            // Pos to neg
            Line([Point([ 1.,  1., 1.]), Point([ 1.,  1., -1.])]),
            Line([Point([-1.,  1., 1.]), Point([-1.,  1., -1.])]),
            Line([Point([-1., -1., 1.]), Point([-1., -1., -1.])]),
            Line([Point([ 1., -1., 1.]), Point([ 1., -1., -1.])]),

            // Neg to self
            Line([Point([ 1.,  1., -1.]), Point([-1.,  1., -1.])]),
            Line([Point([-1.,  1., -1.]), Point([-1., -1., -1.])]),
            Line([Point([-1., -1., -1.]), Point([ 1., -1., -1.])]),
            Line([Point([ 1., -1., -1.]), Point([ 1.,  1., -1.])]),
        ], 
        '*')
    }
}
