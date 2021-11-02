use crate::gui::Gui;
use bracket_terminal::prelude::*;
use std::cmp::{max, min};

pub struct Map {
    layout: Vec<Tile>,
    revealed: Vec<bool>,
    width: usize,
    height: usize,
    upstairs: Point,
    centerx: usize,
    centery: usize,
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
            centerx: 0,
            centery: 0,
        };

        map.generate_level();
        map
    }

    fn generate_level(&mut self) {
        for row in 10..15 {
            for col in 10..28 {
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
        self.centerx = min(max(xcenter, midx), self.width - midx);
        self.centery = min(max(ycenter, midy), self.height - midy);

        for row in 0..Gui::ROWS {
            for col in 0..Gui::COLS {
                let maploc = (row + self.centery - midy) * self.width + col + self.centerx - midx;
                if self.revealed[maploc] {
                    gui.set(col, row, self.layout[maploc].glyph());
                } else {
                    gui.set(col, row, 32);
                }
            }
        }
    }

    pub fn put_char(&self, gui: &mut Gui, loc: Point, val: u8) {
        let midx = Gui::COLS / 2;
        let midy = Gui::ROWS / 2;
        let xloc = if self.centerx > loc.x as usize {
            midx - (self.centerx - loc.x as usize)
        } else {
            midx + (loc.x as usize - self.centerx)
        };
        let yloc = if self.centery > loc.y as usize {
            midy - (self.centery - loc.y as usize)
        } else {
            midy + (loc.y as usize - self.centery)
        };
        gui.set(xloc, yloc, val);
    }
}
