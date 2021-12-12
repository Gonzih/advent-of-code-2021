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

    fn walk(&self, from: Node, to: Node) -> Vec<Vec<Node>> {
        let init_paths: Vec<Path> = vec![vec![from]];
        let mut paths_cache: Vec<Path> = init_paths.clone();
        let mut paths: Vec<Path> = vec![];
        let mut paths_log: Vec<Path> = init_paths.clone();

        loop {
            println!("Paths cache {:?}", paths_cache);
            println!("Paths log {:?}", paths_log);
            println!("Paths state {:?}\n", paths);

            if paths_log.len() == 0 {
                break;
            }

            let current_path = paths_log.remove(0);
            let current_end: Node = current_path.last().unwrap().clone();

            let edges_from_this: Vec<&Edge> = self
                .edges
                .iter()
                .filter(|e| e.from == current_end)
                .collect();

            edges_from_this.iter().for_each(|e| {
                let mut new_path = current_path.to_owned();
                new_path.push(e.to.clone());

                if !paths_cache.contains(&new_path) {
                    paths_log.push(new_path.clone());
                    paths_cache.push(new_path);
                }
            });

            paths_log
                .iter()
                .filter(|path| path_ends_with(path, to.clone()))
                .for_each(|path| {
                    let mut p: Path = path.clone();
                    p.push(to.clone());
                    paths.push(p);
                });

            paths_log = paths_log
                .into_iter()
                .filter(|path| !path_ends_with(path, to.clone()))
                .collect::<Vec<_>>();
        }

        paths
    }

    fn possible_paths(&self, from: Node, to: Node) -> Vec<Vec<Node>> {
        let paths = self.walk(from.clone(), to.clone());

        println!("Paths {:?}", paths);

        paths
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

    let paths = graph.possible_paths("start".to_string(), "end".to_string());
    println!("Possible paths {:?}", paths);

    Ok(())
}

fn main() -> Result<()> {
    let test_input = include_str!("test-input.txt");
    let input = include_str!("input.txt");

    run(test_input)?;
    // run(input)?;

    Ok(())
}
