mod day01;
mod day02;
pub mod myutils;

fn main() {
    println!("Hello, world!");
    println!("01.1 {:?}",day01::day01("./inputs/day01_1".to_string()));
    println!("01.2 {:?}",day01::day01_sol2("./inputs/day01_1".to_string()));

    println!("02.1 {:?}", day02::day02_sol1("./inputs/day02_1"))
}
