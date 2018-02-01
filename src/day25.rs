use utils;
use utils::Part;
use std::collections::VecDeque;

pub fn solve(part: Part) -> i32 {
    let mut out = 0;

//    let mut tape= vec![0]; //maybe make a deque instead if front insertions end up being too slow
    let mut tape = VecDeque::new();
    tape.push_back(0);
    let mut cursor_pos:i32 = 0;
    let mut state = 'A';

    let step_count = 12_317_297;

    for _ in 0..step_count {
        match state {
            'A' => {
                match tape[cursor_pos as usize] {
                    0 => {
                        tape[cursor_pos as usize] = 1;
                        cursor_pos += 1;
                        state = 'B';
                    },
                    1 => {
                        tape[cursor_pos as usize] = 0;
                        cursor_pos -= 1;
                        state = 'D';
                    },
                    _ => panic!("Unknown Tape Value"),
                }
            },
            'B' => {
                match tape[cursor_pos as usize] {
                    0 => {
                        tape[cursor_pos as usize] = 1;
                        cursor_pos += 1;
                        state = 'C';
                    },
                    1 => {
                        tape[cursor_pos as usize] = 0;
                        cursor_pos += 1;
                        state = 'F';
                    },
                    _ => panic!("Unknown Tape Value"),
                }
            },
            'C' => {
                match tape[cursor_pos as usize] {
                    0 => {
                        tape[cursor_pos as usize] = 1;
                        cursor_pos -= 1;
                        state = 'C';
                    },
                    1 => {
                        tape[cursor_pos as usize] = 1;
                        cursor_pos -= 1;
                        state = 'A';
                    },
                    _ => panic!("Unknown Tape Value"),
                }
            },
            'D' => {
                match tape[cursor_pos as usize] {
                    0 => {
                        tape[cursor_pos as usize] = 0;
                        cursor_pos -= 1;
                        state = 'E';
                    },
                    1 => {
                        tape[cursor_pos as usize] = 1;
                        cursor_pos += 1;
                        state = 'A';
                    },
                    _ => panic!("Unknown Tape Value"),
                }
            },
            'E' => {
                match tape[cursor_pos as usize] {
                    0 => {
                        tape[cursor_pos as usize] = 1;
                        cursor_pos -= 1;
                        state = 'A';
                    },
                    1 => {
                        tape[cursor_pos as usize] = 0;
                        cursor_pos += 1;
                        state = 'B';
                    },
                    _ => panic!("Unknown Tape Value"),
                }
            },
            'F' => {
                match tape[cursor_pos as usize] {
                    0 => {
                        tape[cursor_pos as usize] = 0;
                        cursor_pos += 1;
                        state = 'C';
                    },
                    1 => {
                        tape[cursor_pos as usize] = 0;
                        cursor_pos += 1;
                        state = 'E';
                    },
                    _ => panic!("Unknown Tape Value"),
                }
            },
            _ => panic!("Unknown State"),
        }

        if cursor_pos < 0 {
            cursor_pos = 0;
//            tape.insert(0, 0);
            tape.push_front(0);
        }
        else if cursor_pos >= tape.len() as i32 {
            tape.push_back(0);
        }
    }

    out = tape.iter().sum();

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hard to implement tests given the way the problem input was given, may rectify later

    /*
    #[test]
    fn test_part_one() {
    }*/
}