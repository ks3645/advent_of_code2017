use utils;
use utils::Part;
use utils::Point;
use utils::Direction;
use std::collections::HashMap;
use std::fmt::Write;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 22).unwrap();

    process_infection(input, part)
}

fn process_infection(input:String, part:Part) -> i32 {
    let mut out = 0;

    let mut grid = HashMap::new();

    let mut lines:Vec<&str> = input.lines().map(|s| s.trim()).collect();

    let mid_point:i32 = (lines.len() / 2) as i32;

    for (row, line) in ((-mid_point-1)..mid_point+1).rev().zip(lines.iter()) {
        for (col, c) in ((-mid_point)..mid_point+2).zip(line.chars()) {
            grid.insert(Point{x:col, y:row}, c);

        }
    }

    let mut pos = Point{ x:0, y:0};
    let num_bursts = match part {
        Part::PartOne => 10_000,
        Part::PartTwo => 10_000_000,
    };
    let mut infections = 0;

    let mut facing = Direction::Up;

    for _ in 0..num_bursts {
        let node = grid.entry(pos).or_insert('.');
        match *node {
            '.' => { //turn left
                facing = match facing {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                };
                match part {
                    Part::PartOne => {
                        *node = '#';
                        infections += 1;
                    },
                    Part::PartTwo => {
                        *node = 'W';
                    },
                }
            },
            '#' => { //turn right
                facing = match facing {
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Down,
                };
                match part {
                    Part::PartOne => {
                        *node = '.';
                    }
                    Part::PartTwo => {
                        *node = 'F';
                    }
                }
            },
            'W' => { //no turn
                *node = '#';
                infections += 1;
            },
            'F' => { //reverse direction
                facing = match facing {
                    Direction::Up => Direction::Down,
                    Direction::Left => Direction::Right,
                    Direction::Down => Direction::Up,
                    Direction::Right => Direction::Left,
                };
                *node = '.';
            },

            _ => panic!("Unknown Node Value"),
        }
        match facing {
            Direction::Up => pos += Point::n(),
            Direction::Left => pos += Point::w(),
            Direction::Down => pos += Point::s(),
            Direction::Right => pos += Point::e(),
        }
    }

    out = infections;

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = String::from("..#
#..
...");
        assert_eq!(process_infection(test_input, Part::PartOne), 5587)
    }

    #[test]
    fn test_part_two() {
        let test_input = String::from("..#
#..
...");
        assert_eq!(process_infection(test_input, Part::PartTwo), 2511944)
    }
}