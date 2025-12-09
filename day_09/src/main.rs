use std::{
    cmp::{Reverse, max, min},
    collections::HashSet,
};

use itertools::Itertools;

use Handedness::*;

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
            ((x1 as i64 - x2 as i64).unsigned_abs() + 1)
                * ((y1 as i64 - y2 as i64).unsigned_abs() + 1)
        })
        .max()
        .expect("there was at least one pair of squares")
}

fn part_2(red_squares: &[(u32, u32)]) -> u64 {
    let mut areas = red_squares
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| {
            (
                (x1, y1),
                (x2, y2),
                ((x1 as i64 - x2 as i64).unsigned_abs() + 1)
                    * ((y1 as i64 - y2 as i64).unsigned_abs() + 1),
            )
        })
        .collect_vec();
    areas.sort_unstable_by_key(|&(_corner_a, _corner_b, area)| Reverse(area));

    areas
        .into_iter()
        .filter_map(|((x1, y1), (x2, y2), area)| {
            // For this rectangle to be fully on red-or-green squares:
            //   * at least one of the corners must be inside the red-or-green polygon (I will chose
            //     (x1, y1)), and
            //   * none of the line segments of the loop may pass through the inside of this
            //     rectangle.  Since the borders of the loop are considered OK, the loop may touch
            //     the borders of this rectangle -- it is strictly the inside that we care about.

            // Iterate over the corners of the loop.  This is the corner at point `b`, and treating
            // it for handedness as if it were from `a` via `b` to `c`.
            for (&a, &b, &c) in red_squares
                .iter()
                .chain([&red_squares[0]].into_iter())
                .tuple_windows()
            {
                // If (x1, y1) is outside the polygon, then for some corner it will be on different
                // sides of the a->b line than the b->c line.  I don't actually have a proof for
                // this, but it feels like the sort of trick that would work.
            }

            Some(area)
        })
        .next()
        .expect("no rectangles that are filled with only red and green")
}

enum Handedness {
    OnTheLine,
    Left,
    Right,
}

impl Handedness {
    fn compute(a: (usize, usize), b: (usize, usize), x: (usize, usize)) -> Handedness {
        if a.0 == b.0 {
            if a.1 > b.1 {
                if x.0 == a.0 && (b.1..=a.1).contains(&x.1) {
                    return OnTheLine;
                }
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

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
        assert_eq!(24, part_2(&red_squares));
    }
}
