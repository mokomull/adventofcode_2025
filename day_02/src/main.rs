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

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1, part_2};

    #[test]
    fn example() {
        let input = parse_input(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(1227775554, part_1(&input));
        assert_eq!(4174379265, part_2(&input));
    }
}
