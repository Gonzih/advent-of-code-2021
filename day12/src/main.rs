use std::fmt;
use std::io::Result;

type Node = String;

type Path = Vec<Node>;

fn path_ends_with(path: &Path, node: Node) -> bool {
    path.last().map(|n| n == &node).unwrap_or(false)
}

#[derive(PartialEq)]
struct Edge {
    from: Node,
    to: Node,
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{}", self.from, self.to)
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }
}

impl Graph {
    fn add_node(&mut self, node: Node) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        self.add_node(from.clone());
        self.add_node(to.clone());
        self.edges.push(Edge {
            from: from.clone(),
            to: to.clone(),
        });
        if from != "start" && to != "end" {
            self.edges.push(Edge {
                from: to.clone(),
                to: from.clone(),
            });
        }
    }

    fn walk(&self, from: Node, to: Node, visited: Path, can_visit_twice: bool) -> usize {
        let mut counter: usize = 0;

        self.edges.iter().filter(|e| e.from == from).for_each(|e| {
            if e.to == to {
                counter += 1;
            } else if e.to.to_uppercase() == e.to {
                counter += self.walk(e.to.clone(), to.clone(), visited.clone(), can_visit_twice);
            } else if !visited.contains(&e.to) {
                let mut new_visited = visited.clone();
                new_visited.push(e.to.clone());
                counter += self.walk(e.to.clone(), to.clone(), new_visited, can_visit_twice);
            } else if e.to != "start".to_string() && can_visit_twice {
                counter += self.walk(e.to.clone(), to.clone(), visited.clone(), false);
            }
        });

        counter
    }

    fn possible_paths(&self, from: Node, to: Node, can_visit_twice: bool) -> usize {
        self.walk(from.clone(), to.clone(), vec![from], can_visit_twice)
    }
}

fn run(input: &'static str) -> Result<()> {
    let edges: Vec<Vec<Node>> = input
        .lines()
        .map(|line| line.split("-").map(|s| s.to_string()).collect())
        .collect();

    let mut graph = Graph::new();
    edges
        .iter()
        .for_each(|edge| graph.add_edge(edge[0].clone(), edge[1].clone()));

    println!("{:?}", graph);

    let p1 = graph.possible_paths("start".to_string(), "end".to_string(), false);
    println!("Part one {:?}", p1);
    let p2 = graph.possible_paths("start".to_string(), "end".to_string(), true);
    println!("Part two {:?}", p2);

    Ok(())
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    run(test_input)?;
    run(input)?;

    Ok(())
}
