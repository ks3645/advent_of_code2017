use std::fs::File;
use std::io::Read;
use std::io;
use std::ops::AddAssign;

pub enum Part {
    PartOne,
    PartTwo,
}

pub fn read_input_to_string(buf:&mut String, day:u8) -> io::Result<usize> {
    let filename = format!("day{}.txt", day);
    let mut file = File::open(filename).expect("Invalid Day Input");
    file.read_to_string(buf)
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Up,
    Down,
    Left,
}

pub fn get_neighbors(pos: Point) -> Vec<Point> {
    let mut neighbors = Vec::with_capacity(8);

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 { continue; }

            neighbors.push(Point { x: pos.x + i, y: pos.y + j });
        }
    }

    neighbors
}

#[derive(Debug, Clone, Copy)]
pub struct HexTile {
    pub r:i32,
    pub s:i32,
    pub t:i32,
}

impl HexTile {
    pub fn n() -> Self {
        HexTile{ r:1, s:0, t:-1}
    }

    pub fn nw() -> Self {
        HexTile{ r:0, s:1, t:-1}
    }

    pub fn sw() -> Self {
        HexTile{ r:-1, s:1, t:0}
    }

    pub fn s() -> Self {
        HexTile{ r:-1, s:0, t:1}
    }

    pub fn se() -> Self {
        HexTile{ r:0, s:-1, t:1}
    }

    pub fn ne() -> Self {
        HexTile{ r:1, s:-1, t:0}
    }

    pub fn distance(self, other:Self) -> i32 {
        ((self.r - other.r).abs() + (self.s - other.s).abs() + (self.t - other.t).abs())/2
    }
}

impl AddAssign for HexTile {
    fn add_assign(&mut self, other:Self) {
        *self = HexTile {
            r: self.r + other.r,
            s: self.s + other.s,
            t: self.t + other.t,
        };
    }
}
