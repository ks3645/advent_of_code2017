use utils;
use utils::Part;
use std::fmt::Write;

pub fn solve(part:Part) -> String {
    const LIST_SIZE:usize = 256;

    let mut input = String::new();
    utils::read_input_to_string(&mut input, 10).unwrap();

    do_the_thing(input, LIST_SIZE, part)
}

//TODO: Refactor to use hash function in utils
fn do_the_thing(input:String, list_size:usize, part:Part) -> String {

    const SALT:[u32;5] = [17, 31, 73, 47, 23];

    let mut out = String::new();

    let mut list:Vec<u32>= (0..list_size as u32).collect();
    let mut lengths:Vec<u32> = match part {
        Part::PartOne => input.trim().split(',')
            .map(|s| s.trim().parse().unwrap()).collect(),
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
                sublist.push(list[(pos + i as usize) % list_size]);
            }
            for (i, element) in sublist.iter().rev().enumerate() {
                list[(pos + i as usize) % list_size] = element.clone();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(do_the_thing(String::from("3,4,1,5"), 5, Part::PartOne),
        String::from("12"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(do_the_thing( String::from(""), 256, Part::PartTwo),
            String::from("a2582a3a0e66e6e86e3812dcb672a272"));
        assert_eq!(do_the_thing(String::from("AoC 2017"), 256, Part::PartTwo),
            String::from("33efeb34ea91902bb2f59c9920caa6cd"));
        assert_eq!(do_the_thing(String::from("1,2,3"), 256, Part::PartTwo),
               String::from("3efbe78a8d82f29979031a4aa0b16a9d"));
        assert_eq!(do_the_thing(String::from("1,2,4"), 256, Part::PartTwo),
               String::from("63960835bcdc130f0b66d7ff4f6a5a8e"));
    }
}