#![feature(f128)]
pub mod matrix;
pub mod point;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test {
    use std::f128;
    use crate::{
        matrix::{Matrix, Matrix3x3},
        point::Point,
    };

    #[test]
    fn point() {
        let point = Point([3., 4., 0.]);
        assert_eq!(point.l2_norm(), 5.);
        let point = Point([1., 2., 2.]);
        assert_eq!(point.l2_norm(), 3.);
    }

    #[test]
    fn matmul() {
        let matrix0: Matrix = Matrix3x3([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let matrix1: Matrix = Matrix3x3([[1., 2., 3.], [3., 1., 2.], [2., 3., 1.]]);

        let swap23:  Matrix = Matrix3x3([[1., 0., 0.], [0., 0., 1.], [0., 1., 0.]]);

        let matrix2: Matrix = Matrix3x3([[1., 2., 3.], [2., 3., 1.], [3., 1., 2.]]);
        let matrix3: Matrix = Matrix3x3([[1., 3., 2.], [2., 1., 3.], [3., 2., 1.]]);
        let matrix4: Matrix = Matrix3x3([[1., 3., 2.], [3., 2., 1.], [2., 1., 3.]]);

        assert_eq!(matrix1 * matrix0, matrix1);
        assert_eq!(matrix0 * matrix1, matrix1);
        assert_eq!(swap23 * matrix1, matrix2);

        assert_eq!(matrix1 * swap23, matrix4);

        assert_eq!(matrix2 * swap23, matrix3);
    }

    #[test]
    fn point_matmul() {
        let pi = f128::consts::PI;
        let point = Point([1., 1., 0.]);
        let xrotate = Matrix3x3::rotate_x_theta(pi/2.);
        println!("{}", point);
        println!("{}", xrotate * point);
    }
}
