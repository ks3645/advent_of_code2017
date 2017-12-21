use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut spreadsheet = String::new();
    utils::read_input_to_string(&mut spreadsheet, 2).unwrap();

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