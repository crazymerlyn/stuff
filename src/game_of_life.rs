use std::env;
use std::fmt;
use std::str;
use std::io::Write;

extern crate rand;
use rand::Rng;

static SCALE: usize = 6;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        writeln!(std::io::stderr(), "Three arguments required").unwrap();
        std::process::exit(1);
    }

    let w: usize = args[1].parse().unwrap();
    let h: usize = args[2].parse().unwrap();
    let it: usize = args[3].parse().unwrap();

    let mut grid = Grid::new(w, h);
    let mut stdout = std::io::stdout();
    grid.print(&mut stdout);

    for _ in 0..it {
        grid = grid.step();
        grid.print(&mut stdout);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    DEAD,
    RED,
    BLUE,
}
use State::*;

pub struct Grid {
    grid: Vec<State>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut grid = vec![DEAD; width * height];
        let mut rng = rand::thread_rng();

        for i in 0..height {
            for j in 0..width {
                grid[i * width + j] = match rng.gen::<u8>() % 4 {
                    0 | 1 => DEAD,
                    2 => RED,
                    3 => BLUE,
                    _ => unreachable!(),
                }
            }
        }

        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> State {
        self.grid[i * self.width + j]
    }

    pub fn get_surrounding(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let i_l = if i == 0 { self.height - 1 } else { i - 1 };
        let j_l = if j == 0 { self.width - 1 } else { j - 1 };
        let i_u = (i + 1) % self.height;
        let j_u = (j + 1) % self.width;

        for i in [i_l, i, i_u].iter() {
            for j in [j_l, j, j_u].iter() {
                res.push((*i, *j));
            }
        }

        res
    }

    pub fn step(&self) -> Grid {
        let mut grid = vec![DEAD; self.width * self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                let mut red = 0;
                let mut blue = 0;
                let index = i * self.width + j;

                for (i_s, j_s) in self.get_surrounding(i, j) {
                    match self.get(i_s, j_s) {
                        RED => red += 1,
                        BLUE => blue += 1,
                        _ => {}
                    }
                }
                let total = red + blue;
                match self.get(i, j) {
                    DEAD => {
                        if total == 3 {
                            grid[index] = if red > blue { RED } else { BLUE };
                        }
                    }
                    RED => {
                        if red < blue {
                            grid[index] = BLUE;
                        } else {
                            if total >= 3 && total <= 4 {
                                grid[index] = RED;
                            }
                        }
                    }
                    BLUE => {
                        if red > blue {
                            grid[index] = RED;
                        } else {
                            if total >= 3 && total <= 4 {
                                grid[index] = BLUE;
                            }
                        }
                    }
                }
            }
        }
        Grid {
            grid: grid,
            height: self.height,
            width: self.width,
        }
    } 

    pub fn print<W: Write>(&self, w: &mut W) {
        writeln!(w, "P6 {} {} 255", self.width * SCALE, self.height * SCALE).unwrap();
        for i in 0..self.height*SCALE {
            for j in 0..self.width*SCALE {
                match self.get(i / SCALE, j / SCALE) {
                    DEAD => w.write(&[0, 0, 0]),
                    RED => w.write(&[255, 0, 0]),
                    BLUE => w.write(&[0, 0, 255]),
                };
            }
        }
    }
}

