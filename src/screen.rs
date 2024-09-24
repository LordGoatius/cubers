use std::{char, fmt::Display, thread::sleep, time::Duration};

use crate::{line::Line, point::Point, shape::Shape};

pub type ScreenBuffer = [[char; 78]; 42];
pub struct MyScreenBuffer(ScreenBuffer);

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    screen: ScreenBuffer,
}

impl Screen {
    pub fn init_render_cube(mut self) -> ! {
        let mut cube = Shape::cube() * 1.1;
        let mut small_cube = cube.clone() * 0.6;
        cube.1 = '.';
        small_cube.1 = '*';
        loop {
            self.render_shape(small_cube.clone());
            self.render_shape(cube.clone());
            self.print_screen();
            self.clear_screen();
            // small_cube = small_cube.clone().rotate_y_theta(std::f128::consts::PI / 90.);
            // small_cube = small_cube.clone().rotate_x_theta(std::f128::consts::PI / 70.);
            // small_cube = small_cube.clone().rotate_z_theta(std::f128::consts::PI / 180.);

            // cube = cube.clone().rotate_y_theta(std::f128::consts::PI / (2. * -90.));
            // cube = cube.clone().rotate_x_theta(std::f128::consts::PI / (2. * -70.));
            // cube = cube.clone().rotate_z_theta(std::f128::consts::PI / (2. * -180.));

            cube = cube.clone().rotate_y_theta(std::f128::consts::PI / 90.);
            cube = cube.clone().rotate_x_theta(std::f128::consts::PI / 70.);
            cube = cube.clone().rotate_z_theta(std::f128::consts::PI / 180.);

            small_cube = small_cube.clone().rotate_y_theta(std::f128::consts::PI / (2. * -90.));
            small_cube = small_cube.clone().rotate_x_theta(std::f128::consts::PI / (2. * -70.));
            small_cube = small_cube.clone().rotate_z_theta(std::f128::consts::PI / (2. * -180.));
            sleep(Duration::from_millis(50));
        }
    }

    fn render_shape(&mut self, shape: Shape) {
        shape.0.iter().for_each(|line| {
            self.render_line(*line, shape.1);
        });
    }

    fn render_line(&mut self, line: Line, char: char) {
        line.find_78_points_in_between()
            .iter()
            .for_each(|point| self.set_point(*point, char));
    }

    fn set_point(&mut self, coords: Point, char: char) {
        let (screen_x, screen_y) = coords.to_screen_xy();
        let (buff_coord_x, buff_coord_y) = (
            ((screen_x * 39. / 2.16666666666666 * (5. / 3.)) + 39.).round() as usize,
            ((screen_y * 21. / 1.16666666666666) + 21.).round() as usize,
            //((screen_x * 39. / 3.25 * (5. / 3.)) + 39.).round() as usize,
            //((screen_y * 21. / 1.75) + 21.).round() as usize,
        );
        self.screen[buff_coord_y][buff_coord_x] = char;
    }

    fn print_screen(&self) {
        println!("{}", MyScreenBuffer(self.screen));
    }

    fn clear_screen(&mut self) {
        self.screen = [[' '; 78]; 42];
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            screen: [[' '; 78]; 42],
        }
    }
}

impl Display for MyScreenBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display: String = String::new();

        self.0
            .iter()
            .for_each(|row| display = format!("{}{}\n", display, row.iter().collect::<String>()));
        write!(f, "{display}")
    }
}

#[cfg(test)]
pub mod test {
    use crate::{line::Line, point::Point, shape::Shape};

    use super::Screen;

    #[test]
    fn print_screen() {
        let mut scr = Screen::default();
        let mut cube: Shape = Shape::cube();

        scr.render_shape(cube.clone());
        scr.clear_screen();

        let cube = cube.rotate_y_theta(std::f128::consts::PI / 5.);

        scr.render_shape(cube.clone());
        scr.clear_screen();
    }

    #[test]
    fn line_in_between() {
        let line: Line = Line([Point([1., 1., 1.]), Point([1., 1., -1.])]);
        line.find_78_points_in_between()
            .iter()
            .for_each(|pt| println!("{pt}"));
    }
}
