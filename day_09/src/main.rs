use std::cmp::{Reverse, max, min};

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

    let mut vertical_lines = red_squares
        .iter()
        .chain([&red_squares[0]])
        .tuple_windows()
        .filter_map(|(&a, &b)| {
            if a.0 == b.0 {
                Some((a.0, min(a.1, b.1)..=max(a.1, b.1)))
            } else if a.1 == b.1 {
                None
            } else {
                panic!("not vertical or horizontal line")
            }
        })
        .collect_vec();
    vertical_lines.sort_unstable_by_key(|(x, _ys)| *x);

    let is_inside_polygon = move |(x, y)| -> bool {
        // cast a ray horizontally from =(-infinity, y) to (x, y) ... because of this, we could just
        // deal with only the vertical lines.
        let mut is_inside = false;
        for (line_x, line_ys) in &vertical_lines {
            if !line_ys.contains(&y) {
                // this vertical line is somewhere else on the map
                continue;
            }

            // by definition, if (x, y) lands directly on a line, then it is "inside" the polygon
            if x == *line_x {
                return true;
            }

            if *line_x > x {
                // we've iterated through all of the lines at and to the left of this
                return is_inside;
            }

            is_inside = !is_inside;
        }

        assert!(
            !is_inside,
            "we passed clear through the polygon but we think we're inside"
        );
        is_inside
    };

    areas
        .into_iter()
        .filter_map(|((x1, y1), (x2, y2), area)| {
            // Since the loop should not form a polygon with holes in it (i.e. it is "simple") we
            // should be able to get away with walking the perimeter of the rectangle and using the
            // ray casting algorithm from
            // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm.

            let xs = (min(x1, x2))..=max(x1, x2);
            let ys = (min(y1, y2))..=max(y1, y2);

            for x in xs {
                if !is_inside_polygon((x, y1)) || !is_inside_polygon((x, y2)) {
                    return None;
                }
            }

            for y in ys {
                if !is_inside_polygon((x1, y)) || !is_inside_polygon((x2, y)) {
                    return None;
                }
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
