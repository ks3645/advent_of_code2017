use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut database = String::new();
    utils::read_input_to_string(&mut database, 4).unwrap();

    let mut sum: u32 = 0;

    'lines: for passphrase in database.lines() {
        let words: Vec<&str> = passphrase.split_whitespace().collect();

        'outer: for i in 0..words.len() {
            'inner: for j in i + 1..words.len() {
                match part {
                    Part::PartOne => if words[i] == words[j] { continue 'lines; },
                    Part::PartTwo => {
                        let mut letters_i: Vec<char> = words[i].chars().collect();
                        let mut letters_j: Vec<char> = words[j].chars().collect();
                        letters_i.sort();
                        letters_j.sort();
                        if letters_i == letters_j { continue 'lines; }
                    }
                }
            }
        }
        sum += 1;
    }

    sum
}