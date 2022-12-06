use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::num::ParseIntError;
use typestuff;
use std::cmp::Ordering;
use std::iter::Iterator;

pub fn day01 (filename: String) -> Option<i32> {
    println!("{:?}", std::env::current_dir());
    let mut file = File::open(filename).ok()?;
    let mut str: String = "".to_string();
    file.read_to_string(&mut str);
    let split: Vec<_> = str.split("\n").map(|x| x.to_string()).collect();
    let packs = to_packs(split).ok()?;

    println!("packs {:?}", packs);
    let max_elf = find_max(packs)?;
    println!("packs {:?}", max_elf);

    let m = max_elf.cals.iter().sum();

    Some(m)
}

#[derive(Debug, Clone)]
struct ElfPack {
    cals: Vec<i32>
}

fn to_packs(strs: Vec<String>) -> Result<Vec<ElfPack>, String> {
    let collected: Vec<&[String]> = strs.split(|x| x == "").collect();

    let packs = collected.iter().map(|x| to_elfpack(x.to_vec())).into_iter().collect();
    match packs {
        Ok(elfpacks) => Ok(elfpacks),
        Err(_) => Err("parseerr".to_string())
    }
}

fn to_elfpack(strs: Vec<String>) -> Result<ElfPack, ParseIntError> {
    let errs: Vec<i32> = strs.iter().map(|s| s.parse()).into_iter().collect::<Result<Vec<i32>,_>>()?;
    Ok(ElfPack{cals: errs})
}

fn find_max(packs: Vec<ElfPack>) -> Option<ElfPack> {
    let with_max = packs.iter().map(|pack| (pack, pack.cals.iter().sum::<i32>()));
    let found_max = with_max.reduce(|(ep, i), (ep2,i2)| if i2 > i {(ep2,i2)} else {(ep,i)})?.0;
    Some((*found_max).clone())
}

fn get_top_three(packs: Vec<ElfPack>) -> Vec<ElfPack> {
    let mut with_max: Vec<(&ElfPack, i32)> = packs.iter().map(|pack| (pack, pack.cals.iter().sum::<i32>())).collect();
    with_max.sort_by(|(e,o), (e2,o2)| o.cmp(o2));
    // let found_max = with_max.reduce(|(ep, i), (ep2,i2)| if i2 > i {(ep2,i2)} else {(ep,i)})?.0;
    // Some((*found_max).clone())
    with_max.reverse();
    let mut res = with_max.iter().map(|(e,_)| (*e).clone()).collect::<Vec<_>>().clone();
    res.iter().take(3).map(|e| e.clone()).collect()
}


pub fn day01_sol2 (filename: String) -> Option<i32> {
    println!("{:?}", std::env::current_dir());
    let mut file = File::open(filename).ok()?;
    let mut str: String = "".to_string();
    file.read_to_string(&mut str);
    let split: Vec<_> = str.split("\n").map(|x| x.to_string()).collect();
    let packs = to_packs(split).ok()?;

    println!("packs {:?}", packs);
    let elves = get_top_three(packs);
    println!("packs {:?}", elves);

    let m = elves
        .iter()
        .flat_map(|e| e.cals.clone())
        .collect::<Vec<_>>();
    //.reduce(|acc,e| acc.extend(e)).iter().sum();
    
    Some(m.iter().sum())
}
