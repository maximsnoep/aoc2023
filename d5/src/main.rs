use std::collections::HashMap;
use rayon::prelude::*;

use itertools::Itertools;

fn main() {
    let file = "input/test";

    // PART 1
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let seeds = lines.next().unwrap().unwrap().split_whitespace().flat_map(|seed| seed.parse::<u64>()).collect::<Vec<_>>();

    let (_, maps) = lines
        .into_iter()
        .map(|x| x.unwrap())
        .fold((0, vec![]), |(i, mut maps), line| {
            if line.is_empty() { return (i+1, maps) }
            if i != maps.len() {
                maps.push(rangemap::map::RangeMap::new());
            }
            if let Some((dst, src, len)) = line.split_whitespace().flat_map(|x| x.parse::<u64>()).collect_tuple() {
                maps[i-1].insert(src..src+len, (src, dst));
            }
            (i, maps)
        });

    let res = seeds.par_iter().map(|&seed| {
        let mut tmp = seed;
        for k in 0..maps.len() {
            if let Some(&(src, dst)) = maps[k].get(&tmp) {
                tmp = dst+(tmp-src);
            }
        }
        tmp
    }).min().unwrap();

    println!("{:?}", res);


    // PART 2
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let seeds = lines.next().unwrap().unwrap().split_whitespace().flat_map(|seed| seed.parse::<u64>()).chunks(2).into_iter().map(|mut x| { 
        let s = x.next().unwrap(); 
        let l = x.next().unwrap();
        (s..(s+l)).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();

    let (_, maps) = lines
        .into_iter()
        .map(|x| x.unwrap())
        .fold((0, vec![]), |(i, mut maps), line| {
            if line.is_empty() { return (i+1, maps) }
            if i != maps.len() {
                maps.push(rangemap::map::RangeMap::new());
            }
            if let Some((dst, src, len)) = line.split_whitespace().flat_map(|x| x.parse::<u64>()).collect_tuple() {
                maps[i-1].insert(src..src+len, (src, dst));
            }
            (i, maps)
        });

    let res = seeds.par_iter().map(|&seed| {
        let mut tmp = seed;
        for k in 0..maps.len() {
            if let Some(&(src, dst)) = maps[k].get(&tmp) {
                tmp = dst+(tmp-src);
            }
        }
        tmp
    }).min().unwrap();

    println!("{:?}", res);

}