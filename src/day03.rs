use crate::myutils::files::read_lines;

#[derive(Debug, Clone)]
struct Rucksack {
    left: Vec<u8>,
    right: Vec<u8>,
}

fn to_sack(s: &str) -> Rucksack {
    let len = s.len();
    let mut left = to_compartment(&s[..len / 2]);
    left.sort();

    let mut right = to_compartment(&s[len / 2..]);
    right.sort();

    Rucksack {
        left,
        right,
    }
}

fn to_compartment(s: &str) -> Vec<u8> {
    s.as_bytes()
        .iter()
        .map(|x| {
            if *x >= b'A' && *x <= b'Z' {
                x + 1 + 26 - b'A'
            } else if *x >= b'a' && *x <= b'z' {
                x + 1 - b'a'
            } else {
                *x
            }
        })
        .collect()
}

fn find_match(Rucksack { left, right }: &Rucksack) -> u32 {
    (*left.iter()
        .fold(None, |acc, l| acc.or(right.iter().find(|r| *r == l)))
        .unwrap())
        as u32
}

pub fn day03_sol1(f:&str) -> u32 {
    read_lines(f).iter().map(|s| to_sack(s.as_str())).map(|r| find_match(&r)).sum()
}
