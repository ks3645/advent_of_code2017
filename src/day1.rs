use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut captcha = String::new();
    utils::read_input_to_string(&mut captcha, 1).unwrap();

    let captcha = captcha.trim();

    let mut sum: u32 = 0;

    let offset = match part {
        Part::PartOne => 1,
        Part::PartTwo => captcha.len() / 2,
    };

    let char_iter = captcha.chars();
    let mut offset_iter = captcha.chars().cycle().skip(offset);

    for c in char_iter {
        let i: u32 = c.to_digit(10).unwrap();
        let j: u32 = offset_iter.next().unwrap().to_digit(10).unwrap();

        if i == j {
            sum += i;
        }
    }

    sum
}