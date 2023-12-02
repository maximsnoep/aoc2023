use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = "input/test";

    // PART 1
    let lines = BufReader::new(File::open(file).expect("Error opening file (!)")).lines();

    let result = lines
        .into_iter()
        .map(|x| x.unwrap())
        .fold(0, |acc, line| {
            let (i, cubes) = line.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
            if cubes.split("; ").map(|round| round.split(", ")).flatten().all(|grab| {
                if let Some((q, c)) = grab.split_once(" ") {
                    if let Ok(q_i32) = q.parse::<i32>() {
                        return !(q_i32 > 12 && c.len() == 3 || q_i32 > 13 && c.len() == 5 || q_i32 > 14 && c.len() == 4)
                    }
                }
                false
            }) {
                return acc + i.parse::<usize>().unwrap()
            }
            acc            
        });

    println!("{:?}", result);



    // PART 2
    let lines = BufReader::new(File::open(file).expect("Error opening file (!)")).lines();

    let result = lines
        .into_iter()
        .map(|x| x.unwrap())
        .fold(0, |acc, line| {
            let (i, cubes) = line.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
            let (tr,tg,tb) = cubes.split("; ").map(|round| round.split(", ")).flatten().fold((0,0,0), |(r,g,b), grab| {
                if let Some((q, c)) = grab.split_once(" ") {
                    if let Ok(q_i32) = q.parse::<i32>() {
                        return match c.len() {
                            3 => (std::cmp::max(r,q_i32),g,b), // red
                            5 => (r,std::cmp::max(g,q_i32),b), // green
                            4 => (r,g,std::cmp::max(b,q_i32)), // blue 
                            _ => (0,0,0), // :^)
                        }
                    }
                }
                (0,0,0)
            }); 
            acc + (tr*tg*tb)
        });

    println!("{:?}", result);
    

}
