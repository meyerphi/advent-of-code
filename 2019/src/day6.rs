mod common;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

struct Edge {
    source: String,
    target: String,
}

impl FromStr for Edge {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Vec<_> = s.split(')').collect();
        if m.len() != 2 {
            Err("could not parse edge")
        } else {
            Ok(Edge {
                source: m[0].to_string(),
                target: m[1].to_string(),
            })
        }
    }
}

struct OrbitMap {
    edges: HashMap<String, Vec<String>>,
}

impl OrbitMap {
    fn from_edges(edges: &[Edge]) -> OrbitMap {
        let mut m = OrbitMap {
            edges: HashMap::new(),
        };
        for e in edges {
            let u = &e.source;
            let v = &e.target;
            m.edges
                .entry(u.clone())
                .or_insert_with(|| vec![])
                .push(v.clone());
            m.edges
                .entry(v.clone())
                .or_insert_with(|| vec![])
                .push(u.clone());
        }
        m
    }

    fn distances<'a>(&'a self, source: &'a str) -> HashMap<&'a str, i32> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        distances.insert(source, 0);
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            let dist = *distances.get(u).unwrap();
            for v in self.edges.get(u).iter().flat_map(|e| e.iter()) {
                distances.entry(v).or_insert_with(|| {
                    queue.push_back(v);
                    dist + 1
                });
            }
        }
        distances
    }

    fn checksum(&self) -> i32 {
        let orbits = self.distances("COM");
        orbits.values().sum()
    }

    fn transfers(&self, source: &str, target: &str) -> Option<i32> {
        let distances = self.distances(source);
        let distance = distances.get(target)?;
        Some(distance - 2)
    }
}

fn main() {
    let edges: Vec<_> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Edge>().expect("could not parse orbit"))
        .collect();
    let map = OrbitMap::from_edges(&edges);

    let result1 = map.checksum();
    println!("Part1: orbit map checksum: {}", result1);

    let result2 = map.transfers("YOU", "SAN").unwrap();
    println!("Part1: orbit map checksum: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbits() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        let edges: Vec<_> = input.iter().map(|e| e.parse::<Edge>().unwrap()).collect();
        let map = OrbitMap::from_edges(&edges);
        let orbits = map.distances("COM");
        assert_eq!(orbits.get("D"), Some(&3));
        assert_eq!(orbits.get("L"), Some(&7));
        assert_eq!(orbits.get("COM"), Some(&0));
        let checksum = map.checksum();
        assert_eq!(checksum, 42)
    }

    #[test]
    fn test_distances() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        let edges: Vec<_> = input.iter().map(|e| e.parse::<Edge>().unwrap()).collect();
        let map = OrbitMap::from_edges(&edges);
        let distances = map.distances("YOU");
        assert_eq!(distances.get("YOU"), Some(&0));
        assert_eq!(distances.get("K"), Some(&1));
        assert_eq!(distances.get("SAN"), Some(&6));
        let transfers = map.transfers("YOU", "SAN");
        assert_eq!(transfers, Some(4))
    }
}
