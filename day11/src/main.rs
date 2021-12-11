use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

const GREEN: &str = "\x1b[0;32m";
const NC: &str = "\x1b[0m";

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
    fn from_offset(i: usize, width: usize) -> Vec2 {
        let y = i / width;
        let x = i % width;
        Vec2 {
            x: x as i32,
            y: y as i32,
        }
    }
}
impl Vec2 {
    fn add(&self, other: &Vec2) -> Vec2 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2 { x, y }
    }

    fn neighbours(&self, width: usize, height: usize) -> Vec<Vec2> {
        let w = width as i32;
        let h = height as i32;
        let offsets = [-1, 0, 1];
        let mut neighbours: Vec<Vec2> = vec![];

        for i in offsets {
            for j in offsets {
                if !(i == 0 && j == 0) {
                    let ii = i as i32;
                    let jj = j as i32;
                    neighbours.push(Vec2::new(ii, jj));
                }
            }
        }

        neighbours
            .iter()
            .map(|v| self.add(v))
            .filter(|v| v.x >= 0 && v.y >= 0 && v.x < w && v.y < h)
            .collect()
    }

    fn to_offset(&self, width: usize) -> usize {
        let w = width as i32;
        (self.y * w + self.x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2() {
        let v1 = Vec2::from_offset(25, 10);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.to_offset(10), 25);

        let nb = Vec2::new(5, 5).neighbours(10, 10);
        println!("{:?}", nb);
        assert_eq!(nb.len(), 8);
    }
}

#[derive(Debug)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    fn new(energy: u32) -> Octopus {
        Octopus {
            energy,
            flashed: false,
        }
    }

    fn from_char(c: char) -> Result<Octopus> {
        let energy = c.to_digit(10).ok_or(Error::new(
            ErrorKind::Other,
            format!("could not parse {} to int", c),
        ))?;

        Ok(Octopus::new(energy))
    }
}

impl Octopus {
    fn flash(&mut self) -> bool {
        let should_flash = !self.flashed && self.energy > 9;
        self.flashed = should_flash;
        should_flash
    }

    fn tick(&mut self) {
        self.energy += 1;
    }

    fn reset(&mut self) {
        if self.flashed {
            self.energy = 0;
            self.flashed = false;
        }
    }
}

#[derive(Debug)]
struct Game {
    octopusses: Vec<Octopus>,
    width: usize,
    height: usize,
    elements: usize,
    n_ticks: usize,
    flashes: usize,
    last_tick_flashes: usize,
}

impl Game {
    fn new(field: Vec<Vec<Octopus>>) -> Game {
        let height = field.len();
        let width = field[0].len();
        let octopusses: Vec<Octopus> = field.into_iter().flatten().collect();
        let elements = octopusses.len();
        Game {
            octopusses,
            height,
            width,
            elements,
            n_ticks: 0,
            flashes: 0,
            last_tick_flashes: 0,
        }
    }
}

impl Game {
    fn flash_neighbours(&mut self, i: usize) {
        let neighbours: Vec<usize> = Vec2::from_offset(i, self.width)
            .neighbours(self.width, self.height)
            .iter()
            .map(|v| v.to_offset(self.width))
            .collect();

        for j in neighbours {
            self.octopusses[j].tick();
            let flashed = self.octopusses[j].flash();
            if flashed {
                self.last_tick_flashes += 1;
                self.flash_neighbours(j);
            }
        }
    }

    fn tick(&mut self) {
        self.last_tick_flashes = 0;

        for i in 0..self.elements {
            self.octopusses[i].tick();
        }

        for i in 0..self.elements {
            let flashed = self.octopusses[i].flash();
            if flashed {
                self.last_tick_flashes += 1;
                self.flash_neighbours(i);
            }
        }

        self.print();
        println!("");

        for i in 0..self.elements {
            self.octopusses[i].reset();
        }

        self.flashes += self.last_tick_flashes;
        self.n_ticks += 1;
    }

    fn print(&self) {
        let s = self
            .octopusses
            .iter()
            .map(|o| {
                if o.flashed {
                    format!("{}{}{}", GREEN, 0, NC)
                } else {
                    format!("{}", o.energy)
                }
            })
            .collect::<Vec<String>>()
            .chunks(self.width)
            .map(|c| c.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", s);
    }

    fn simulate(&mut self) {
        for i in 0..100 {
            println!("#{}", i + 1);
            self.tick();
        }
        println!("Total flashes after 100: {}", self.flashes);

        loop {
            self.tick();
            if self.last_tick_flashes == self.octopusses.len() {
                println!("Synchronized on tick {}", self.n_ticks);
                break;
            }
        }
    }
}

fn run(input: &str) -> Result<usize> {
    let octopusses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Octopus::from_char(c))
                .collect::<Result<Vec<Octopus>>>()
        })
        .collect::<Result<Vec<Vec<Octopus>>>>()?;

    let mut game = Game::new(octopusses);
    game.simulate();

    Ok(game.flashes)
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    run(test_input)?;
    run(input)?;

    Ok(())
}
