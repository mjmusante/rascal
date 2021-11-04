use crate::gui::Gui;
use bracket_terminal::prelude::*;
use std::cmp::{max, min};

pub struct Map {
    layout: Vec<Tile>,
    revealed: Vec<bool>,
    width: usize,
    height: usize,
    upstairs: Point,
    offsetx: usize,
    offsety: usize,
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Stone,
    Open,
}

impl Tile {
    pub fn glyph(&self) -> u8 {
        match self {
            Tile::Stone => 35,
            Tile::Open => 46,
        }
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut map = Map {
            width,
            height,
            layout: vec![Tile::Stone; width * height],
            revealed: vec![false; width * height],
            upstairs: Point { x: 0, y: 0 },
            offsetx: 0,
            offsety: 0,
        };

        map.generate_level();
        map
    }

    fn generate_level(&mut self) {
        for row in 2..29 {
            for col in 2..48 {
                let loc = row * self.width + col;
                self.layout[loc] = Tile::Open;
            }
        }
        self.upstairs = Point { x: 12, y: 12 };
    }

    pub fn get_upstairs(&self) -> Point {
        self.upstairs
    }

    pub fn can_go(&self, x: i32, y: i32) -> bool {
        let loc = y as usize * self.width + x as usize;
        self.layout[loc] == Tile::Open
    }

    pub fn reveal(&mut self, loc: Point, radius: usize) {
        let offset = radius / 2;
        let top: usize = if offset > loc.y as usize {
            0
        } else {
            loc.y as usize - offset
        };
        let left: usize = if offset > loc.x as usize {
            0
        } else {
            loc.x as usize - offset
        };

        for row in top..(top + radius) {
            for col in left..(left + radius) {
                let loc = row * self.width + col;
                self.revealed[loc] = true;
            }
        }
    }

    pub fn draw(&mut self, gui: &mut Gui, xcenter: usize, ycenter: usize) {
        let midx = Gui::COLS / 2;
        let midy = Gui::ROWS / 2;

        self.offsetx = if xcenter < midx {
            0
        } else if xcenter >= self.width - midx {
            self.width - Gui::COLS
        } else {
            xcenter - midx
        };

        self.offsety = if ycenter < midy {
            0
        } else if ycenter >= self.height - midy {
            self.height - Gui::ROWS
        } else {
            ycenter - midy
        };

        for row in 0..Gui::ROWS {
            for col in 0..Gui::COLS {
                let maploc = (row + self.offsety) * self.width + col + self.offsetx;

                if self.revealed[maploc] {
                    gui.set(col, row, self.layout[maploc].glyph());
                } else {
                    gui.set(col, row, 32);
                }
            }
        }
    }

    pub fn get_offset(&self) -> (i32, i32) {
        (self.offsetx as i32, self.offsety as i32)
    }
}
