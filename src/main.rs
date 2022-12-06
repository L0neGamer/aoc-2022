mod day01;
mod day02;
mod day03;
pub mod myutils;

fn main() {
    println!("Hello, world!");
    println!("01.1 {:?}",day01::day01("./inputs/day01_1".to_string()));
    println!("01.2 {:?}",day01::day01_sol2("./inputs/day01_1".to_string()));

    println!("02.1 {:?}", day02::day02_sol1("./inputs/day02_1"));
    println!("02.2 {:?}", day02::day02_sol2("./inputs/day02_1"));

    println!("03.1.e {:?}", day03::day03_sol1("./inputs/day03_example"));
    println!("03.1 {:?}", day03::day03_sol1("./inputs/day03_1"));
    println!("03.2 {:?}", day03::day03_sol2("./inputs/day03_1"));
}
