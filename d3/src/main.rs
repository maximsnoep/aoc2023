use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() {
    let file = "input/test";

    // PART 1
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));
    
    let mut syms: HashSet<(i32, i32)> = HashSet::new();
    let mut nums: HashMap<(RangeInclusive<i32>, RangeInclusive<i32>), usize> = HashMap::new();

    for (j, line) in lines.into_iter().map(|x| x.unwrap()).enumerate() {
        for (i, _) in line.chars().enumerate().filter(|&(_, ch)| ch != '.' && !ch.is_ascii_digit()) {
            syms.insert((j as i32, i as i32));
        }

        let mut index = 0;
        for part in line.split(|ch: char| !ch.is_ascii_digit()) {
            if let Ok(num) = part.parse::<usize>() {
                nums.insert(((j as i32 - 1)..=(j as i32 + 1), (index-1)..=(index+part.len() as i32)), num);
            }
            index += 1 + part.len() as i32;
        }
    }

    let sum = nums.into_iter().fold(0, |acc, ((rj, ri), num)| {
        if rj.clone().any(|j| ri.clone().any(|i| syms.contains(&(j,i)))) {
            return acc + num
        }
        acc
    });

    println!("{:?}", sum);



    // PART 2
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));
    
    let mut gears: HashSet<(i32, i32)> = HashSet::new();
    let mut nums: HashMap<(i32, i32), Vec<_>> = HashMap::new();

    for (j, line) in lines.into_iter().map(|x| x.unwrap()).enumerate() {
        for (i, _) in line.chars().enumerate().filter(|&(_, ch)| ch == '*') {
            gears.insert((j as i32, i as i32));
        }

        let mut index = 0;
        for part in line.split(|ch: char| !ch.is_ascii_digit()) {
            if let Ok(num) = part.parse::<usize>() {
                for i in (index-1)..=(index+part.len() as i32) {
                    nums.entry((j as i32 - 1, i)).or_default().push(num);
                    nums.entry((j as i32,     i)).or_default().push(num);
                    nums.entry((j as i32 + 1, i)).or_default().push(num);
                }
            }
            index += 1 + part.len() as i32;
        }
    }

    let ratio = gears.into_iter().fold(0, |acc, (j, i)| {
        if let Some(val) = nums.get_mut(&(j, i)) {
            if val.len() == 2 {
                return acc + val[0] * val[1]
            }
        }
        acc
    });
        
    println!("{:?}", ratio);

}