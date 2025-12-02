use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    println!("part 1: {}", part_1(&input));
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
