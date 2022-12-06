mod day01;

fn main() {
    println!("Hello, world!");
    println!("01.1 {:?}",day01::day01("./inputs/day01_1".to_string()));
    println!("02.2 {:?}",day01::day01_sol2("./inputs/day01_1".to_string()));
}
