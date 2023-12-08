fn encode(s: &str) -> usize {
    let mut bits = s.chars().map(|ch| ch as u8 -'A' as u8);
    usize::from_be_bytes([0, 0, 0, 0, 0, bits.next().unwrap(), bits.next().unwrap(), bits.next().unwrap()])
}

fn main() {
    let file = "input/test";

    // PART 1
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let mut map = vec![[0usize, 0usize]; 1644826];
    let instructions = lines.next().unwrap().unwrap().chars().map(|ch| match ch { 'L' => 0usize, 'R' => 1usize, _ => unreachable!() }).collect::<Vec<_>>();

    lines.map(|x| x.unwrap()).for_each(|line| {
        if let Some((node, lr)) = line.split_once(" = ") {
            if let Some((l, r)) = lr.split_once(", ") {
                map[encode(node) as usize] = [encode(&l[1..=3]), encode(&r[0..=2])];
            }
        }
    });

    let mut cur = encode("AAA");
    let mut iter = 0;
    while cur != encode("ZZZ") {
        cur = map[cur as usize][instructions[iter % instructions.len()]];
        iter += 1;
    }
    dbg!(iter);


    // PART 2
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).expect("Error opening file (!)")));

    let mut map = vec![[0usize, 0usize]; 1644826];
    let mut starts = vec![];
    let instructions = lines.next().unwrap().unwrap().chars().map(|ch| match ch { 'L' => 0usize, 'R' => 1usize, _ => unreachable!() }).collect::<Vec<_>>();

    lines.map(|x| x.unwrap()).for_each(|line| {
        if let Some((node, lr)) = line.split_once(" = ") {
            if let Some((l, r)) = lr.split_once(", ") {
                if node.chars().nth(2) == Some('A') { starts.push(encode(node)); }
                map[encode(node) as usize] = [encode(&l[1..=3]), encode(&r[0..=2])];
            }
        }
    });

    let lcm = starts.iter().fold(1, |acc, &start| {
        let mut cur = start;
        let mut iter = 0;
        while cur.to_be_bytes()[7] != 25u8 {
            cur = map[cur][instructions[iter % instructions.len()]];
            iter += 1;
        }
        num::integer::lcm(acc, iter)
    });

    dbg!(lcm);

}
