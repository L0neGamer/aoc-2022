use crate::myutils::{files::read_lines};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Rps {
    Rock, // also means lose in sol2
    Paper, // also means draw in sol2
    Scissors, // also means win in sol2
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

fn pair_score(Pair{opp, me}: &Pair) -> i32 {
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

fn from_int(i: u8) -> Rps {
    match i%3 {
        0 => Rps::Rock,
        1 => Rps::Paper,
        2 => Rps::Scissors,
        _ => panic!("unexpected i: {i:?}")
}
}

fn make_move(starter: Rps, goal: Rps) -> Rps {
    match goal {
        Rps::Rock => from_int((starter as u8) + 2),
        Rps::Paper => starter,
        Rps::Scissors => from_int((starter as u8) + 1)
    }
}

fn swap_pair(Pair { opp, me }: &Pair) -> Pair {
    Pair { opp: *opp, me: make_move(*opp, *me) }
}

pub fn day02_sol2(file: &str) -> i32 {
    day02_sol1_pure(get_pairs(file).iter().map(swap_pair).collect())
}
