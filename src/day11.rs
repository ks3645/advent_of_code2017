use utils;
use utils::Part;
use utils::HexTile;

pub fn solve(part:Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 11).unwrap();

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