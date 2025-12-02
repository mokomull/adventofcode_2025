use std::ops::{RangeBounds, RangeInclusive};

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

fn sum_of_replicated_digitstrings<R>(ids: RangeInclusive<u64>, repeats: R) -> u64
where
    R: RangeBounds<usize>,
    R: IntoIterator<Item = usize>,
{
    let start = ids.start().to_string();
    let end = ids.end().to_string();
    repeats
        .into_iter()
        .take_while(|&n| n < end.len())
        .map(|n| -> u64 {
            (start.len()..=end.len())
                .map(|target_length| -> u64 {
                    if target_length % n != 0 {
                        return 0;
                    }

                    let partial_start = start[..(target_length / n)]
                        .parse::<u64>()
                        .expect("a truncated integer should still be an integer");

                    (partial_start..)
                        .map(|partial| {
                            // take the first digits and glue them together n times
                            let mut id = String::new();
                            let partial = partial.to_string();

                            for _ in 0..n {
                                id.push_str(&partial);
                            }

                            id.parse::<u64>().expect(
                                "repeating a substring of digits n times should still fit in a u64",
                            )
                        })
                        // and stop once we've completely exhausted the input range
                        .take_while(|id| ids.contains(id))
                        .sum()
                })
                .sum()
        })
        .sum()
}

fn part_1(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .map(|ids| sum_of_replicated_digitstrings(ids, 2..=2))
        .sum()
}

fn part_2(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .map(|ids| sum_of_replicated_digitstrings(ids, 2..))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{INPUT, parse_input, part_1, part_2};

    #[test]
    fn example() {
        let input = parse_input(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(1227775554, part_1(&input));
        assert_eq!(4174379265, part_2(&input));
    }

    #[test]
    fn input() {
        let input = parse_input(INPUT);
        assert_eq!(19605500130, part_1(&input));
        assert_eq!(36862281418, part_2(&input));
    }
}
