use utils;
use utils::Part;
use utils::HexTile;

pub fn solve(part:Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 11).unwrap();

    find_distance(input, part)
}

fn find_distance(input:String, part:Part) -> i32 {

    let mut distance = 0;

    let steps:Vec<String> = input.trim().split(',')
        .map(|s| s.to_string()).collect();

    let mut pos = HexTile{ r:0, s:0, t:0};
    let mut max_distance = 0;

    for step in steps {
        pos += match step.as_ref() {
            "n" => HexTile::n(),
            "nw" => HexTile::nw(),
            "sw" => HexTile::sw(),
            "s" => HexTile::s(),
            "se" => HexTile::se(),
            "ne" => HexTile::ne(),
            _ => panic!("Unrecognized Direction"),
        };

        if pos.distance(HexTile{ r:0, s:0, t:0}) > max_distance {
            max_distance = pos.distance(HexTile{ r:0, s:0, t:0});
        }
    }

    distance = match part {
        Part::PartOne => pos.distance(HexTile { r: 0, s: 0, t: 0 }),
        Part::PartTwo => max_distance,
    };

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(find_distance(String::from("ne,ne,ne"), Part::PartOne), 3);
        assert_eq!(find_distance(String::from("ne,ne,sw,sw"), Part::PartOne), 0);
        assert_eq!(find_distance(String::from("ne,ne,s,s"), Part::PartOne), 2);
        assert_eq!(find_distance(String::from("se,sw,se,sw,sw"), Part::PartOne), 3);
    }

    #[test]
    fn test_part_two() {
        // no test case provided in problem description
    }
}