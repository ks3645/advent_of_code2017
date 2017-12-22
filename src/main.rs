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
days!( day16);

use utils::Part;

fn main() {
    print_all_solutions();
}