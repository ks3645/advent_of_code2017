use utils;
use utils::Part;
use std::cmp::Ordering;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 24).unwrap();

    process_bridges(input, part)
}

fn process_bridges(input:String, part:Part) -> i32 {
    let mut out = 0;

    let mut pieces = Vec::new(); //input has no dupe pieces so this should be fine

    for line in input.lines() {
        pieces.push(Component::from(line));
    }

    let bridge_start = vec![Component{one:0, two:0}];

    let mut active_bridges = vec![bridge_start];
    let mut complete_bridges = Vec::new();

    while !active_bridges.is_empty() {
         let mut new_bridges = Vec::new();
         for bridge in active_bridges.drain(..) {
             let old_length = bridge.len();
             for piece in pieces.to_vec() {
                 if !bridge.contains(&piece) {
                     if bridge.last().unwrap().connects_to(piece) {
                         let mut new_bridge = bridge.to_vec();
                         new_bridge.push(piece);
                         new_bridges.push(new_bridge);
                     }
                     else if bridge.last().unwrap().connects_to(piece.flip()) {
                         let mut new_bridge = bridge.to_vec();
                         new_bridge.push(piece.flip());
                         new_bridges.push(new_bridge);
                     }
                 }
             }
             if bridge.len() == old_length {
                 complete_bridges.push(bridge);
             }
         }
        active_bridges = new_bridges;
    }



    out = match part {
        Part::PartOne => {
            complete_bridges.sort_by( &compare_bridges_by_strength);
            complete_bridges.last().unwrap().iter().map(|p| p.strength()).sum()
        },
        Part::PartTwo => {
            complete_bridges.sort_by( &compare_bridges_by_length);
            complete_bridges.last().unwrap().iter().map(|p| p.strength()).sum()
        }
    };

    out
}

fn compare_bridges_by_strength(one:&Vec<Component>, two:&Vec<Component>) -> Ordering {
    let one_strength:i32 = one.iter().map(|p| p.strength()).sum();
    let two_strength:i32 = two.iter().map(|p| p.strength()).sum();

    one_strength.cmp(&two_strength)
}

fn compare_bridges_by_length(one:&Vec<Component>, two:&Vec<Component>) -> Ordering {
    match one.len().cmp(&two.len()) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal  => {
            let one_strength:i32 = one.iter().map(|p| p.strength()).sum();
            let two_strength:i32 = two.iter().map(|p| p.strength()).sum();

            one_strength.cmp(&two_strength)
        },
    }
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
struct Component {
    one:i32,
    two:i32,
}

impl PartialEq for Component {
    fn eq(&self, other:&Self) -> bool {
        (self.one == other.one && self.two == other.two)
            || (self.one == other.two && self.two == other.one)
    }
}

impl Component {
    fn strength(&self) -> i32 {
        self.one + self.two
    }

    fn flip(&self) -> Self{
        Component {
            one:self.two,
            two:self.one,
        }
    }

    fn from(s:&str) -> Self {
        let tokens:Vec<i32> = s.trim().split('/')
            .map(|s| s.parse().unwrap()).collect();
        Component {
            one:tokens[0],
            two:tokens[1],
        }
    }

    fn connects_to(&self, other:Self) -> bool{
        self.two == other.one
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = String::from("0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10");
        assert_eq!(process_bridges(test_input, Part::PartOne), 31);
    }

    #[test]
    fn test_part_two() {
        let test_input = String::from("0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10");
        assert_eq!(process_bridges(test_input, Part::PartTwo), 19);
    }
}