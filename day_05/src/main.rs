use std::collections::BTreeMap;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let (fresh, ingredients) = parse(INPUT);
    println!("part 1: {}", part_1(&fresh, &ingredients));
}

fn parse(input: &str) -> (BTreeMap<u64, u64>, Vec<u64>) {
    let mut fresh = BTreeMap::new();
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

        fresh.insert(first, last);
    }

    for l in &mut lines {
        let i = l.parse().expect("ingredients must be integers");
        ingredients.push(i);
    }

    (fresh, ingredients)
}

fn part_1(fresh: &BTreeMap<u64, u64>, ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|&&i| {
            let Some((&from, &to)) = fresh.range(..=i).next() else {
                return false;
            };

            (from..=to).contains(&i)
        })
        .count()
}
