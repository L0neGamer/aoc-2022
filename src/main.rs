mod day01;

fn main() {
    println!("Hello, world!");
    println!("{:?}",day01::day01("./inputs/day01_example".to_string()));
    println!("{:?}",day01::day01("./inputs/day01_1".to_string()));
    println!("{:?}",day01::day01_sol2("./inputs/day01_1".to_string()));
}
