use utils;
use utils::Part;
use utils::Point;
use utils::Direction;
use std::fmt::Write;

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 19).unwrap();

    let mut out = String::new();

    let mut path = Vec::new();

    for line in input.lines() {
        let row:Vec<char> = line.replace('\n', "").chars().collect();
        path.push(row);
    }

    let mut pos = Point {x:0, y:0};


    loop {
        pos.x += 1;
        match path[pos.y as usize][pos.x as usize] {
            '|' => break,
            _ => {},
        }
    }

    let mut travel_dir = Direction::Down;
    let mut tokens_visited = Vec::new();
    let mut steps = 0;

    loop {
        if pos.x < 0 || pos.x >= path[0].len() as i32
            || pos.y < 0 || pos.y >= path.len() as i32 {
            break;
        }

        match path[pos.y as usize][pos.x as usize] {
            '|' | '-' => {},
            ' ' => break,
            '+' => {
                match travel_dir {
                    Direction::Down | Direction::Up => {
                        let check_left = pos+Point::left();
                        let check_right = pos+Point::right();
                        if !(check_left.x < 0) {
                            match path[check_left.y as usize][check_left.x as usize] {
                                ' ' | '|' => {
                                },
                                _ => {
                                    travel_dir = Direction::Left;
                                },
                            }
                        }
                        if !(check_right.x >= path[0].len() as i32) {
                            match path[check_right.y as usize][check_right.x as usize] {
                                ' ' | '|' => {
                                },
                                _ => {
                                    travel_dir = Direction::Right;
                                },
                            }
                        }
                    },
                    Direction::Left | Direction::Right => {
                        let check_up = pos+Point::up();
                        let check_down = pos+Point::down();
                        if !(check_up.y < 0) {
                            match path[check_up.y as usize][check_up.x as usize] {
                                ' ' | '-' => {
                                },
                                _ => {
                                    travel_dir = Direction::Up;
                                },
                            }
                        }
                        if !(check_down.y >= path[0].len() as i32) {
                            match path[check_down.y as usize][check_down.x as usize] {
                                ' ' | '-' => {
                                },
                                _ => {
                                    travel_dir = Direction::Down;
                                },
                            }
                        }
                    },
                }
            },
            c => tokens_visited.push(c),
        }

        steps += 1;

        pos += match travel_dir {
            Direction::Up => Point::up(),
            Direction::Left => Point::left(),
            Direction::Down => Point::down(),
            Direction::Right => Point::right(),
        };
    }
    let mut visited = String::new();

    for token in tokens_visited {
        write!(&mut visited, "{}", token).unwrap();
    }

    out = match part {
        Part::PartOne => visited,
        Part::PartTwo => steps.to_string(),
    };

    out
}