use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

    let distance = day_three(Part::PartOne);
    println!("Day 3 Part One Distance: {}", distance);

    let value = day_three(Part::PartTwo);
    println!("Day 3 Part Two Value: {}", value);

    let count = day_four(Part::PartOne);
    println!("Day 4 Part One Valid Count: {}", count);

    let count = day_four(Part::PartTwo);
    println!("Day 4 Part Two Valid Count: {}", count);

    let count = day_five(Part::PartOne);
    println!("Day 5 Part One Move Count: {}", count);

    let count = day_five(Part::PartTwo);
    println!("day 5 Part Two Move Count: {}", count);
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


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point{
    x: i32,
    y: i32,
}


#[derive(Debug, Clone, Copy)]
enum Direction{
    Right,
    Up,
    Down,
    Left,
}

fn get_neighbors(pos:Point) -> Vec<Point> {
    let mut neighbors = Vec::with_capacity(8);

    for i in -1..2 {
        for j in -1..2{
            if i==0 && j==0 {continue;}

            neighbors.push(Point{x:pos.x+i, y:pos.y+j});
        }
    }

    neighbors
}

fn day_three(part:Part) -> u32{
    let mut pos = String::new();
    let mut file = File::open("day3.txt").unwrap();
    file.read_to_string(&mut pos).unwrap();

    let pos:u32 = pos.trim().parse().unwrap();

    let mut result:u32 = 0;

    let mut grid_map = HashMap::new();

    let mut spiral_directions = vec![Direction::Up, Direction::Left,
                                Direction::Down, Direction::Right]
                                    .into_iter().cycle();
    let mut travel_direction = spiral_directions.next().unwrap();
    let mut check_direction = spiral_directions.next().unwrap();

    grid_map.insert(Point{x:0, y:0}, 1);

    let mut cursor_pos = Point{x:1, y:0};

    for i in 2..pos+1 {

        match part{
            Part::PartOne => {grid_map.insert(cursor_pos, i);},
            Part::PartTwo => {
                let mut sum = 0;
                for neighbor in get_neighbors(cursor_pos).into_iter() {
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
            },
        }

        if i == pos{
            result = (cursor_pos.x.abs() + cursor_pos.y.abs()) as u32;
        }

        match travel_direction {
            Direction::Up => cursor_pos.y += 1,
            Direction::Left => cursor_pos.x += -1,
            Direction::Down => cursor_pos.y += -1,
            Direction::Right => cursor_pos.x += 1,
        }

        let check_pos = match check_direction{
            Direction::Up => Point{x:cursor_pos.x, y:cursor_pos.y+1},
            Direction::Left => Point{x:cursor_pos.x-1, y:cursor_pos.y},
            Direction::Down => Point{x:cursor_pos.x, y:cursor_pos.y-1},
            Direction::Right => Point{x:cursor_pos.x+1, y:cursor_pos.y},
        };

        if !grid_map.contains_key(&check_pos) {
            travel_direction = check_direction;
            check_direction = spiral_directions.next().unwrap();
        }

    }

    result
}

fn day_four(part:Part) -> u32{
    let mut database = String::new();
    let mut file = File::open("day4.txt").unwrap();
    file.read_to_string(&mut database).unwrap();
    
    let mut sum:u32 = 0;
    
    'lines: for passphrase in database.split('\n') {
        if passphrase.is_empty() {continue;}

        let words:Vec<&str> = passphrase.split_whitespace().collect();

        'outer: for i in 0..words.len(){
            'inner: for j in i+1..words.len(){
                match part {
                    Part::PartOne => if words[i] == words[j] {continue 'lines;},
                    Part::PartTwo => {
                        let mut letters_i:Vec<char> = words[i].chars().collect();
                        let mut letters_j:Vec<char> = words[j].chars().collect();
                        letters_i.sort();
                        letters_j.sort();
                        if letters_i==letters_j {continue 'lines;}
                    }
                }
            }
        }
        sum += 1;
    }

    sum
}

fn day_five(part:Part) -> u32{
    let mut maze = String::new();
    let mut file = File::open("day5.txt").unwrap();
    file.read_to_string(&mut maze).unwrap();

    let mut sum:u32 = 0;

    let mut instructions:Vec<i32> = Vec::new();

    for line in maze.split('\n'){
        if line.is_empty() {continue;}

        instructions.push(line.parse().unwrap());
    }
    
    let mut pos:i32 = 0;

    while let Some(result) = instructions.to_vec().get(pos as usize) {
        instructions[pos as usize] += match part {
            Part::PartOne => 1,
            Part::PartTwo => {
                if *result >= 3 { -1}
                else {1}
            }
        };
        pos = pos + *result;
        sum += 1;
    }

    sum
}
