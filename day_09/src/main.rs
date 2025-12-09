use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let red_squares = parse(INPUT);
    println!("part 1: {}", part_1(&red_squares));
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            let [a, b] = l
                .split(',')
                .map(|s| s.parse().expect("input must be an integer"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("input must be pairs");
            (a, b)
        })
        .collect()
}

fn part_1(red_squares: &[(u32, u32)]) -> u64 {
    red_squares
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| {
            (x1 as i64 - x2 as i64).unsigned_abs() * (y1 as i64 - y2 as i64).unsigned_abs()
        })
        .max()
        .expect("there was at least one pair of squares")
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1};

    #[test]
    fn example() {
        let red_squares = parse(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(50, part_1(&red_squares));
    }
}
