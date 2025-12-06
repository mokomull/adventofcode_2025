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
    println!("part 2: {}", part_2(INPUT))
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

fn part_2(input: &str) -> u64 {
    let data = input.lines().collect::<Vec<&str>>();
    assert!(data.iter().all(|l| l.len() == data[0].len()));

    let mut result = 0;
    let mut this_number: Option<String> = None;
    let mut operands = vec![];
    for i in (0..data[0].len()).rev() {
        for &line in &data {
            match line.as_bytes()[i] {
                c @ b'0'..=b'9' => {
                    this_number.get_or_insert_default().push(c as char);
                    continue;
                }
                _ => {
                    if let Some(s) = this_number {
                        operands.push(s.parse::<u64>().expect("should be an integer"));
                        this_number = None;
                    }
                }
            }

            match line.as_bytes()[i] {
                b' ' => continue,
                b'*' => result += std::mem::take(&mut operands).iter().product::<u64>(),
                b'+' => result += std::mem::take(&mut operands).iter().sum::<u64>(),
                c => panic!("unexpected character {c:?}"),
            }
        }
    }

    result
}
