use std::collections::HashMap;
use std::io::Result;

#[derive(Debug)]
struct Polymer {
    vocab: HashMap<String, String>,
    pairs: HashMap<String, usize>,
    counter: HashMap<char, usize>,
}

impl Polymer {
    fn new() -> Polymer {
        Polymer {
            vocab: HashMap::new(),
            pairs: HashMap::new(),
            counter: HashMap::new(),
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
                line.to_string()
                    .chars()
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|s| {
                        self.inc(s.to_string(), 1);
                    });

                let mut pairs = self.pairs.clone();
                line.to_string()
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(2)
                    .map(|cs| cs.iter().collect::<String>())
                    .for_each(|s| {
                        self.ins(&mut pairs, s, 1);
                    });
                self.pairs = pairs;
            }
        }
    }

    fn ins(&mut self, pairs: &mut HashMap<String, usize>, pair: String, v: usize) {
        println!("Inserting {}", pair);
        let entry = pairs.entry(pair).or_insert(0);
        *entry += v;
    }

    fn rem(&mut self, pairs: &mut HashMap<String, usize>, pair: String, v: usize) {
        println!("Removing {}", pair);
        let entry = pairs.entry(pair).or_insert(0);
        if *entry > 0 {
            *entry -= v;
        }
    }

    fn inc(&mut self, c: String, v: usize) {
        // println!("Inc {} by {}", c, v);
        let entry = self
            .counter
            .entry(c.chars().collect::<Vec<_>>().first().unwrap().to_owned())
            .or_insert(0);
        *entry += v;
    }

    fn print(&self) {
        println!("=============");
        self.pairs
            .iter()
            .for_each(|(pair, count)| println!("{} -> {}", pair, count));
        println!("=============");
        self.counter
            .iter()
            .for_each(|(pair, count)| println!("{} -> {}", pair, count));
        println!("=============");
    }

    fn tick(&mut self) {
        let mut pairs = self.pairs.clone();

        self.pairs.clone().iter().for_each(|(pair, count)| {
            if self.vocab.contains_key(pair) {
                if *count > 0 {
                    let count = count.to_owned();

                    self.rem(&mut pairs, pair.to_string(), count);

                    let ins_ch = self.vocab.get(pair).unwrap().to_owned();
                    self.inc(ins_ch.to_string(), count);

                    let chars = pair.chars().collect::<Vec<_>>();
                    self.ins(
                        &mut pairs,
                        format!("{}{}", chars[0], ins_ch).to_string(),
                        count,
                    );
                    self.ins(
                        &mut pairs,
                        format!("{}{}", ins_ch, chars[1]).to_string(),
                        count,
                    );
                }
            }
        });

        self.pairs = pairs;
    }

    fn count(&self) -> usize {
        println!("{:?}", self.counter);

        let values: Vec<usize> = self.counter.values().map(|v| v.to_owned()).collect();
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
    println!("{:?}", polymer);
    for _ in 0..steps {
        polymer.print();
        polymer.tick();
    }
    polymer.count()
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    assert_eq!(run(input, 10), 2223);
    assert_eq!(run(input, 40), 1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input_for(input_for_test: &'static str, polymer: &Polymer) {
        let mut counter: HashMap<char, usize> = HashMap::new();

        input_for_test.chars().for_each(|c| {
            let entry = counter.entry(c).or_insert(0);
            *entry += 1;
        });

        counter.iter().for_each(|(c, v)| {
            assert_eq!(
                polymer.counter.get(c),
                Some(v),
                "{}",
                format!("Checknig if {} was counted {}", c, v)
            );
        });

        let mut pairs: HashMap<String, usize> = HashMap::new();
        input_for_test
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|cs| cs.iter().collect::<String>())
            .for_each(|s| {
                println!("{:?}", s);
                let entry = pairs.entry(s).or_insert(0);
                *entry += 1;
            });

        pairs.iter().for_each(|(s, v)| {
            assert_eq!(
                polymer.pairs.get(s),
                Some(v),
                "{}",
                format!(
                    "Checking for present of {} with value {} for {}",
                    s, v, input_for_test
                )
            );
        });
    }

    #[test]
    fn test_single_tick() {
        let input = include_str!("test-input.txt");
        let mut polymer = Polymer::new();
        input.lines().for_each(|line| polymer.process(line));

        test_input_for("NNCB", &polymer);

        polymer.tick();
        // polymer.print();
        test_input_for("NCNBCHB", &polymer);

        polymer.tick();
        // polymer.print();
        test_input_for("NBCCNBBBCBHCB", &polymer);

        polymer.tick();
        // polymer.print();
        test_input_for("NBBBCNCCNBBNBNBBCHBHHBCHB", &polymer);

        polymer.tick();
        polymer.print();
        test_input_for(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            &polymer,
        );
    }

    #[test]
    fn test_full_run_test_input() {
        let test_input = include_str!("test-input.txt");
        assert_eq!(run(test_input, 10), 1588);
        assert_eq!(run(test_input, 40), 2188189693529);
    }

    #[test]
    fn test_full_run_sample_input() {
        let test_input = include_str!("input.txt");
        assert_eq!(run(test_input, 10), 2223);
        assert_eq!(run(test_input, 40), 2566282754493);
    }
}
