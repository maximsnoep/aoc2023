fn main() {
    let file = "input/test";

    // PART 1
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));
    dbg!(lines.next().unwrap().unwrap().split_whitespace().flat_map(|seed| seed.parse::<u64>()).zip(lines.next().unwrap().unwrap().split_whitespace().flat_map(|seed| seed.parse::<u64>())).fold(1, |acc, (t, r)| acc * (1..t).filter(|s| (t-s)*s > r).count()));

    // PART 2
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));
    let t = lines.next().unwrap().unwrap().chars().filter(|ch| ch.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap();
    let r = lines.next().unwrap().unwrap().chars().filter(|ch| ch.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap();
    dbg!(t - 1 - ((1..t).rev().position(|s| (t-s)*s > r).unwrap() + (1..t).position(|s| (t-s)*s > r).unwrap()) as u64);
    
}
