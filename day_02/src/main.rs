use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);
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
