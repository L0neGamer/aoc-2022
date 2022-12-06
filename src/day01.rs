use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::num::ParseIntError;
use std::iter::Iterator;

pub fn day01 (filename: String) -> Option<i32> {
    println!("{:?}", std::env::current_dir());
    let file = File::open(filename).ok()?;
    let file2 = BufReader::new(file);
    let split: Vec<_> = file2.lines().into_iter().collect::<Result<Vec<String>,_>>().ok()?;
    let packs = to_packs(split).ok()?;

    let max_elf = find_max(packs)?;

    let m = max_elf.cals.iter().sum();

    Some(m)
}

#[derive(Debug, Clone)]
struct ElfPack {
    cals: Vec<i32>
}

fn to_packs(strs: Vec<String>) -> Result<Vec<ElfPack>, String> {
    let collected: Vec<&[String]> = strs.split(|x| x.is_empty()).collect();

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
    with_max.sort_by(|(_,o), (_,o2)| o.cmp(o2));
    with_max.reverse();
    let res = with_max.iter().map(|(e,_)| (*e).clone()).collect::<Vec<_>>();
    res.iter().take(3).cloned().collect()
}


pub fn day01_sol2 (filename: String) -> Option<i32> {
    println!("{:?}", std::env::current_dir());
    let mut file = File::open(filename).ok()?;
    let mut str: String = "".to_string();
    file.read_to_string(&mut str).ok()?;
    let split: Vec<_> = str.split('\n').map(|x| x.to_string()).collect();
    let packs = to_packs(split).ok()?;

    let elves = get_top_three(packs);

    let m = elves
        .iter()
        .flat_map(|e| e.cals.clone())
        .collect::<Vec<_>>();
    
    Some(m.iter().sum())
}
