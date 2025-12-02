use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|i| {
            let [low, high] = i.split('-').collect::<Vec<_>>().try_into().unwrap();
            (low.parse().unwrap())..=(high.parse().unwrap())
        })
        .collect()
}

fn is_duplicated(id: u64) -> bool {
    let id = id.to_string();
    if id.len() % 2 != 0 {
        return false;
    }

    id[0..(id.len() / 2)] == id[(id.len() / 2)..]
}

fn part_1(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .flatten()
        .filter(|&i| is_duplicated(i))
        .sum()
}

fn is_repeated(id: u64) -> bool {
    let id = id.to_string();
    let id = id.as_bytes();

    for how_many_repeats in 2..id.len() {
        if id.len() % how_many_repeats != 0 {
            // if we can't divide the string evenly then there's simply no hope
            continue;
        }

        let chunks = id
            .chunks(id.len() / how_many_repeats)
            .collect::<Vec<&[u8]>>();
        if chunks[1..].iter().all(|&chunk| chunk == chunks[0]) {
            return true;
        }
    }

    false
}

fn part_2(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .flatten()
        .filter(|&i| is_repeated(i))
        .sum()
}
