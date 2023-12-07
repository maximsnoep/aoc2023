use itertools::Itertools;

fn main() {
    let file = "input/test";

    // five of a kind  => 1 distinct card,  5         -> 5^2 = 32
    // four of a kind  => 2 distinct cards, 4,1       -> 4^2 + 1 = 17
    // full house      => 2 distinct cards, 3,2       -> 3^2 + 2^2 = 9 + 4 = 13
    // three of a kind => 3 distinct cards  3,1,1     -> 3^2 + 1 + 1 = 11
    // two pair        => 3 distinct cards  2,2,1     -> 2^2 + 2^2 + 1 = 9
    // one pair        => 4 distinct cards  2,1,1,1   -> 2^2 + 1 + 1 + 1 = 7
    // high card       => 5 distinct cards  1,1,1,1,1 -> 1 + 1 + 1 + 1 + 1 = 5

    // PART 1
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let winnings = lines
        .map(|x| x.unwrap())
        .map(|line| {
            let (hand_s, bid_s) = line.split_once(' ').unwrap();
            let hand = hand_s
                .chars()
                .map(|ch| match ch {
                    'A' => 12u8,
                    'K' => 11u8,
                    'Q' => 10u8,
                    'J' => 9u8,
                    'T' => 8u8,
                    _ => ch as u8 - 50u8,
                })
                .collect::<Vec<_>>();

            let strength = hand
                .iter()
                .into_group_map_by(|&x| x)
                .iter()
                .map(|(_, v)| v.len().pow(2) as u8)
                .sum();

            (
                u64::from_be_bytes([0, 0, strength, hand[0], hand[1], hand[2], hand[3], hand[4]]),
                bid_s.parse::<u32>().unwrap(),
            )
        })
        .sorted_by_key(|x| x.0)
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank as u32 + 1) * bid);

    println!("{winnings:?}");

    assert!(winnings == 254024898);

    // PART 2
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let winnings = lines
        .map(|x| x.unwrap())
        .map(|line| {
            let (hand_s, bid_s) = line.split_once(' ').unwrap();
            let hand = hand_s
                .chars()
                .map(|ch| match ch {
                    'A' => 12u8,
                    'K' => 11u8,
                    'Q' => 10u8,
                    'J' => 0u8,
                    'T' => 9u8,
                    _ => ch as u8 - 49u8,
                })
                .collect::<Vec<_>>();

            let strength = hand
                .iter()
                .into_group_map_by(|&x| x)
                .into_iter()
                .map(|(&k, v)| if k == 0u8 { (k, vec![]) } else { (k, v) })
                .sorted_by_key(|x| -(x.1.len() as i8))
                .enumerate()
                .map(|(i, (_, v))| {
                    if i == 0 {
                        ((v.len() + hand.iter().filter(|&&c| c == 0u8).count()) as u8).pow(2)
                    } else {
                        (v.len() as u8).pow(2)
                    }
                })
                .sum();

            (
                u64::from_be_bytes([0, 0, strength, hand[0], hand[1], hand[2], hand[3], hand[4]]),
                bid_s.parse::<u32>().unwrap(),
            )
        })
        .sorted_by_key(|x| x.0)
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank as u32 + 1) * bid);

    println!("{winnings:?}");

    assert!(winnings == 254115617);
}
