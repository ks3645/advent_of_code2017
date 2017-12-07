use std::fs::File;
use std::io::Read;

enum Part{
    PartOne,
    PartTwo,
}

fn main() {
    let solution:u32 = day_one(Part::PartOne);
    println!("Day 1 Part One Captcha Solution: {}", solution);

    let solution:u32 = day_one(Part::PartTwo);
    println!("Day 1 Part Two Captcha Solution: {}", solution);

    let hash:u32 = day_two(Part::PartOne);
    println!("Day 2 Part One Spreadsheet Hash: {}", hash);

    let hash:u32 = day_two(Part::PartTwo);
    println!("Day 2 Part Two Spreadsheet Hash: {}", hash);
}

fn day_one(part:Part) -> u32{
    let mut captcha = String::new();
    let mut file = File::open("day1.txt").unwrap();
    file.read_to_string(&mut captcha).unwrap();

    let captcha = captcha.trim();

    let mut sum:u32 = 0;

    let offset = match part{
        Part::PartOne => 1,
        Part::PartTwo => captcha.len()/2,
    };

    let char_iter = captcha.chars();
    let mut offset_iter = captcha.chars().cycle().skip(offset);

    for c in char_iter{
        let i:u32 = c.to_digit(10).unwrap();
        let j:u32 = offset_iter.next().unwrap().to_digit(10).unwrap();

        if i == j {
            sum += i;
        }
    }

    sum
}

fn day_two(part:Part) -> u32{
    let mut spreadsheet = String::new();
    let mut file = File::open("day2.txt").unwrap();
    file.read_to_string(&mut spreadsheet).unwrap();

    let mut sum:u32 = 0;

    for line in spreadsheet.split('\n') {

        if line.is_empty() {continue;}

        let mut nums:Vec<u32> = line.split_whitespace()
            .map(|s| s.parse().unwrap()).collect();

        nums.sort();

        match part {
            Part::PartOne => {sum += nums.last().unwrap() - nums.first().unwrap()},
            Part::PartTwo => {
                for (i, divisor) in nums.iter().enumerate() {
                    for num in nums[i+1..nums.len()].iter() {
                        if num % divisor == 0 { sum += num/divisor;}
                    }
                }
            },
        }
    }

    sum
}
