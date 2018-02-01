use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut spreadsheet = String::new();
    utils::read_input_to_string(&mut spreadsheet, 2).unwrap();

   checksum(spreadsheet, part)
}

fn checksum(spreadsheet:String, part:Part) -> u32 {
    let mut sum: u32 = 0;

    for line in spreadsheet.lines() {
        let mut nums: Vec<u32> = line.split_whitespace()
            .map(|s| s.parse().unwrap()).collect();

        nums.sort();

        match part {
            Part::PartOne => { sum += nums.last().unwrap() - nums.first().unwrap() }
            Part::PartTwo => {
                for (i, divisor) in nums.iter().enumerate() {
                    for num in nums[i + 1..nums.len()].iter() {
                        if num % divisor == 0 { sum += num / divisor; }
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(checksum(String::from("5 1 9 5\n7 5 3\n2 4 6 8"), Part::PartOne), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(checksum(String::from("5 9 2 8\n9 4 7 3\n3 8 6 5"), Part::PartTwo), 9);
    }
}