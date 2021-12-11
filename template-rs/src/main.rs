use std::io::Result;

#[derive(Debug)]
struct Line {
    line: &'static str,
}

impl Line {
    fn new(line: &'static str) -> Line {
        Line { line }
    }
}

impl Line {
    fn process(&self) -> Result<()> {
        println!("{}", self.line);
        Ok(())
    }
}

fn run(input: &'static str) -> Result<()> {
    input
        .lines()
        .map(|line| Line::new(line).process())
        .collect()
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    run(test_input)?;
    run(input)?;

    Ok(())
}
