use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    const A_FACTOR:i64 = 16_807;
    const B_FACTOR:i64 = 48_271;

    const A_START:i64 = 634;
    const B_START:i64 = 301;

    const A_MULTIPLE:i64 = 4;
    const B_MULTIPLE:i64 = 8;

    const DIVISOR:i64 = 2_147_483_647;
    const SIXTEEN_BITS:i64 = 0b1111_1111_1111_1111;

    let pair_count = match part {
        Part::PartOne => 40_000_000,
        Part::PartTwo => 5_000_000,
    };

    let mut out = 0;

    let mut gen_a = A_START;
    let mut gen_b = B_START;

    let mut judge_count = 0;

    for _ in 0..pair_count {
        gen_a = match part {
            Part::PartOne => (gen_a * A_FACTOR) % DIVISOR,
            Part::PartTwo => loop {
                    gen_a = (gen_a * A_FACTOR) % DIVISOR;
                    if gen_a % A_MULTIPLE == 0 { break gen_a; }
                },
        };
        gen_b = match part {
            Part::PartOne => (gen_b * B_FACTOR) % DIVISOR,
            Part::PartTwo => loop {
                gen_b = (gen_b * B_FACTOR) % DIVISOR;
                if gen_b % B_MULTIPLE == 0 { break gen_b; }
            },
        };

        if format!("{:016b}",gen_a & SIXTEEN_BITS) == format!("{:016b}",gen_b & SIXTEEN_BITS) {
            judge_count += 1;
        }
    }

    out = judge_count;

    out
}