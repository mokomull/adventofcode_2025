use std::num::ParseIntError;

static INPUT: &str = include_str!("input.txt");

enum Operation {
    Add,
    Multiply,
}

fn main() {
    let (numbers, operations) = parse(INPUT);
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut numbers = vec![];

    for line in input.lines() {
        if let Ok(vs) = line
            .split(' ')
            .filter(|word| !word.is_empty())
            .map(|word| word.parse())
            .collect::<Result<Vec<u64>, ParseIntError>>()
        {
            numbers.push(vs)
        } else {
            return (
                numbers,
                line.split(' ')
                    .filter(|word| !word.is_empty())
                    .map(|word| match word {
                        "+" => Operation::Add,
                        "*" => Operation::Multiply,
                        x => panic!("unexpected word {x:?}"),
                    })
                    .collect(),
            );
        }
    }

    panic!("never saw the operations")
}
