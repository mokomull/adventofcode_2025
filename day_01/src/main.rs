static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            if let Some(s) = l.strip_prefix("L") {
                -s.parse::<i32>().expect("input must be integers")
            } else if let Some(s) = l.strip_prefix("R") {
                s.parse::<i32>().expect("input must be integers")
            } else {
                panic!("input must start with L or R")
            }
        })
        .collect()
}

fn part_1(input: &[i32]) -> usize {
    let mut count_of_zero = 0;
    let mut position = 50;

    for &i in input {
        position += i;
        position %= 100;

        if position == 0 {
            count_of_zero += 1;
        }
    }

    count_of_zero
}

fn part_2(input: &[i32]) -> usize {
    let mut count_of_zero = 0;
    let mut position = 50;

    for &i in input {
        // full rotations in either direction, rounded down
        count_of_zero += (i.abs() as usize) / 100;

        let new_position = (position + i).rem_euclid(100);

        // and if we're ending up on the wrong side of the previous position, then we must have
        // passed zero an additional time.  if we end on zero, then definitely count that because we
        // might be going back to the right without wrapping around.  and if we started on zero,
        // then don't count any rollover, since we already counted that last iteration
        if position != 0 && ((new_position - position).signum() != i.signum() || new_position == 0)
        {
            count_of_zero += 1;
        }

        position = new_position;
    }

    count_of_zero
}

#[cfg(test)]
mod tests {
    use crate::{INPUT, parse_input, part_1, part_2};

    #[test]
    fn personal_input() {
        let input = parse_input(INPUT);
        assert_eq!(1118, part_1(&input));
        assert_eq!(6289, part_2(&input));
    }
}
