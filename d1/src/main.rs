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
            let mut alphanumerics = [1, 3, 4, 5]
                .iter()
                .map(|&w| {
                    line
                        .as_bytes()
                        .iter()
                        .copied()
                        .enumerate()
                        .collect::<Vec<_>>()
                        .windows(w)
                        .filter_map(|s| match s {
                            [(_, b'0')] | [(_, b'z'),(_,b'e'),(_,b'r'),(_,b'o')]            => Some((s[0].0, 0)),
                            [(_, b'1')] | [(_, b'o'),(_,b'n'),(_,b'e')]                     => Some((s[0].0, 1)),
                            [(_, b'2')] | [(_, b't'),(_,b'w'),(_,b'o')]                     => Some((s[0].0, 2)),
                            [(_, b'3')] | [(_, b't'),(_,b'h'),(_,b'r'),(_,b'e'),(_,b'e')]   => Some((s[0].0, 3)),
                            [(_, b'4')] | [(_, b'f'),(_,b'o'),(_,b'u'),(_,b'r')]            => Some((s[0].0, 4)),
                            [(_, b'5')] | [(_, b'f'),(_,b'i'),(_,b'v'),(_,b'e')]            => Some((s[0].0, 5)),
                            [(_, b'6')] | [(_, b's'),(_,b'i'),(_,b'x')]                     => Some((s[0].0, 6)),
                            [(_, b'7')] | [(_, b's'),(_,b'e'),(_,b'v'),(_,b'e'),(_,b'n')]   => Some((s[0].0, 7)),
                            [(_, b'8')] | [(_, b'e'),(_,b'i'),(_,b'g'),(_,b'h'),(_,b't')]   => Some((s[0].0, 8)),
                            [(_, b'9')] | [(_, b'n'),(_,b'i'),(_,b'n'),(_,b'e')]            => Some((s[0].0, 9)),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();

            alphanumerics.sort_by(|(a, _), (b, _)| a.cmp(b));
            acc + 10 * alphanumerics.first().unwrap().1 + alphanumerics.last().unwrap().1
        });
    
    println!("{:?}", sum);
}
