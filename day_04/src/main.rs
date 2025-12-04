static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'@' => true,
                    b'.' => false,
                    _ => panic!("invalid"),
                })
                .collect()
        })
        .collect()
}
