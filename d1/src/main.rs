use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = "input/test";

    // PART 1
    let mut lines = BufReader::new(File::open(file).expect("Error opening file (!)")).lines();
    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let bytes = line.as_bytes().into_iter().copied();
        let numerics = bytes.filter(|ch| ch.is_ascii_digit()).collect::<Vec<_>>();
        let calibration_value = &[numerics[0], numerics[numerics.len() - 1]];
        sum += String::from_utf8_lossy(calibration_value)
            .parse::<i32>()
            .unwrap();
    }
    println!("{:?}", sum);

    // PART 2
    let lines = BufReader::new(File::open(file).expect("Error opening file (!)")).lines();
    let sum = lines
        .into_iter()
        .map(|x| x.unwrap())
        .fold(0, |acc, line| {
            let mut alphanumerics = (1..=5)
                .map(|w| {
                    line
                        .as_bytes()
                        .iter()
                        .copied()
                        .enumerate()
                        .collect::<Vec<_>>()
                        .windows(w)
                        .map(|bytes_w| {
                            (bytes_w[0].0, String::from_utf8(bytes_w.iter().map(|&(_, byte)| byte).collect()).unwrap())
                        })
                        .filter_map(|(i, string_w)| match string_w.as_str() {
                            "0" | "zero"  => Some((i, b'0')),
                            "1" | "one"   => Some((i, b'1')),
                            "2" | "two"   => Some((i, b'2')),
                            "3" | "three" => Some((i, b'3')),
                            "4" | "four"  => Some((i, b'4')),
                            "5" | "five"  => Some((i, b'5')),
                            "6" | "six"   => Some((i, b'6')),
                            "7" | "seven" => Some((i, b'7')),
                            "8" | "eight" => Some((i, b'8')),
                            "9" | "nine"  => Some((i, b'9')),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();

            alphanumerics.sort_by(|(a, _), (b, _)| a.cmp(b));
            acc + String::from_utf8_lossy(&[alphanumerics.first().unwrap().1, alphanumerics.last().unwrap().1]).parse::<i32>().unwrap()
        });
    
    println!("{:?}", sum);
}
