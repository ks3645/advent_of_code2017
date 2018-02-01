use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut maze = String::new();
    utils::read_input_to_string(&mut maze, 5).unwrap();

    count_steps_to_escape(maze, part)
}

fn count_steps_to_escape(maze:String, part:Part) -> u32 {
    let mut sum: u32 = 0;

    let mut instructions: Vec<i32> = Vec::new();

    for line in maze.lines() {
        instructions.push(line.parse().unwrap());
    }

    let mut pos: i32 = 0;

    while let Some(result) = instructions.to_vec().get(pos as usize) {
        instructions[pos as usize] += match part {
            Part::PartOne => 1,
            Part::PartTwo => {
                if *result >= 3 { -1 } else { 1 }
            }
        };
        pos = pos + *result;
        sum += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(count_steps_to_escape(String::from("0\n3\n0\n1\n-3"), Part::PartOne), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(count_steps_to_escape(String::from("0\n3\n0\n1\n-3"), Part::PartTwo), 10);
    }
}