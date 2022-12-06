use crate::myutils::files::read_lines;
use std::collections::{hash_map::RandomState, HashSet};

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

    Rucksack { left, right }
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
    (*left
        .iter()
        .fold(None, |acc, l| acc.or(right.iter().find(|r| *r == l)))
        .unwrap()) as u32
}

pub fn day03_sol1(f: &str) -> u32 {
    read_lines(f)
        .iter()
        .map(|s| to_sack(s.as_str()))
        .map(|r| find_match(&r))
        .sum()
}

fn to_badges(ss: Vec<String>) -> Vec<u8> {
    ss.chunks(3)
        .map(|v| {
            *v.iter()
                .map(|s| HashSet::from_iter(to_compartment(&s[..])))
                .reduce(|acc, e| {
                    acc.intersection(&e)
                        .copied()
                        .collect::<HashSet<u8, RandomState>>()
                })
                .unwrap()
                .iter()
                .next()
                .unwrap()
        })
        .collect()
}

pub fn day03_sol2(f: &str) -> u32 {
    to_badges(read_lines(f)).iter().map(|i| *i as u32).sum()
}
