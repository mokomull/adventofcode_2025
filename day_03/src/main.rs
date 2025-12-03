use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1: {}", part_1(INPUT.lines()));
}

fn joltage(batteries: &str) -> u64 {
    batteries
        .as_bytes()
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| {
            let number = [a, b];
            let number =
                str::from_utf8(&number).expect("two digits should still be a valid UTF-8 string");
            number
                .parse::<u64>()
                .expect("two digits should still be an integer")
        })
        .max()
        .unwrap()
}

fn part_1<'a>(input: impl Iterator<Item = &'a str>) -> u64 {
    input.map(joltage).sum()
}

#[cfg(test)]
mod tests {
    use crate::joltage;

    #[test]
    fn example() {
        assert_eq!(98, joltage("987654321111111"));
        assert_eq!(89, joltage("811111111111119"));
        assert_eq!(78, joltage("234234234234278"));
    }
}
