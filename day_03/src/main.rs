use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1: {}", part_1(INPUT.lines()));
    println!("part 2: {}", part_2(INPUT.lines()));
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

fn joltage_override(batteries: &str) -> u64 {
    batteries
        .as_bytes()
        .iter()
        .tuple_combinations()
        .map(|(&a, &b, &c, &d, &e, &f, &g, &h, &i, &j, &k, &l)| {
            let number = [a, b, c, d, e, f, g, h, i, j, k, l];
            let number =
                str::from_utf8(&number).expect("two digits should still be a valid UTF-8 string");
            number
                .parse::<u64>()
                .expect("two digits should still be an integer")
        })
        .max()
        .unwrap()
}

fn part_2<'a>(input: impl Iterator<Item = &'a str>) -> u64 {
    input.map(joltage_override).sum()
}

#[cfg(test)]
mod tests {
    use crate::{joltage, joltage_override};

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
}
