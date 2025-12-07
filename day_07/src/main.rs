use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("input.txt");

enum Cell {
    Empty,
    Splitter,
}

fn main() {
    let (start, cells) = parse(INPUT);
    println!("part 1: {}", part_1(start, &cells));
    println!("part 2: {}", part_2(start, &cells));
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

fn part_2(start: (usize, usize), cells: &[Vec<Cell>]) -> u64 {
    let (start_i, start_j) = start;

    let mut beams = HashMap::from([(start_j, 1)]);

    for line in cells.iter().skip(start_i) {
        let mut next_beams = HashMap::new();

        for (beam_j, count) in beams {
            match line[beam_j] {
                Cell::Empty => {
                    next_beams.insert(beam_j, count);
                }
                Cell::Splitter => {
                    if beam_j > 0 {
                        *next_beams.entry(beam_j - 1).or_default() += count;
                    }
                    if beam_j < line.len() - 1 {
                        *next_beams.entry(beam_j + 1).or_default() += count;
                    }
                }
            }
        }

        beams = next_beams;
    }

    beams.into_values().sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

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
        assert_eq!(40, part_2(start, &cells));
    }
}
