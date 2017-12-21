use utils;
use utils::Part;
use std::fmt::Write;

pub fn solve(part:Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 10).unwrap();

    const LIST_SIZE:usize = 256;
    const SALT:[u32;5] = [17, 31, 73, 47, 23];

    let mut out = String::new();

    let mut list:Vec<u32>= (0..LIST_SIZE as u32).collect();
    let mut lengths:Vec<u32> = match part {
        Part::PartOne => input.trim().split(',').map(|s| s.parse().unwrap()).collect(),
        Part::PartTwo => input.trim().bytes().map(|b| b as u32).collect(),
    };

    match part {
        Part::PartTwo => { lengths.extend_from_slice(&SALT); },
        _ => {},
    }

    let round_count = match part {
        Part::PartOne => 1,
        Part::PartTwo => 64,
    };

    let mut skip_size = 0;
    let mut pos:usize = 0;

    for round in 0..round_count {
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

    out = match part {
        Part::PartOne => (list[0] as i32 * list[1] as i32).to_string(),
        Part::PartTwo => {
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
    };

    out
}