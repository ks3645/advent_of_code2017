use std::fs::File;
use std::io::Read;
use std::io;
use std::ops::Add;
use std::ops::AddAssign;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub enum Part {
    PartOne,
    PartTwo,
}

pub fn read_input_to_string(buf:&mut String, day:u8) -> io::Result<usize> {
    let filename = format!("input/day{}.txt", day);
    let mut file = File::open(filename).expect("Invalid Day Input");
    file.read_to_string(buf)
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    //Cartesian Coordinates
    pub fn n() -> Self{
        Point{x:0, y:1}
    }

    pub fn w() -> Self{
        Point{x:-1, y:0}
    }

    pub fn s() -> Self{
        Point{x:0, y:-1}
    }

    pub fn e() -> Self{
        Point{x:1, y:0}
    }

    //Screen Coordinates
    pub fn up() -> Self{
        Point{x:0, y:-1}
    }

    pub fn left() -> Self{
        Point{x:-1, y:0}
    }

    pub fn down() -> Self{
        Point{x:0, y:1}
    }

    pub fn right() -> Self{
        Point{x:1, y:0}
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other:Self) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Up,
    Down,
    Left,
}

pub fn get_neighbors(pos:Point) -> Vec<Point> {
    vec![pos+Point::n(), pos+Point::w(), pos+Point::s(), pos+Point::e()]
}

pub fn get_neighbors_diag(pos: Point) -> Vec<Point> {
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

pub fn knot_hash(key:String) -> String {
    const LIST_SIZE:usize = 256;
    const SALT:[u32;5] = [17, 31, 73, 47, 23];
    const ROUND_COUNT:u32 = 64;

    let mut list:Vec<u32> = (0..LIST_SIZE as u32).collect();
    let mut lengths:Vec<u32> = key.trim().bytes().map(|b| b as u32).collect();

    lengths.extend_from_slice(&SALT);

    let mut skip_size = 0;
    let mut pos: usize = 0;

    for _ in 0..ROUND_COUNT {
        for l in lengths.clone() {
            let mut sublist = Vec::new();
            for i in 0..l {
                sublist.push(list[(pos + i as usize) % LIST_SIZE]);
            }
            for (i, element) in sublist.iter().rev().enumerate() {
                list[(pos + i as usize) % LIST_SIZE] = element.clone();
            }
            pos += l as usize + skip_size as usize;
            skip_size += 1;
        }
    }

    let mut hash = String::new();
    for i in 0..16 {
        let mut num = list[16*i as usize];
        for j in 1..16 {
            num ^= list[16*i as usize + j as usize];
        }
        write!(&mut hash, "{:02x}", num).unwrap();
    }

    hash
}
