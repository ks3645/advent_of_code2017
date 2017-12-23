use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    const INSERT_START:i32 = 2;

    let mut input = String::new();
    utils::read_input_to_string(&mut input, 17).unwrap();

    let mut out = 0;

    let step_count:usize = input.trim().parse().unwrap();

    let mut spinlock = vec![0, 1]; // This spinlock is made regardless of step_count choice
    let mut pos:usize = 1;

    let max_insert = match part {
        Part::PartOne => 2017,
        Part::PartTwo => 50_000_000,
    };

    let find_after = match part {
        Part::PartOne => 2017,
        Part::PartTwo => 0,
    };

    for to_insert in INSERT_START..(max_insert + 1) {
        for _ in 0..(step_count) {
            pos += 1;
            pos %= spinlock.len();
        }
        pos += 1;
        spinlock.insert(pos , to_insert);
    }

    println!("{:?}", spinlock);

    let mut spinlock_iter = spinlock.iter().cycle();
    spinlock_iter.find(|&&x| x == find_after).unwrap();
    let value = spinlock_iter.next().unwrap();

    out = *value;

    out
}