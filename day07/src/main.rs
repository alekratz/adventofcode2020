use regex::Regex;
use petgraph::{Direction, Directed, graphmap::GraphMap};
use std::collections::HashSet;
use std::io::{stdin, Read};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn read_lines(source: &mut dyn Read) -> Result<Vec<String>> {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;
    Ok(buffer
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().to_string())
        .collect())
}

fn main() -> Result<()> {
    let lines = {
        let mut file = stdin();
        read_lines(&mut file)?
    };

    let graph = parse_rules(&lines);
    println!("Got {} nodes with {} edges", graph.node_count(), graph.edge_count());

    part1(&graph);
    println!();
    part2(&graph);

    Ok(())
}

fn part1(graph: &GraphMap<&str, usize, Directed>) {
    // find the "shiny gold" bag and walk up the tree
    fn find_ancestors<'a>(start: &'a str, graph: &GraphMap<&'a str, usize, Directed>) -> HashSet<&'a str> {
        graph.neighbors_directed(start, Direction::Incoming)
            .map(|n| (n, find_ancestors(n, graph)))
            .fold(HashSet::new(), |mut acc, (n, ancestors)| {
                acc.insert(n);
                acc.extend(ancestors);
                acc
            })
    }

    const START: &str = "shiny gold bag";

    println!("Part 1");
    println!("======");

    let ancestors = find_ancestors(START, graph);
    println!("There are {} ancestors to the '{}'", ancestors.len(), START);
}

fn part2(graph: &GraphMap<&str, usize, Directed>) {
    // find the number of bags that are contained by the shiny gold bag

    const START: &str = "shiny gold bag";
    fn count_descendants<'a>(start: &'a str, graph: &GraphMap<&'a str, usize, Directed>) -> usize {
        graph.neighbors(start)
            .map(|n| graph[(start, n)] * (count_descendants(n, graph) + 1))
            .sum()
    }

    println!("Part 2");
    println!("======");

    let descendants = count_descendants(START, graph);
    println!("There are {} required bags to be held in the '{}'", descendants, START);
}

fn parse_rules<'a>(rules: &'a Vec<String>) -> GraphMap<&'a str, usize, Directed> {
    let mut graph = GraphMap::new();
    let node_regex = Regex::new(r"^(.+? bag)").unwrap();

    // 1. Create all nodes
    for rule in rules {
        let caps = node_regex.captures(rule).unwrap();
        let bag_name = caps.get(1).unwrap().as_str();
        graph.add_node(bag_name);
    }

    let edge_regex = Regex::new(r"(\d+) (.+? bag)").unwrap();
    // 2. Create all edges
    for rule in rules {
        // get the node name again - probably inefficient but who cares
        let caps = node_regex.captures(rule).unwrap();
        let container = caps.get(1).unwrap().as_str();

        for caps in edge_regex.captures_iter(rule) {
            let count: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let contained = caps.get(2).unwrap().as_str();
            graph.add_edge(container, contained, count);
        }
    }

    graph
}
