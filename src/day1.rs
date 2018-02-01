use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut captcha = String::new();
    utils::read_input_to_string(&mut captcha, 1).unwrap();

    solve_captcha(captcha, part)
}

fn solve_captcha(captcha:String, part:Part) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_captcha(String::from("1122"), Part::PartOne), 3);
        assert_eq!(solve_captcha(String::from("1111"), Part::PartOne), 4);
        assert_eq!(solve_captcha(String::from("1234"), Part::PartOne), 0);
        assert_eq!(solve_captcha(String::from("91212129"), Part::PartOne), 9);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_captcha( String::from("1212"), Part::PartTwo), 6);
        assert_eq!(solve_captcha(String::from("1221"), Part::PartTwo), 0);
        assert_eq!(solve_captcha(String::from("123425"), Part::PartTwo), 4);
        assert_eq!(solve_captcha(String::from("123123"), Part::PartTwo), 12);
        assert_eq!(solve_captcha(String::from("12131415"), Part::PartTwo), 4);
    }
}