use std::cmp;
use std::io::Result;

type V2 = (usize, usize);

#[derive(Debug)]
struct Field {
    field: Vec<Vec<bool>>,
    size_x: usize,
    size_y: usize,
}

impl Field {
    fn new(size_x: usize, size_y: usize) -> Field {
        let field = vec![vec![false; size_y]; size_x];
        Field {
            field,
            size_x,
            size_y,
        }
    }
}

impl Field {
    fn add(&mut self, line: &'static str) {
        let nums: Vec<usize> = line
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        if nums.len() == 2 {
            self.field[nums[0]][nums[1]] = true
        }
    }

    fn fold(&mut self, line: &'static str) {
        let input: Vec<_> = line.split("=").collect();

        if input.len() == 2 {
            let axis = input[0].chars().last().unwrap();
            let v = input[1].parse::<usize>().unwrap();
            if axis == 'x' {
                let old_x = self.size_x;
                self.size_x = v;
                for x in 0..self.size_x {
                    for y in 0..self.size_y {
                        self.field[x][y] = self.field[x][y] || self.field[old_x - x - 1][y]
                    }
                }
            } else {
                let old_y = self.size_y;
                self.size_y = v;
                for x in 0..self.size_x {
                    for y in 0..self.size_y {
                        self.field[x][y] = self.field[x][y] || self.field[x][old_y - y - 1]
                    }
                }
            }
        }
    }

    fn process(&mut self, line: &'static str, print: bool) {
        if line.is_empty() {
            return;
        }

        if line.starts_with("fold along") {
            self.fold(line);
            println!("Count {}", self.count());
        } else {
            self.add(line);
        }

        if print {
            println!("");
            self.print();
        }
    }

    fn print(&self) {
        for y in 0..self.size_y {
            println!("");
            for x in 0..self.size_x {
                let v = if self.field[x][y] { "#" } else { "." };
                print!("{}", v);
            }
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;

        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if self.field[x][y] {
                    count += 1;
                }
            }
        }

        count
    }
}

fn run(input: &'static str, print: bool) -> Result<()> {
    let mut max_x = 0;
    let mut max_y = 0;

    input.lines().for_each(|line| {
        let ns = line
            .split(",")
            .map(|s| s.parse::<usize>().ok())
            .collect::<Option<Vec<usize>>>();

        if let Some(nums) = ns {
            max_x = cmp::max(max_x, nums[0] + 1);
            max_y = cmp::max(max_y, nums[1] + 1);
        }
    });

    let mut field = Field::new(max_x, max_y);
    input.lines().for_each(|line| field.process(line, print));
    field.print();

    Ok(())
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    run(test_input, true)?;
    run(input, false)?;

    Ok(())
}
