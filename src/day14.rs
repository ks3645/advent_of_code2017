use utils;
use utils::Part;
use utils::Point;
use std::fmt::Write;
use std::collections::HashSet;

pub fn solve(part: Part) -> i32 {
    const ROWS:usize = 128;

    let mut input = String::new();
    utils::read_input_to_string(&mut input, 14).unwrap();

    let mut out = 0;

    let mut count = 0;

    let mut grid:[[i32; ROWS]; ROWS] = [[0; ROWS]; ROWS];

    for i in 0..ROWS {
        let hash = utils::knot_hash(format!("{}-{}", input.trim(), i));
        let mut grid_row = String::new();
        for c in hash.chars(){
            write!(&mut grid_row, "{:04b}", c.to_digit(16).unwrap());
        }
        for (j, c) in grid_row.chars().enumerate() {
            if c=='1' {
                grid[i][j] = -1;
                count += 1;
            }
        }
    }

    let mut region_num = 1;

    match part {
        Part::PartOne => {out = count as i32;},
        Part::PartTwo => {
            for i in 0..ROWS {
                for j in 0..ROWS {
                    if grid[i][j] == -1 {
                        let mut to_check = HashSet::new();
                        let mut checked = HashSet::new();

                        to_check.insert(Point { x: i as i32, y: j as i32 });

                        while !to_check.is_empty() {
                            let mut check_list = to_check.clone();

                            for pos in check_list.drain() {
                                checked.insert(pos);
                                if grid[pos.x as usize][pos.y as usize] == -1 {

                                    grid[pos.x as usize][pos.y as usize] = region_num;
                                    
                                    for neighbor in utils::get_neighbors(pos) {
                                        if !(neighbor.x < 0 || neighbor.x >= ROWS as i32
                                            || neighbor.y < 0 || neighbor.y >= ROWS as i32) {
                                            to_check.insert(neighbor);
                                        }
                                    }
                                }
                            }
                            to_check = &to_check - &checked;
                        }
                        region_num += 1;
                    }
                }
            }
        out = region_num - 1;
        }
    }

    out
}