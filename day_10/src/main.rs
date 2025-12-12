use std::{
    collections::{BTreeSet, HashMap},
    iter::repeat_with,
};

use microlp::{ComparisonOp, OptimizationDirection::Minimize, Problem};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let machines = INPUT.lines().map(Machine::from_str).collect::<Vec<_>>();
    println!("part 1: {}", part_1(&machines));
    println!("part 2: {}", part_2(&machines));
}

struct Machine {
    desired_indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_str(line: &str) -> Machine {
        let mut tokens = line.split(' ');

        let indicators = tokens.next().expect("you need to provide indicators");
        let indicators = indicators
            .strip_prefix('[')
            .expect("must start with [")
            .strip_suffix(']')
            .expect("must end with ]");
        let desired_indicators = indicators
            .bytes()
            .map(|b| match b {
                b'.' => false,
                b'#' => true,
                x => panic!("unexpected indicator light {x}"),
            })
            .collect();

        let mut tokens = tokens.peekable();
        let mut buttons = Vec::new();
        while let Some(token) = tokens.next_if(|&token| token.starts_with('(')) {
            let token = token
                .strip_prefix('(')
                .expect("button must start with (")
                .strip_suffix(')')
                .expect("button must end with )");
            let button = token
                .split(',')
                .map(|s| s.parse().expect("input must be an integer"))
                .collect();
            buttons.push(button);
        }

        let token = tokens.next().expect("should also have joltages");
        let token = token
            .strip_prefix('{')
            .expect("joltage should start with {")
            .strip_suffix('}')
            .expect("joltage should end with }");
        let joltage = token
            .split(',')
            .map(|s| s.parse().expect("joltage should be an integer"))
            .collect();

        Self {
            desired_indicators,
            buttons,
            joltage,
        }
    }

    fn part_1(&self) -> usize {
        let mut results = HashMap::from([(vec![false; self.desired_indicators.len()], 0)]);

        let mut queue = BTreeSet::from([vec![false; self.desired_indicators.len()]]);
        while let Some(this) = queue.pop_first() {
            let next_path = results[&this] + 1;
            for (button) in self.buttons.iter() {
                let mut next_indicators = this.clone();
                for &i in button {
                    next_indicators[i] ^= true;
                }

                if results.get(&next_indicators).cloned().unwrap_or(usize::MAX) > next_path {
                    results.insert(next_indicators.clone(), next_path);
                    queue.insert(next_indicators);
                }
            }
        }

        results[&self.desired_indicators]
    }

    fn part_2(&self) -> usize {
        let mut problem = Problem::new(Minimize);

        let variables = repeat_with(|| problem.add_integer_var(1.0, (0, i32::MAX)))
            .take(self.buttons.len())
            .collect::<Vec<_>>();

        for (i, &joltage) in self.joltage.iter().enumerate() {
            let expr =
                self.buttons
                    .iter()
                    .zip(variables.iter())
                    .filter_map(|(button, &variable)| {
                        if button.contains(&i) {
                            Some((variable, 1.0))
                        } else {
                            None
                        }
                    });
            problem.add_constraint(expr, ComparisonOp::Eq, joltage as f64);
        }

        problem.solve().unwrap().objective().round() as usize
    }
}

fn part_1(machines: &[Machine]) -> u64 {
    machines.iter().map(Machine::part_1).map(|x| x as u64).sum()
}

fn part_2(machines: &[Machine]) -> u64 {
    machines.iter().map(Machine::part_2).map(|x| x as u64).sum()
}

#[cfg(test)]
mod tests {
    use crate::Machine;

    #[test]
    fn example() {
        let machine =
            Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(2, machine.part_1());
        assert_eq!(11, machine.part_2());
    }
}
