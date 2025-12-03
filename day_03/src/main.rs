static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1: {}", part_1(INPUT.lines()));
    println!("part 2: {}", part_2(INPUT.lines()));
}

fn do_joltage(n: usize, batteries: &[u8]) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    if n > batteries.len() {
        return None;
    }

    let (next_batteries, this_battery) = batteries.split_at(batteries.len() - 1);
    assert_eq!(1, this_battery.len());
    let this_battery = str::from_utf8(&[this_battery[0]])
        .expect("a digit should be UTF-8")
        .parse::<u64>()
        .expect("a digit should be an integer");

    let subproblems = [
        do_joltage(n, next_batteries),
        do_joltage(n - 1, next_batteries).map(|j| j * 10 + this_battery),
    ];

    // Yes, this doesn't need to be unwrapped and then rewrapped, but I'd rather have the diagnostic
    // if it violates my assumption somewhere deep in the recursion tree.
    Some(
        subproblems
            .into_iter()
            .flat_map(Option::into_iter)
            .max()
            .expect("at least one of them should be Some"),
    )
}

fn joltage(batteries: &str) -> u64 {
    do_joltage(2, batteries.as_bytes()).expect("input should be longer than 2 to make this work")
}

fn part_1<'a>(input: impl Iterator<Item = &'a str>) -> u64 {
    input.map(joltage).sum()
}

fn joltage_override(batteries: &str) -> u64 {
    do_joltage(12, batteries.as_bytes()).expect("input should be longer than 12 to make this work")
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
