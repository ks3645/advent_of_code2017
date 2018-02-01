use utils;
use utils::Part;

pub fn solve(part: Part) -> u32 {
    let mut database = String::new();
    utils::read_input_to_string(&mut database, 4).unwrap();

    count_valid_passphrases(database, part)
}


//TODO: split into counting and checking functions
fn count_valid_passphrases(database:String, part:Part) -> u32 {

    let mut sum: u32 = 0;

    'lines: for passphrase in database.lines() {
        if passphrase.is_empty() {continue 'lines;}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            count_valid_passphrases(
                String::from("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa"),
                   Part::PartOne
            ), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            count_valid_passphrases(
                String::from("abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niii oiii ooii oooi oooo\n oiii ioii iioi iiio"),
                Part::PartTwo
        ), 3);
    }
}