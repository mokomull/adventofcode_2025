use std::{collections::BTreeMap, ops::RangeInclusive};

type RangeSet = range_set::RangeSet<[RangeInclusive<u64>; 0]>;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let (fresh, ingredients) = parse(INPUT);
    println!("part 1: {}", part_1(&fresh, &ingredients));
}

fn parse(input: &str) -> (RangeSet, Vec<u64>) {
    let mut fresh = RangeSet::new();
    let mut ingredients = vec![];

    let mut lines = input.lines();
    for l in &mut lines {
        if l.is_empty() {
            break;
        }

        let [first, last] = l
            .split('-')
            .map(|s| s.parse::<u64>().expect("components must be integers"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("there should be exactly two components to a range");

        fresh.insert_range(first..=last);
    }

    for l in &mut lines {
        let i = l.parse().expect("ingredients must be integers");
        ingredients.push(i);
    }

    (fresh, ingredients)
}

fn part_1(fresh: &RangeSet, ingredients: &[u64]) -> usize {
    ingredients.iter().filter(|&&i| fresh.contains(i)).count()
}
