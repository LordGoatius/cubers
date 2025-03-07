#![feature(f128, mpmc_channel)]

use std::{io::{self, Write}, process::exit};

use screen::Screen;
pub mod matrix;
pub mod point;
pub mod line;
pub mod screen;
pub mod shape;

fn main() -> ! {
    let mut buffer =String::new();
    let mut choice = 0;
    loop {
        print!("Pick 1 for cube, 2 for fivecell, 3 for hypercube, and 4 for controllable hypercube: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "1" => {
                choice = 1;
                break;
            },
            "2" => {
                choice = 2;
                break;
            },
            "3" => {
                choice = 3;
                break;
            },
            "4" => {
                choice = 4;
                break;
            },
            _ => continue,
        };
    }
    loop {
        buffer.clear();
        print!("1 for large terminal, 2 for small (if the terminal is too small, it may not display the large screen correctly): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "1" => {
                let screen: Screen<257, 171> = Screen::default();
                match choice {
                    1 => screen.init_render_cube(),
                    2 => screen.init_render_fivecell(),
                    3 => screen.init_render_hypercube(),
                    4 => screen.init_render_hypercube_manual(),
                    _ => unreachable!()
                }
                exit(0);
            },
            "2" => {
                let screen: Screen<78, 42> = Screen::default();
                match choice {
                    1 => screen.init_render_cube(),
                    2 => screen.init_render_fivecell(),
                    3 => screen.init_render_hypercube(),
                    4 => screen.init_render_hypercube_manual(),
                    _ => unreachable!()
                }
                exit(0);
            }
            _ => continue
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::f128;
    use crate::{
        matrix::Matrix3x3,
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
        let matrix0: Matrix3x3 = Matrix3x3([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let matrix1: Matrix3x3 = Matrix3x3([[1., 2., 3.], [3., 1., 2.], [2., 3., 1.]]);

        let swap23:  Matrix3x3 = Matrix3x3([[1., 0., 0.], [0., 0., 1.], [0., 1., 0.]]);

        let matrix2: Matrix3x3 = Matrix3x3([[1., 2., 3.], [2., 3., 1.], [3., 1., 2.]]);
        let matrix3: Matrix3x3 = Matrix3x3([[1., 3., 2.], [2., 1., 3.], [3., 2., 1.]]);
        let matrix4: Matrix3x3 = Matrix3x3([[1., 3., 2.], [3., 2., 1.], [2., 1., 3.]]);

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
