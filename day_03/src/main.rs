use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Hello, world!");
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
