#[derive(Debug)]
struct State {}

impl State {
    fn new() -> State {
        State {}
    }
}

impl State {
    fn process(&self, line: &'static str) {
        println!("{}", line);
    }
}

fn run(input: &'static str) {
    let state = State::new();
    input.lines().for_each(|line| state.process(line));
}

fn main() {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    run(test_input);
    run(input);
}
