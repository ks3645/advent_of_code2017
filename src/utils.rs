use std::fs::File;
use std::io::Read;
use std::io;

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

pub struct HexTile {
    r:i32,
    s:i32,
    t:i32,
}
