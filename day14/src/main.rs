use std::collections::HashMap;
use std::io::Result;

#[derive(Debug)]
struct Polymer {
    vocab: HashMap<String, String>,
    init: &'static str,
    current: String,
}

impl Polymer {
    fn new() -> Polymer {
        Polymer {
            init: "",
            current: "".to_string(),
            vocab: HashMap::new(),
        }
    }
}

impl Polymer {
    fn process(&mut self, line: &'static str) {
        if line.len() > 0 {
            let entries = line.split(" -> ").collect::<Vec<&'static str>>();
            if entries.len() == 2 {
                self.vocab
                    .insert(entries[0].to_string(), entries[1].to_string());
            } else {
                self.init = line;
                self.current = line.to_string();
            }
        }
    }

    fn tick(&mut self) {
        self.current
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|cs| cs.iter().collect::<String>())
            .enumerate()
            .rev()
            .for_each(|(i, s)| {
                if let Some(ns) = self.vocab.get(&s) {
                    let p = i + 1;
                    self.current.insert_str(p, ns);
                }
            });
    }

    fn count(&self) -> usize {
        // println!("Template: {}", self.current);
        println!("Len {}", self.current.len());

        let mut counter: HashMap<char, usize> = HashMap::new();
        self.current.chars().for_each(|c| {
            let entry = counter.entry(c).or_insert(0);
            *entry += 1;
        });

        println!("{:?}", counter);

        let values: Vec<usize> = counter.values().map(|v| v.to_owned()).collect();
        let min: usize = *(values.iter().min().unwrap_or(&0));
        let max: usize = *(values.iter().max().unwrap_or(&0));
        let res = max - min;
        println!("{} - {} = {}", max, min, res);

        res
    }
}

fn run(input: &'static str, steps: usize) -> usize {
    let mut polymer = Polymer::new();
    input.lines().for_each(|line| polymer.process(line));
    for _ in 0..steps {
        polymer.tick();
    }
    polymer.count()
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    assert_eq!(run(test_input, 10), 1588);
    assert_eq!(run(input, 10), 2223);

    assert_eq!(run(test_input, 40), 2188189693529);
    assert_eq!(run(input, 40), 1);

    Ok(())
}
