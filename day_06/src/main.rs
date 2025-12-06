use std::num::ParseIntError;

static INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

fn main() {
    let (numbers, operations) = parse(INPUT);
    println!("part 1: {}", part_1(&numbers, &operations));
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

fn part_1(numbers: &[Vec<u64>], operations: &[Operation]) -> u64 {
    operations
        .iter()
        .enumerate()
        .map(|(i, &op)| {
            let xs = numbers.iter().map(|x| x[i]);
            match op {
                Operation::Add => xs.sum::<u64>(),
                Operation::Multiply => xs.product(),
            }
        })
        .sum()
}
