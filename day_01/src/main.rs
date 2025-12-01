static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.lines().map(|l| {
        if let Some(s) = l.strip_prefix("L") {
            -1 * s.parse::<i32>().expect("input must be integers")
        } else if let Some(s) = l.strip_prefix("R") {
            s.parse::<i32>().expect("input must be integers")
        } else {
            panic!("input must start with L or R")
        }
    }).collect::<Vec<_>>();

    let mut count_of_zero = 0;
    let mut position = 50;

    for &i in &input {
        position += i;
        position %= 100;

        if position == 0 {
            count_of_zero += 1;
        }
    }

    println!("part 1: {count_of_zero}");

    let mut count_of_zero = 0;
    let mut position = 50;

    for &i in &input {
        // divide toward negative infinity so that even `-1` counts as a "wraparound"
        let wraparounds = (position + i).div_euclid(100);
        count_of_zero += wraparounds.abs();

        position = (position + i).rem_euclid(100);
        assert!(position >= 0);
    }

    println!("part 2: {count_of_zero}");
}
