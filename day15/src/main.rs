use std::io::Result;

const GREEN: &str = "\x1b[0;32m";
const NC: &str = "\x1b[0m";

type Point = (usize, usize);

#[derive(Clone, Debug)]
struct Node {
    pos: Point,
    parent: Option<Point>,
    cost: usize,
    total_cost: usize,
    unexplored: bool,
}

#[derive(Debug)]
struct Path {
    field: Vec<Vec<Node>>,
}

impl Path {
    fn new() -> Path {
        Path { field: vec![] }
    }
}

impl Path {
    fn width(&self) -> usize {
        self.field[0].len()
    }

    fn height(&self) -> usize {
        self.field.len()
    }

    fn process(&mut self, line: &'static str) {
        let nums = line
            .chars()
            .map(|c| {
                let cost = c.to_string().parse().unwrap();
                Node {
                    cost,
                    parent: None,
                    pos: (0, 0),
                    total_cost: std::usize::MAX,
                    unexplored: true,
                }
            })
            .collect::<Vec<Node>>();
        self.field.push(nums);
    }

    fn expand_map(&mut self) {
        let width = self.width();
        let height = self.height();
        let p = self.field[0][0].clone();
        let old_field = self.field.clone();
        self.field = vec![vec![p; height * 5]; width * 5];

        for x in 0..width {
            for y in 0..height {
                for i in 0..5 {
                    let old_p = old_field[x][y].clone();
                    let mut new_p = old_p.clone();
                    let nx = x;
                    let ny = y + i * height;
                    new_p.cost += i;
                    if new_p.cost > 9 {
                        new_p.cost %= 9;
                    }
                    new_p.pos = (nx, ny);
                    self.field[x][y] = old_p;
                    self.field[nx][ny] = new_p;
                }
            }
        }

        for x in 0..width {
            for y in 0..self.height() {
                for i in 0..5 {
                    let old_p = self.field[x][y].clone();
                    let mut new_p = old_p.clone();
                    let nx = x + i * width;
                    let ny = y;
                    new_p.cost += i;
                    if new_p.cost > 9 {
                        new_p.cost %= 9;
                    }
                    new_p.pos = (nx, ny);
                    self.field[x][y] = old_p;
                    self.field[nx][ny] = new_p;
                }
            }
        }
    }

    fn reset_cache(&mut self, start: Point) {
        for x in 0..(self.width()) {
            for y in 0..(self.height()) {
                self.field[x][y].pos = (x, y);
                self.field[x][y].unexplored = true;
            }
        }
        self.field[start.0][start.1].total_cost = self.field[start.0][start.1].cost;
    }

    fn neighbours(&self, point: Point) -> Vec<Node> {
        let (ox, oy) = point;
        let points: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut result = vec![];

        points.iter().for_each(|(x, y)| {
            let nx: i32 = (ox as i32) + x;
            let ny: i32 = (oy as i32) + y;
            if nx >= 0
                && ny >= 0
                && nx < self.width() as i32
                && ny < self.height() as i32
                && self.field[nx as usize][ny as usize].unexplored
            {
                result.push(self.field[nx as usize][ny as usize].clone())
            }
        });

        result
    }

    fn print(&self, tp: &'static str) {
        println!();
        for x in 0..(self.width()) {
            println!();
            for y in 0..(self.height()) {
                let p = self.field[x][y].clone();
                let mut color = GREEN;
                if p.unexplored {
                    color = NC;
                }
                let mut v = p.cost;

                // if tp == "cache" {
                //     v = p.total_cost;
                //     if v == std::usize::MAX {
                //         v = 0
                //     }
                // }
                print!("{}{}{}", color, v, NC)
            }
        }
        println!();
    }

    fn still_unexplored(&self) -> bool {
        self.field
            .iter()
            .any(|row| row.iter().any(|p| p.unexplored))
    }

    fn next_unexplored_node(&self) -> Node {
        let mut nodes: Vec<Node> = vec![];

        for x in 0..(self.width()) {
            for y in 0..(self.height()) {
                let p = self.field[x][y].clone();
                if p.unexplored {
                    nodes.push(p);
                }
            }
        }

        nodes.sort_by_key(|p| p.total_cost);
        // println!("{:?}", nodes);
        nodes.first().unwrap().to_owned()
    }

    fn cheapest_path(&mut self, start: Point, end: Point) -> usize {
        self.reset_cache(start);

        // let (end_x, end_y) = end;
        let mut current: Node = self.field[start.0][start.1].clone();

        while self.still_unexplored() {
            // println!();
            // println!("Current {:?}", current);
            if current.pos == end {
                break;
            }

            self.field[current.pos.0][current.pos.1].unexplored = false;

            let nbs = self.neighbours(current.pos);
            nbs.iter().for_each(|nb| {
                let new_cost = nb.cost + current.total_cost;

                if new_cost < nb.total_cost {
                    self.field[nb.pos.0][nb.pos.1].parent = Some(current.pos);
                    self.field[nb.pos.0][nb.pos.1].total_cost = new_cost;
                }
            });

            current = self.next_unexplored_node();
            // println!("Current {:?}", current);
            // self.print("field");
            // self.print("cache");
        }

        // println!();
        // println!("{:?}", current);

        current.total_cost - self.field[start.0][start.1].cost
    }
}

fn run(input: &'static str, v: usize) {
    let mut p = Path::new();
    input.lines().for_each(|line| p.process(line));
    p.expand_map();
    p.reset_cache((0, 0));
    // p.print("field");
    let cheapest = p.cheapest_path((0, 0), (p.width() - 1, p.height() - 1));
    // p.print("cache");
    println!("Cheapest path {}", cheapest);
    assert_eq!(cheapest, v);
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    // Part one
    // run(test_input, 40);
    // run(input, 619);

    run(test_input, 315);
    run(input, 619);

    Ok(())
}
