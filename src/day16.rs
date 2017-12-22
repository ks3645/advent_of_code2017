use utils;
use utils::Part;
use std::fmt::Write;

pub fn solve(part: Part) -> String {
    const SIZE:usize = 16;

    let mut input = String::new();
    utils::read_input_to_string(&mut input, 16).unwrap();

    let mut out = String::new();

    let mut programs = vec!["a", "b", "c", "d", "e", "f", "g", "h"
                        , "i", "j", "k", "l", "m", "n", "o", "p"];

    let start = programs.to_vec();

    let loop_count = match part {
        Part::PartOne => 1,
        Part::PartTwo => 1_000_000_000,
    };

    let mut cycle_count = 1;

    for count in 0..loop_count {
        for dance_move in input.split(',') {
            match dance_move[0..1].as_ref() {
                "s" => {
                    let count: usize = dance_move[1..].trim().parse().unwrap();
                    programs.reverse();
                    let (a, b) = programs.split_at_mut(count);
                    a.reverse();
                    b.reverse();
                },
                "x" => {
                    let mut positions = dance_move[1..].split('/')
                        .map(|s| s.trim().parse().unwrap());
                    programs.swap(positions.next().unwrap(), positions.next().unwrap());
                },
                "p" => {
                    let mut names = dance_move[1..].split('/');
                    let one = names.next().unwrap().trim();
                    let two = names.next().unwrap().trim();
                    let a = programs.iter().position(|&s| s == one).unwrap();
                    let b = programs.iter().position(|&s| s == two).unwrap();
                    programs.swap(a, b);
                },
                _ => panic!("Unknown Dance Move"),
            }
        }

        if programs == start {
            cycle_count = count+1;
            break;
        }
    }

    for _ in 0..(loop_count%cycle_count) {
        for dance_move in input.split(',') {
            match dance_move[0..1].as_ref() {
                "s" => {
                    let count: usize = dance_move[1..].trim().parse().unwrap();
                    programs.reverse();
                    let (a, b) = programs.split_at_mut(count);
                    a.reverse();
                    b.reverse();
                },
                "x" => {
                    let mut positions = dance_move[1..].split('/')
                        .map(|s| s.trim().parse().unwrap());
                    programs.swap(positions.next().unwrap(), positions.next().unwrap());
                },
                "p" => {
                    let mut names = dance_move[1..].split('/');
                    let one = names.next().unwrap().trim();
                    let two = names.next().unwrap().trim();
                    let a = programs.iter().position(|&s| s == one).unwrap();
                    let b = programs.iter().position(|&s| s == two).unwrap();
                    programs.swap(a, b);
                },
                _ => panic!("Unknown Dance Move"),
            }
        }
    }



    let mut order = String::new();

    for name in programs {
        write!(&mut order, "{}", name).unwrap();
    }
    out = order;

    out
}