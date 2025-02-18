use std::ops::{Deref, DerefMut, Mul};

use itertools::Itertools;

use super::Shape as Shape3d;
use crate::{line::fourd::Line, point::fourd::Point};

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
        Shape(
            self.clone().iter().map(|line| *line * rhs).collect(),
            self.1,
        )
    }
}

impl Shape {
    /// Turns a 4d shape into a 3d shape
    pub fn to_shape_3d(&self) -> Shape3d {
        Shape3d(
            self.0
                .iter()
                .map(|line| line.to_line_3d())
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    /// Rotate shape around the zw plane theta radians
    pub fn rotate_zw_theta(self, theta: f128) -> Shape {
        Shape(
            self.0
                .iter()
                .map(|x| x.rotate_zw_theta(theta))
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    /// Rotate shape around the yw plane theta radians
    pub fn rotate_yw_theta(self, theta: f128) -> Shape {
        Shape(
            self.0
                .iter()
                .map(|x| x.rotate_yw_theta(theta))
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    /// Rotate shape around the yz plane theta radians
    pub fn rotate_yz_theta(self, theta: f128) -> Shape {
        Shape(
            self.0
                .iter()
                .map(|x| x.rotate_yz_theta(theta))
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    /// Rotate shape around the xw plane theta radians
    pub fn rotate_xw_theta(self, theta: f128) -> Shape {
        Shape(
            self.0
                .iter()
                .map(|x| x.rotate_xw_theta(theta))
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    /// Rotate shape around the xy plane theta radians
    pub fn rotate_xy_theta(self, theta: f128) -> Shape {
        Shape(
            self.0
                .iter()
                .map(|x| x.rotate_xy_theta(theta))
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    /// Rotate shape around the xz plane theta radians
    pub fn rotate_xz_theta(self, theta: f128) -> Shape {
        Shape(
            self.0
                .iter()
                .map(|x| x.rotate_xz_theta(theta))
                .collect::<Vec<_>>(),
            self.1,
        )
    }

    pub fn fivecell() -> Shape {
        let sqrt_5 = f128::sqrt(5.);
        let coords = vec![
            Point([ sqrt_5,  sqrt_5,  sqrt_5, -1.].map(|x| x * 0.25)),
            Point([ sqrt_5, -sqrt_5, -sqrt_5, -1.].map(|x| x * 0.25)),
            Point([-sqrt_5,  sqrt_5, -sqrt_5, -1.].map(|x| x * 0.25)),
            Point([-sqrt_5, -sqrt_5,  sqrt_5, -1.].map(|x| x * 0.25)),
            Point([0.,0.,0.,1.])
        ];
        let lines = coords.iter().combinations(2).map(|x| Line([*x[0], *x[1]]));
        Shape(lines.collect_vec(), '*')
    }

    pub fn hypercube() -> Shape {
        Shape(
            vec![
                // NEG ONE HYPERCUBE
                // Pos to self
                Line([Point([1., 1., 1., -1.]), Point([-1., 1., 1., -1.])]),
                Line([Point([-1., 1., 1., -1.]), Point([-1., -1., 1., -1.])]),
                Line([Point([-1., -1., 1., -1.]), Point([1., -1., 1., -1.])]),
                Line([Point([1., -1., 1., -1.]), Point([1., 1., 1., -1.])]),
                // Pos to neg
                Line([Point([1., 1., 1., -1.]), Point([1., 1., -1., -1.])]),
                Line([Point([-1., 1., 1., -1.]), Point([-1., 1., -1., -1.])]),
                Line([Point([-1., -1., 1., -1.]), Point([-1., -1., -1., -1.])]),
                Line([Point([1., -1., 1., -1.]), Point([1., -1., -1., -1.])]),
                // Neg to self
                Line([Point([1., 1., -1., -1.]), Point([-1., 1., -1., -1.])]),
                Line([Point([-1., 1., -1., -1.]), Point([-1., -1., -1., -1.])]),
                Line([Point([-1., -1., -1., -1.]), Point([1., -1., -1., -1.])]),
                Line([Point([1., -1., -1., -1.]), Point([1., 1., -1., -1.])]),
                // POS ONE HYPERCUBE
                // Pos to self
                Line([Point([1., 1., 1., 1.]), Point([-1., 1., 1., 1.])]),
                Line([Point([-1., 1., 1., 1.]), Point([-1., -1., 1., 1.])]),
                Line([Point([-1., -1., 1., 1.]), Point([1., -1., 1., 1.])]),
                Line([Point([1., -1., 1., 1.]), Point([1., 1., 1., 1.])]),
                // Pos to neg
                Line([Point([1., 1., 1., 1.]), Point([1., 1., -1., 1.])]),
                Line([Point([-1., 1., 1., 1.]), Point([-1., 1., -1., 1.])]),
                Line([Point([-1., -1., 1., 1.]), Point([-1., -1., -1., 1.])]),
                Line([Point([1., -1., 1., 1.]), Point([1., -1., -1., 1.])]),
                // Neg to self
                Line([Point([1., 1., -1., 1.]), Point([-1., 1., -1., 1.])]),
                Line([Point([-1., 1., -1., 1.]), Point([-1., -1., -1., 1.])]),
                Line([Point([-1., -1., -1., 1.]), Point([1., -1., -1., 1.])]),
                Line([Point([1., -1., -1., 1.]), Point([1., 1., -1., 1.])]),
                // 8 HYPERCUBE Connectors
                Line([Point([1., 1., 1., 1.]), Point([1., 1., 1., -1.])]),
                Line([Point([1., 1., -1., 1.]), Point([1., 1., -1., -1.])]),
                Line([Point([1., -1., 1., 1.]), Point([1., -1., 1., -1.])]),
                Line([Point([1., -1., -1., 1.]), Point([1., -1., -1., -1.])]),
                Line([Point([-1., 1., 1., 1.]), Point([-1., 1., 1., -1.])]),
                Line([Point([-1., 1., -1., 1.]), Point([-1., 1., -1., -1.])]),
                Line([Point([-1., -1., 1., 1.]), Point([-1., -1., 1., -1.])]),
                Line([Point([-1., -1., -1., 1.]), Point([-1., -1., -1., -1.])]),
            ],
            '*',
        )
    }
}
