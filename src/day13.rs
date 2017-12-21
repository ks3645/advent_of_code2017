use utils;
use utils::Part;

pub fn solve(part:Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 13).unwrap();

    let mut out = 0;

    let mut firewall:Vec<(i32,i32)> = Vec::new();

    for line in input.lines() {
        let tokens:Vec<i32> = line.split(':')
            .map(|s| s.trim().parse().unwrap()).collect();
        firewall.push((tokens[0], tokens[1]));
    }

    let mut delay = 0;
    let mut caught = false;
    loop {
        let mut severity = 0;
        for (depth, range) in firewall.to_vec() {
            if (depth+delay) % (range * 2 - 2) == 0 {
                severity += depth * range;
                caught = true;
            }
        }
        match part {
            Part::PartOne => {
                out = severity;
                break;
            }
            Part::PartTwo => {
                if !caught {
                    out = delay;
                    break;
                }
            }
        }
        delay += 1;
        caught = false;
    }

    out
}