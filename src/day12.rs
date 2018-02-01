use utils;
use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(part:Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 12).unwrap();

    process_groups(input, part)
}

fn process_groups(input:String, part:Part) -> i32 {
    let mut out = 0;

    let mut village:HashMap<i32, Vec<i32>> = HashMap::new();

    for line in input.lines() {
        let mut token_iter = line.split("<->");
        let program:i32 = token_iter.next().unwrap().trim().parse().unwrap();

        let mut pipe_iter = token_iter.next().unwrap().split(',')
            .map(|s| s.trim().parse().unwrap());
        let mut pipe_vec:Vec<i32> = Vec::new();

        while let Some(pipe) = pipe_iter.next() {
            pipe_vec.push(pipe);
        }
        village.insert(program, pipe_vec);
    }

    let loop_count = match part {
        Part::PartOne => 1,
        Part::PartTwo => village.len(),
    };

    let mut in_any_group = HashSet::new();
    let mut group_count = 0;

    for i in 0..loop_count {
        if in_any_group.contains(&(i as i32)) {continue;}
        let mut group: HashSet<i32> = HashSet::new();
        let mut to_check: HashSet<i32> = HashSet::new();

        to_check.insert(i as i32);
        group_count += 1;

        while !to_check.is_empty() {
            let mut check_list = to_check.clone();
            for item in check_list.drain() {
                let connections = village.get(&item).unwrap();
                group.insert(item);
                in_any_group.insert(item);
                for pipe in connections {
                    if !group.contains(pipe) { to_check.insert(*pipe); }
                }
            }
            to_check = &to_check - &group;
        }
    }

    out = match part {
        Part::PartOne => in_any_group.len() as i32,
        Part::PartTwo => group_count,
    };

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = String::from("0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5");
        assert_eq!(process_groups(test_input, Part::PartOne), 6);
    }

    #[test]
    fn test_part_two() {
        let test_input = String::from("0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5");
        assert_eq!(process_groups(test_input, Part::PartTwo), 2);
    }
}