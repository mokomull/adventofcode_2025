use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1: {}", part_1(INPUT.lines()));
    println!("part 2: {}", part_2(INPUT.lines()));
}

fn do_joltage<'a>(n: usize, batteries: &'a [u8]) -> u64 {
    let mut digits = BinaryHeap::new();

    for (i, &d) in batteries.iter().enumerate() {
        // sorted ascending by digit value, and descending by index (i.e. we'll pick the left-most
        // one of a given digit value first)
        digits.push((d, Reverse(i)));
    }

    let mut result = String::new();
    let mut max_i = 0;
    'selected: for _ in 0..n {
        let mut returned = vec![];
        // pick the largest digit for the most-significant figure that we can
        let digit = loop {
            let (d, Reverse(i)) = digits.pop().expect("we're out of possibilities");
            if i < max_i {
                // this digit can never be used, we've already used a digit after it
                continue;
            }

            if i > batteries.len() + result.len() - n {
                // this digit is too far to the right to leave remaining possibilities, but we might
                // use it later so put it back in the pot after we select this digit
                returned.push((d, Reverse(i)));
                continue;
            }

            break d;
        };

        digits.extend(returned);
        result.push(digit as char);
    }

    result
        .parse()
        .expect("pasting multiple digits together should yield an integer")
}

fn joltage(batteries: &str) -> u64 {
    do_joltage(2, batteries.as_bytes())
}

fn part_1<'a>(input: impl Iterator<Item = &'a str>) -> u64 {
    input.map(joltage).sum()
}

fn joltage_override(batteries: &str) -> u64 {
    do_joltage(12, batteries.as_bytes())
}

fn part_2<'a>(input: impl Iterator<Item = &'a str>) -> u64 {
    input.map(joltage_override).sum()
}

#[cfg(test)]
mod tests {
    use crate::{INPUT, joltage, joltage_override, part_1, part_2};

    #[test]
    fn example() {
        assert_eq!(98, joltage("987654321111111"));
        assert_eq!(89, joltage("811111111111119"));
        assert_eq!(78, joltage("234234234234278"));
    }

    #[test]
    fn example_override() {
        assert_eq!(987654321111, joltage_override("987654321111111"));
        assert_eq!(811111111119, joltage_override("811111111111119"));
    }

    #[test]
    fn personal_input() {
        assert_eq!(17207, part_1(INPUT.lines()));
        assert_eq!(170997883706617, part_2(INPUT.lines()));
    }
}
