use crate::myutils::files::read_lines;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Pair {
    opp: Rps,
    me: Rps,
}

fn rps_value(play: &Rps) -> i32 {
    match play {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

fn rps_score(opp: &Rps, me: &Rps) -> i32 {
    match (me, opp) {
        (Rps::Rock, Rps::Paper) => 0,
        (Rps::Rock, Rps::Scissors) => 6,
        (Rps::Paper, Rps::Rock) => 6,
        (Rps::Paper, Rps::Scissors) => 0,
        (Rps::Scissors, Rps::Rock) => 0,
        (Rps::Scissors, Rps::Paper) => 6,
        _ => 3, // tying
    }
}

fn pair_score(p: &Pair) -> i32 {
    let Pair { opp, me } = p;
    rps_value(me) + rps_score(opp, me)
}

fn day02_sol1_pure(pairs: Vec<Pair>) -> i32 {
    pairs.iter().map(pair_score).sum()
}

fn get_pairs(filename: &str) -> Vec<Pair> {
    let lines = read_lines(filename);
    lines
        .iter()
        .map(|s| make_pair(&s[..]))
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .unwrap()
}

fn make_pair(input: &str) -> Option<Pair> {
    let split: Vec<_> = input.split(' ').collect();
    let opp = match split[0] {
        "A" => Some(Rps::Rock),
        "B" => Some(Rps::Paper),
        "C" => Some(Rps::Scissors),
        _ => None,
    }?;
    let me = match split[1] {
        "X" => Some(Rps::Rock),
        "Y" => Some(Rps::Paper),
        "Z" => Some(Rps::Scissors),
        _ => None,
    }?;

    Some(Pair { opp, me })
}

pub fn day02_sol1(file: &str) -> i32 {
    day02_sol1_pure(get_pairs(file))
}
