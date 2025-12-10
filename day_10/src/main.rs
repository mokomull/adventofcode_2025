static INPUT: &str = include_str!("input.txt");

fn main() {
    let machines = INPUT.lines().map(Machine::from_str).collect::<Vec<_>>();
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
}
