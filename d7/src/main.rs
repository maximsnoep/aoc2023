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
        let hand = hand_s.chars().map(|ch| match ch {
            'T' => 8u32, 'J' => 9u32, 'Q' => 10u32, 'K' => 11u32, 'A' => 12u32, _ => ch as u32 - 50u32
        }).collect::<Vec<_>>();
        let bid = bid_s.parse::<u32>().unwrap();
        
        let strength = hand.iter().into_group_map_by(|&x| x).iter().map(|(_, v)| v.len().pow(2u32)).sum::<usize>() as f64;
        let strength2 = hand[0]as f64/120f64 + hand[1]as f64/12000f64 + hand[2]as f64/1200000f64 + hand[3]as f64/120000000f64 + hand[4]as f64/12000000000f64;

        (strength + strength2, bid)
     })
     .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap() )
     .enumerate()
     .fold(0, |acc, (rank, (_, bid))| { acc + (rank as u32 + 1) * bid });

    println!("{winnings:?}");



    // PART 2
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));
    
    let winnings = lines.map(|x| x.unwrap())
    .map(|line| {
        let (hand_s, bid_s) = line.split_once(' ').unwrap();
        let hand = hand_s.chars().map(|ch| match ch {
            'T' => 9u32, 'J' => 0u32, 'Q' => 10u32, 'K' => 11u32, 'A' => 12u32, _ => ch as u32 - 49u32
        }).collect::<Vec<_>>();
        let bid = bid_s.parse::<u32>().unwrap();

        let jokers = hand.iter().filter(|&&c| c==0u32).count();        

        let strength = hand.iter().into_group_map_by(|&x| x).into_iter().sorted_by_key(|k| if *k.0 == 0u32 { 0 } else { k.1.len() }).rev().enumerate().map(|(i, (k, v))| if *k == 0u32 { if jokers==5 { (jokers).pow(2u32) } else { 0 } } else if i == 0 { (v.len() + jokers).pow(2u32) } else { v.len().pow(2u32) }).sum::<usize>() as f64;
        let strength2 = hand[0]as f64/120f64 + hand[1]as f64/12000f64 + hand[2]as f64/1200000f64 + hand[3]as f64/120000000f64 + hand[4]as f64/12000000000f64;

        (strength + strength2, bid)
    })
    .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap() )
    .enumerate().fold(0, |acc, (rank, (_, bid))| {
        acc + (rank as u32 + 1) * bid
    });
 
    println!("{winnings:?}");
    
}
