static INPUT: &str = include_str!("input.txt");

fn main() {
    let boxes = parse(INPUT);
}

fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let [a, b, c] = line
                .split(',')
                .map(|s| s.parse().expect("input must be an integer"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("input must have exactly 3 components");
            (a, b, c)
        })
        .collect()
}
