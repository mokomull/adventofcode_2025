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

#[cfg(test)]
mod tests {
    use crate::parse;

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
    }
}
