use utils;
use utils::Part;

pub fn solve(part:Part) -> i32 {
    let mut stream = String::new();
    utils::read_input_to_string(&mut stream, 9).unwrap();

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