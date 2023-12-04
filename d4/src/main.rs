use itertools::Itertools;

fn main() {
    let file = "input/test";

    // PART 1
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let result = lines
        .into_iter()
        .map(|x| x.unwrap())
        .fold(0, |acc, line| {
            let (win, our) = line.strip_prefix("Card ").unwrap().split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let mult = win.split_whitespace().filter(|x| our.split_whitespace().contains(x)).count() as u32;
            if mult == 0 { return acc } else { return acc + 2u32.pow(mult-1) }
        });

    println!("{:?}", result);



    // PART 2
    let lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let wins = lines
        .into_iter()
        .map(|x| x.unwrap())
        .map(|line| {
            let (win, our) = line.strip_prefix("Card ").unwrap().split_once(": ").unwrap().1.split_once(" | ").unwrap();
            win.split_whitespace().filter(|x| our.split_whitespace().contains(x)).count()
        }).collect::<Vec<_>>();

    let mut sum = 0;
    let mut copies = vec![1; wins.len()];
    loop {
        let n = copies.iter().sum::<i32>();
        if n == 0 { break; }
        sum += n;

        let mut new = vec![0; wins.len()];
        for (card_i, copies) in copies.iter().enumerate() {
            for card_j in (card_i+1)..=(card_i+wins[card_i]) {
                new[card_j] += copies;
            } 
        }
        copies = new;
    }

    println!("{}", sum)

}