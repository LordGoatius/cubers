use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::{char, fmt::Display, thread::sleep, time::Duration};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::shape::fourd::Shape as Shape4d;
use crate::{line::Line, point::Point, shape::Shape};

pub type ScreenBuffer = [[char; 78]; 42];
pub struct MyScreenBuffer(ScreenBuffer);

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    screen: ScreenBuffer,
}

impl Screen {
    pub fn init_render_fivecell(mut self) -> ! {
        let mut fivecell = Shape4d::fivecell() * 3.5;
        fivecell.1 = '.';
        //let mut fivecell2 = Shape4d::fivecell();
        //fivecell = fivecell.clone().rotate_xy_theta(std::f128::consts::PI / 6.);
        loop {
            let cube = fivecell.to_shape_3d();
            self.render_shape(cube);
            //let cube2 = fivecell2.to_shape_3d();
            //self.render_shape(cube2);
            self.print_screen();
            self.clear_screen();

            //fivecell = fivecell.rotate_xw_theta(std::f128::consts::PI / 70.);
            //fivecell = fivecell.rotate_yz_theta(std::f128::consts::PI / 90.);

            //fivecell = fivecell.rotate_yw_theta(std::f128::consts::PI / 70.);
            //fivecell = fivecell.rotate_xz_theta(std::f128::consts::PI / 90.);

            fivecell = fivecell.rotate_zw_theta(std::f128::consts::PI / 90.);
            //fivecell = fivecell.clone().rotate_xz_theta(std::f128::consts::PI / 90.);
            fivecell = fivecell.rotate_xy_theta(std::f128::consts::PI / 90.);

            //fivecell2 = fivecell2.rotate_zw_theta(std::f128::consts::PI / -90.);
            //fivecell2 = fivecell2.rotate_xy_theta(std::f128::consts::PI / -90.);

            sleep(Duration::from_millis(50));
        }
    }

    pub fn init_render_hypercube_manual(mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        ).unwrap();
        stdout.flush().unwrap();

        let mut hypercube = Shape4d::hypercube() * 1.5;
        hypercube.1 = '.';

        let ref_stdout = &mut stdout;

        let (tx, rx) = std::sync::mpmc::channel::<Option<Key>>();

        thread::scope(|s| {
            s.spawn(|| {
                for k in stdin.keys() {
                    match k.as_ref().unwrap() {
                        Key::Char('q') | Key::Ctrl('c') => {
                            tx.send(Some(Key::Char('q'))).unwrap();
                            break;
                        }
                        key @ Key::Char('h' | 'j' | 'k' | 'l' | 'i' | 'n') => {
                            tx.send(Some(*key)).unwrap()
                        }
                        _ => (),
                    }
                }
            });
            s.spawn(move || {
                enum Rotate {
                    ZW,
                    YW,
                    YZ,
                    XW,
                    XZ,
                    XY,
                }
                let cube = hypercube.to_shape_3d();
                self.render_shape(cube);
                self.print_screen_raw(ref_stdout);
                self.clear_screen();

                loop {
                    let message = match rx.try_recv() {
                        Err(TryRecvError::Disconnected) => break,
                        Ok(Some(Key::Char('q'))) => {
                            break;
                        },
                        Ok(Some(Key::Char(char @ ('h' | 'j' | 'k' | 'l' | 'i' | 'n')))) => {
                            match char {
                                'h' => Some(Rotate::XY),
                                'j' => Some(Rotate::XZ),
                                'k' => Some(Rotate::YZ),
                                'l' => Some(Rotate::ZW),
                                'i' => Some(Rotate::YW),
                                'n' => Some(Rotate::XW),
                                _ => todo!(),
                            }
                        }
                        Ok(None) => None,
                        _ => None,
                    };

                    if message.is_none() {
                        continue;
                    }

                    let cube = hypercube.to_shape_3d();
                    self.render_shape(cube);
                    self.print_screen_raw(ref_stdout);
                    self.clear_screen();


                    //hypercube = hypercube.rotate_zw_theta(std::f128::consts::PI / 90.);
                    //hypercube = hypercube.clone().rotate_xz_theta(std::f128::consts::PI / 90.);
                    //hypercube = hypercube.rotate_xy_theta(std::f128::consts::PI / 90.);

                    match message.unwrap() {
                        Rotate::ZW => {
                            hypercube = hypercube.rotate_zw_theta(std::f128::consts::PI / 12.);
                        }
                        Rotate::YW => {
                            hypercube = hypercube.rotate_yw_theta(std::f128::consts::PI / 12.);
                        }
                        Rotate::YZ => {
                            hypercube = hypercube.rotate_yz_theta(std::f128::consts::PI / 12.);
                        }
                        Rotate::XW => {
                            hypercube = hypercube.rotate_xw_theta(std::f128::consts::PI / 12.);
                        }
                        Rotate::XZ => {
                            hypercube = hypercube.rotate_xz_theta(std::f128::consts::PI / 12.);
                        }
                        Rotate::XY => {
                            hypercube = hypercube.rotate_xy_theta(std::f128::consts::PI / 12.);
                        }
                    }

                    sleep(Duration::from_millis(50));
                }
            });
        });

        write!(stdout, "{}", termion::cursor::Show).unwrap();
        println!("Exit\n");
    }

    pub fn init_render_hypercube(mut self) -> ! {
        let mut hypercube = Shape4d::hypercube() * 1.5;
        hypercube.1 = '.';
        //let mut hypercube2 = Shape4d::hypercube();
        //hypercube = hypercube.clone().rotate_xy_theta(std::f128::consts::PI / 6.);
        loop {
            let cube = hypercube.to_shape_3d();
            self.render_shape(cube);
            //let cube2 = hypercube2.to_shape_3d();
            //self.render_shape(cube2);
            self.print_screen();
            self.clear_screen();

            //hypercube = hypercube.rotate_xw_theta(std::f128::consts::PI / 70.);
            //hypercube = hypercube.rotate_yz_theta(std::f128::consts::PI / 90.);

            //hypercube = hypercube.rotate_yw_theta(std::f128::consts::PI / 70.);
            //hypercube = hypercube.rotate_xz_theta(std::f128::consts::PI / 90.);

            hypercube = hypercube.rotate_zw_theta(std::f128::consts::PI / 90.);
            //hypercube = hypercube.clone().rotate_xz_theta(std::f128::consts::PI / 90.);
            hypercube = hypercube.rotate_xy_theta(std::f128::consts::PI / 90.);

            //hypercube2 = hypercube2.rotate_zw_theta(std::f128::consts::PI / -90.);
            //hypercube2 = hypercube2.rotate_xy_theta(std::f128::consts::PI / -90.);

            sleep(Duration::from_millis(50));
        }
    }

    pub fn init_render_cube(mut self) -> ! {
        let mut cube = Shape::cube() * 1.1;
        //let mut small_cube = cube.clone() * 0.6;
        cube.1 = '.';
        //small_cube.1 = '*';
        loop {
            //self.render_shape(small_cube.clone());
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

            //small_cube = small_cube.clone().rotate_y_theta(std::f128::consts::PI / (2. * -90.));
            //small_cube = small_cube.clone().rotate_x_theta(std::f128::consts::PI / (2. * -70.));
            //small_cube = small_cube.clone().rotate_z_theta(std::f128::consts::PI / (2. * -180.));
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
        if let Some(x) = self.screen.get_mut(buff_coord_y) {
            if let Some(coord) = x.get_mut(buff_coord_x) {
                *coord = char;
            }
        }
        //self.screen[buff_coord_y][buff_coord_x] = char;
    }

    fn print_screen(&self) {
        println!("{}", MyScreenBuffer(self.screen));
    }

    fn print_screen_raw(&self, stdout: &mut Stdout) {
        for i in self.screen {
            write!(stdout, "{}\r\n", i.into_iter().collect::<String>()).unwrap();
        }
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
