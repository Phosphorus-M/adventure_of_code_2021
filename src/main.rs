pub mod days;
use days::{day1, day2, day3};

fn main() {
    day1::part1("./Inputs/day1.txt");
    println!("------------------------");
    day2::part1("./Inputs/day2.txt");
    println!("------------------------");
    day2::part2("./Inputs/day2.txt");
    println!("------------------------");
    day3::part1("./Inputs/day3.txt");
    day3::part2("./Inputs/day3.txt");
}
