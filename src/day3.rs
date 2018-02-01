use utils;
use utils::Part;
use utils::Point;
use utils::Direction;
use std::collections::HashMap;

pub fn solve(part: Part) -> u32 {
    let mut pos = String::new();
    utils::read_input_to_string(&mut pos, 3).unwrap();

    do_the_thing(pos, part)
}

// TODO: Split this into separate functions
fn do_the_thing(pos:String, part:Part) -> u32 {

    let pos: u32 = pos.trim().parse().unwrap();

    let mut result: u32 = 0;

    let mut grid_map = HashMap::new();

    let mut spiral_directions = vec![Direction::Up, Direction::Left,
                                     Direction::Down, Direction::Right]
        .into_iter().cycle();
    let mut travel_direction = spiral_directions.next().unwrap();
    let mut check_direction = spiral_directions.next().unwrap();

    grid_map.insert(Point { x: 0, y: 0 }, 1);

    let mut cursor_pos = Point { x: 1, y: 0 };

    for i in 2..pos + 1 {
        match part {
            Part::PartOne => { grid_map.insert(cursor_pos, i); }
            Part::PartTwo => {
                let mut sum = 0;
                for neighbor in utils::get_neighbors(cursor_pos).into_iter() {
                    match grid_map.get(&neighbor) {
                        Some(value) => sum = sum + value,
                        None => continue,
                    }
                }
                grid_map.insert(cursor_pos, sum);

                if sum > pos {
                    result = sum;
                    break;
                }
            }
        }

        if i == pos {
            result = (cursor_pos.x.abs() + cursor_pos.y.abs()) as u32;
        }

        match travel_direction {
            Direction::Up => cursor_pos.y += 1,
            Direction::Left => cursor_pos.x += -1,
            Direction::Down => cursor_pos.y += -1,
            Direction::Right => cursor_pos.x += 1,
        }

        let check_pos = match check_direction {
            Direction::Up => Point { x: cursor_pos.x, y: cursor_pos.y + 1 },
            Direction::Left => Point { x: cursor_pos.x - 1, y: cursor_pos.y },
            Direction::Down => Point { x: cursor_pos.x, y: cursor_pos.y - 1 },
            Direction::Right => Point { x: cursor_pos.x + 1, y: cursor_pos.y },
        };

        if !grid_map.contains_key(&check_pos) {
            travel_direction = check_direction;
            check_direction = spiral_directions.next().unwrap();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(do_the_thing(String::from("1"), Part::PartOne), 0);
        assert_eq!(do_the_thing(String::from("12"), Part::PartOne), 3);
        assert_eq!(do_the_thing(String::from("23"), Part::PartOne), 2);
        assert_eq!(do_the_thing(String::from("1024"), Part::PartOne), 31);
    }

    #[test]
    fn test_part_two() {
        //TODO: implement these tests when the function above is split up
    }
}