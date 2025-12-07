use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

enum Cell {
    Empty,
    Splitter,
}

fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> ((usize, usize), Vec<Vec<Cell>>) {
    let mut start = None;

    let cells = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &c)| match c {
                    b'S' => {
                        assert!(start.is_none(), "We can't have two starting positions");
                        start = Some((i, j));
                        Cell::Empty
                    }
                    b'.' => Cell::Empty,
                    b'^' => Cell::Splitter,
                    x => panic!("unexpected item {x:?} in the bagging area"),
                })
                .collect()
        })
        .collect();

    (start.expect("we must have gotten one starting cell"), cells)
}

fn part_1(start: (usize, usize), cells: &[Vec<Cell>]) -> u64 {
    let (start_i, start_j) = start;

    let mut splits = 0;
    let mut beams = HashSet::from([start_j]);

    for line in cells.iter().skip(start_i) {
        let mut next_beams = HashSet::new();

        for beam in beams {
            match line[beam] {
                Cell::Empty => {
                    next_beams.insert(beam);
                }
                Cell::Splitter => {
                    splits += 1;

                    if beam > 0 {
                        next_beams.insert(beam - 1);
                    }
                    if beam < line.len() - 1 {
                        next_beams.insert(beam + 1);
                    }
                }
            }
        }

        beams = next_beams;
    }

    splits
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1};

    #[test]
    fn example() {
        let (start, cells) = parse(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert_eq!(21, part_1(start, &cells));
    }
}
