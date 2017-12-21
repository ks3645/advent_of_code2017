use std::fmt::Write;
use utils::Part;

macro_rules! days {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
        )*
        fn print_all_solutions() {
            $(
                  println!("{}: Part One Solution: {}", stringify!($x), $x::solve(Part::PartOne));
                  println!("{}: Part Two Solution: {}", stringify!($x), $x::solve(Part::PartTwo));
            )*
        }
    };
}

mod utils;
days!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11);

fn main() {
    print_all_solutions();
}