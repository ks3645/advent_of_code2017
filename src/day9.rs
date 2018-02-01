use utils;
use utils::Part;

pub fn solve(part:Part) -> i32 {
    let mut stream = String::new();
    utils::read_input_to_string(&mut stream, 9).unwrap();

    process_garbage(stream, part)
}

fn process_garbage(stream:String, part:Part) -> i32 {
    let mut out = 0;

    let mut score = 0;
    let mut garbage_count = 0;
    let mut level = 0;
    let mut in_garbage = false;
    let mut canceled = false;

    let mut stack = Vec::new();

    'char_loop: for c in stream.chars() {
        if canceled {
            canceled = false;
            continue 'char_loop;
        }

        if c=='!' {
            canceled = true;
            continue 'char_loop;
        }

        if in_garbage {
            if c=='>' {
                while stack.pop().unwrap() != '<' {}
                in_garbage = false;
            }
                else {
                    garbage_count += 1;
                }
            continue 'char_loop;
        }

        if c=='<' {
            in_garbage = true;
        }

        if c=='}' {
            while stack.pop().unwrap() != '{' {}
            score += level;
            level -= 1;
            continue 'char_loop;
        }

        if c=='{' {
            level += 1;
        }

        stack.push(c);
    }

    match part{
        Part::PartOne => {out = score;},
        Part::PartTwo => {out = garbage_count;},
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(process_garbage(String::from("{}"), Part::PartOne), 1);
        assert_eq!(process_garbage(String::from("{{{}}}"), Part::PartOne), 6);
        assert_eq!(process_garbage(String::from("{{},{}}"), Part::PartOne), 5);
        assert_eq!(process_garbage(String::from("{{{},{},{{}}}}"), Part::PartOne), 16);
        assert_eq!(process_garbage(String::from("{<a>,<a>,<a>,<a>}"),
                                   Part::PartOne), 1);
        assert_eq!(process_garbage(String::from("{{<ab>},{<ab>},{<ab>},{<ab>}}"),
                                   Part::PartOne), 9);
        assert_eq!(process_garbage(String::from("{{<!!>},{<!!>},{<!!>},{<!!>}}"),
                                   Part::PartOne), 9);
        assert_eq!(process_garbage(String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}"),
                                   Part::PartOne), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_garbage(String::from("<>"), Part::PartTwo), 0);
        assert_eq!(process_garbage(String::from("<random characters>"),
                                   Part::PartTwo), 17);
        assert_eq!(process_garbage(String::from("<<<<>"), Part::PartTwo), 3);
        assert_eq!(process_garbage(String::from("<{!>}>"), Part::PartTwo), 2);
        assert_eq!(process_garbage(String::from("<!!>"), Part::PartTwo), 0);
        assert_eq!(process_garbage(String::from("<!!!>>"), Part::PartTwo), 0);
        assert_eq!(process_garbage(String::from("<{o\"i!a,<{i<a>"), Part::PartTwo), 10);
    }
}