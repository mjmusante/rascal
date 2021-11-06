use crate::gui::Gui;
use bracket_terminal::prelude::*;

pub struct Map {
    layout: Vec<Tile>,
    revealed: Vec<bool>,
    visible: Vec<Point>,
    width: usize,
    height: usize,
    upstairs: Point,
    map_offset_x: i32,
    map_offset_y: i32,
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
            visible: Vec::new(),
            upstairs: Point { x: 0, y: 0 },
            map_offset_x: 0,
            map_offset_y: 0,
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

        self.visible.clear();
        for row in top..(top + radius) {
            for col in left..(left + radius) {
                let loc = row * self.width + col;
                self.revealed[loc] = true;
                self.visible.push(Point {
                    x: col as i32,
                    y: row as i32,
                });
            }
        }
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        self.visible.contains(&Point { x, y })
    }

    pub fn draw(&mut self, gui: &mut Gui, xcenter: usize, ycenter: usize) {
        let (xleft, ytop, width, height) = gui.map_window();

        let midx = width / 2;
        let midy = height / 2;

        let offsetx = if xcenter < midx {
            0
        } else if xcenter >= self.width - midx {
            self.width - width
        } else {
            xcenter - midx
        };

        let offsety = if ycenter < midy {
            0
        } else if ycenter >= self.height - midy {
            self.height - height
        } else {
            ycenter - midy
        };

        for row in 0..height {
            for col in 0..width {
                let maploc = (row + offsety) * self.width + col + offsetx;

                if self.revealed[maploc] {
                    gui.set(col + xleft, row + ytop, self.layout[maploc].glyph());
                } else {
                    gui.set(col + xleft, row + ytop, 32);
                }
            }
        }

        self.map_offset_x = xleft as i32 - offsetx as i32;
        self.map_offset_y = ytop as i32 - offsety as i32;
    }

    pub fn get_offset(&self) -> (i32, i32) {
        (self.map_offset_x as i32, self.map_offset_y as i32)
    }
}
