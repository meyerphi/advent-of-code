mod common;

use regex::Regex;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Bag {
    colour: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BagAmount {
    bag: Bag,
    amount: usize,
}

#[derive(Debug)]
struct BagRule {
    outer: Bag,
    inner: Vec<BagAmount>,
}

impl FromStr for Bag {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("(?P<colour>[a-z ]*) bag(s)?").unwrap();
        let caps = re.captures(s).ok_or("could not parse bag")?;
        Ok(Bag {
            colour: caps["colour"].to_string(),
        })
    }
}

impl FromStr for BagAmount {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("(?P<amount>[0-9]*) (?P<bag>[a-z ]*)").unwrap();
        let caps = re.captures(s).ok_or("could not parse bag amount")?;
        let bag = caps["bag"].parse::<Bag>()?;
        let amount = caps["amount"]
            .parse::<usize>()
            .map_err(|_| "could not parse amount")?;
        Ok(BagAmount { bag, amount })
    }
}

impl FromStr for BagRule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("(?P<outer>[a-z ]*) contain (?P<inner>(no other bags|[a-z0-9 ,]*)).")
            .unwrap();
        let caps = re.captures(s).ok_or("could not parse reaction")?;
        let outer = caps["outer"].parse::<Bag>()?;
        let inner_str = &caps["inner"];
        let inner: Vec<_> = if inner_str == "no other bags" {
            Vec::new()
        } else {
            inner_str
                .split(", ")
                .map(|s| s.parse::<BagAmount>())
                .collect::<Result<Vec<_>, _>>()?
        };
        Ok(BagRule { outer, inner })
    }
}

type NodeId = usize;

#[derive(Debug)]
struct WeightedEdge {
    weight: usize,
    target: NodeId,
}

#[derive(Debug)]
struct Node {
    predecessors: Vec<WeightedEdge>,
    successors: Vec<WeightedEdge>,
}

#[derive(Debug)]
struct BagGraph {
    node_ids: HashMap<Bag, NodeId>,
    nodes: Vec<Node>,
}

impl BagGraph {
    fn node_for(&mut self, bag: Bag) -> NodeId {
        match self.node_ids.entry(bag) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let next_id = self.nodes.len();
                self.nodes.push(Node {
                    predecessors: Vec::new(),
                    successors: Vec::new(),
                });
                e.insert(next_id);
                next_id
            }
        }
    }

    fn get(&self, bag: Bag) -> Option<NodeId> {
        self.node_ids.get(&bag).copied()
    }

    fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }

    fn node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id]
    }

    fn add_rule(&mut self, rule: BagRule) {
        let id = self.node_for(rule.outer);
        for inner in rule.inner {
            let succ_id = self.node_for(inner.bag);
            self.node_mut(succ_id).predecessors.push(WeightedEdge {
                weight: inner.amount,
                target: id,
            });
            self.node_mut(id).successors.push(WeightedEdge {
                weight: inner.amount,
                target: succ_id,
            });
        }
    }

    fn new() -> BagGraph {
        BagGraph {
            node_ids: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn build<I: Iterator<Item = BagRule>>(rules: I) -> BagGraph {
        let mut graph = BagGraph::new();
        for rule in rules {
            graph.add_rule(rule);
        }
        graph
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }
}

fn part1(graph: &BagGraph) -> usize {
    let target = Bag {
        colour: "shiny gold".to_string(),
    };
    let initial_node = graph.get(target).unwrap();

    let mut queue = VecDeque::new();
    queue.push_back(initial_node);
    let mut visited = vec![false; graph.size()];
    visited[initial_node] = true;

    let mut count = 0;
    while let Some(id) = queue.pop_front() {
        for edge in &graph.node(id).predecessors {
            let pred_id = edge.target;
            if !visited[pred_id] {
                visited[pred_id] = true;
                count += 1;
                queue.push_back(pred_id);
            }
        }
    }
    count
}

fn part2_dfs(graph: &BagGraph, id: NodeId, mut cache: &mut [isize]) -> usize {
    if cache[id] >= 0 {
        cache[id] as usize
    } else {
        let mut count = 0;
        for edge in &graph.node(id).successors {
            let inner = part2_dfs(graph, edge.target, &mut cache);
            count += edge.weight * (1 + inner);
        }
        cache[id] = count as isize;
        count
    }
}

fn part2(graph: &BagGraph) -> usize {
    let target = Bag {
        colour: "shiny gold".to_string(),
    };
    let initial_node = graph.get(target).unwrap();

    let mut cache = vec![-1; graph.size()];
    part2_dfs(graph, initial_node, &mut cache)
}

fn main() {
    let rules: Vec<BagRule> = common::get_input();
    let graph = BagGraph::build(rules.into_iter());
    println!("Part1: {}", part1(&graph));
    println!("Part2: {}", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<BagRule> {
        let rule_strings = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        rule_strings
            .iter()
            .map(|s| s.parse::<BagRule>().unwrap())
            .collect()
    }

    fn test_input2() -> Vec<BagRule> {
        let rule_strings = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];
        rule_strings
            .iter()
            .map(|s| s.parse::<BagRule>().unwrap())
            .collect()
    }

    #[test]
    fn test_part1() {
        let rules: Vec<BagRule> = test_input();
        let graph = BagGraph::build(rules.into_iter());
        assert_eq!(part1(&graph), 4);
    }

    #[test]
    fn test_part2() {
        let rules1: Vec<BagRule> = test_input();
        let graph1 = BagGraph::build(rules1.into_iter());
        assert_eq!(part2(&graph1), 32);

        let rules2: Vec<BagRule> = test_input2();
        let graph2 = BagGraph::build(rules2.into_iter());
        assert_eq!(part2(&graph2), 126);
    }
}
