use std::{
    cmp::{Reverse, max, min},
    collections::HashSet,
};

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let red_squares = parse(INPUT);
    println!("part 1: {}", part_1(&red_squares));
    println!("part 2: {}", part_2(&red_squares));
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

            let inside_xs = (min(x1, x2) + 1)..max(x1, x2);
            let inside_ys = (min(y1, y2) + 1)..max(y1, y2);

            for (&a, &b) in red_squares
                .iter()
                .chain([&red_squares[0]].into_iter())
                .tuple_windows()
            {
                // "none of the line segments of the loop may pass through the inside of this
                // rectangle" -- this one is easier to start with
                if a.0 == b.0 {
                    if inside_xs.contains(&a.0) {
                        let line_ys = min(a.1, b.1)..=max(a.1, b.1);
                        if line_ys.contains(&inside_ys.start)
                            || line_ys.contains(&inside_ys.end)
                            || inside_ys.contains(&a.1)
                            || inside_ys.contains(&b.1)
                        {
                            return None;
                        }
                    }
                } else if a.1 == b.1 {
                    if inside_ys.contains(&a.1) {
                        let line_xs = min(a.0, b.0)..=max(a.0, b.0);
                        if line_xs.contains(&inside_xs.start)
                            || line_xs.contains(&inside_xs.end)
                            || inside_xs.contains(&a.0)
                            || inside_xs.contains(&b.0)
                        {
                            return None;
                        }
                    }
                } else {
                    panic!("not a vertical or horizontal line")
                }

                // TODO: implement the Ray casting algorithm from
                // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
            }

            Some(area)
        })
        .next()
        .expect("no rectangles that are filled with only red and green")
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
