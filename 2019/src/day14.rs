use regex::Regex;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::VecDeque;
use std::str::FromStr;
mod common;

#[derive(Debug, PartialEq, Eq, Clone)]
struct ChemicalAmount {
    amount: u64,
    chemical: String,
}

impl FromStr for ChemicalAmount {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("(?P<amount>[0-9]*) (?P<chemical>[A-Z]*)").unwrap();
        let caps = re.captures(s).ok_or("could not chemical amount")?;
        let amount = caps["amount"]
            .parse::<u64>()
            .map_err(|_| "could not parse amount")?;
        let chemical = caps["chemical"].to_string();
        Ok(ChemicalAmount { amount, chemical })
    }
}

#[derive(Debug)]
struct Reaction {
    lhs: Vec<ChemicalAmount>,
    rhs: ChemicalAmount,
}

impl FromStr for Reaction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("(?P<lhs>[0-9]* [A-Z]*(, [0-9]* [A-Z]*)*) => (?P<rhs>[0-9]* [A-Z]*)")
            .unwrap();
        let caps = re.captures(s).ok_or("could not parse reaction")?;
        let lhs = caps["lhs"]
            .split(", ")
            .map(|s| s.parse::<ChemicalAmount>())
            .collect::<Result<Vec<_>, _>>()?;
        let rhs = caps["rhs"].parse::<ChemicalAmount>()?;
        Ok(Reaction { lhs, rhs })
    }
}

struct ReactionGraph<'a> {
    nodes: HashMap<String, &'a Reaction>,
}

impl<'a> ReactionGraph<'a> {
    fn topological_sort(&self) -> Result<Vec<String>, &'static str> {
        let mut incoming_count: HashMap<String, u64> = HashMap::new();
        for r in self.nodes.values() {
            incoming_count.entry(r.rhs.chemical.clone()).or_insert(0);
            for lhs in r.lhs.iter() {
                *incoming_count.entry(lhs.chemical.clone()).or_insert(0) += 1;
            }
        }
        let mut queue: VecDeque<String> = VecDeque::new();
        for (s, &i) in incoming_count.iter() {
            if i == 0 {
                queue.push_back(s.clone());
            }
        }
        let mut order = Vec::new();
        while let Some(s) = queue.pop_front() {
            for lhs in self.nodes.get(&s).iter().flat_map(|r| r.lhs.iter()) {
                let c = incoming_count.get_mut(&lhs.chemical).unwrap();
                *c -= 1;
                if *c == 0 {
                    queue.push_back(lhs.chemical.clone())
                }
            }
            order.push(s);
        }
        if incoming_count.values().any(|&c| c != 0) {
            Err("graph is not acylic")
        } else {
            Ok(order)
        }
    }
}

fn build_reaction_graph(reactions: &[Reaction]) -> Result<ReactionGraph, &'static str> {
    let mut nodes: HashMap<String, &Reaction> = HashMap::new();
    for r in reactions {
        match nodes.entry(r.rhs.chemical.clone()) {
            Entry::Occupied(_) => return Err("duplicate chemical on right hand side"),
            Entry::Vacant(e) => {
                e.insert(r);
            }
        }
    }
    Ok(ReactionGraph { nodes })
}

fn find_minimimum_amount(
    graph: &ReactionGraph,
    order: &[String],
    target: ChemicalAmount,
) -> Vec<ChemicalAmount> {
    let mut amount: HashMap<String, u64> = HashMap::new();
    amount.insert(target.chemical, target.amount);
    let mut needed: Vec<ChemicalAmount> = Vec::new();
    for cur_target in order.iter() {
        let cur_amount = amount.get(cur_target).copied().unwrap_or(0);
        if cur_amount > 0 {
            if let Some(r) = graph.nodes.get(cur_target) {
                assert!(r.rhs.amount > 0);
                // can produce cur_amount of cur_target by
                // by using reaction r num_application times
                // use integer version of ceil(r.rhs.amount / cur_amount)
                let num_applications = 1 + (cur_amount - 1) / r.rhs.amount;
                for lhs in r.lhs.iter() {
                    let lhs_needed = lhs.amount * num_applications;
                    *amount.entry(lhs.chemical.clone()).or_insert(0) += lhs_needed;
                }
            } else {
                // cannot produce cur_target
                needed.push(ChemicalAmount {
                    amount: cur_amount,
                    chemical: cur_target.clone(),
                });
            }
        }
    }
    needed
}

fn minimum_amount_of_ore_for_fuel(
    graph: &ReactionGraph,
    order: &[String],
    target_fuel: u64,
) -> Result<u64, &'static str> {
    let target = ChemicalAmount {
        amount: target_fuel,
        chemical: "FUEL".to_string(),
    };
    let needed = find_minimimum_amount(&graph, &order, target);
    if needed.len() != 1 || needed[0].chemical != "ORE" {
        return Err("can not produce FUEL with only ORE");
    }
    Ok(needed[0].amount)
}

fn part1(reactions: &[Reaction]) -> Result<u64, &'static str> {
    let graph = build_reaction_graph(&reactions)?;
    let order = graph.topological_sort()?;
    let result = minimum_amount_of_ore_for_fuel(&graph, &order, 1)?;
    Ok(result)
}

fn part2(reactions: &[Reaction]) -> Result<u64, &'static str> {
    let graph = build_reaction_graph(&reactions)?;
    let order = graph.topological_sort()?;
    const CARGO_ORE: u64 = 1_000_000_000_000;
    let mut lower: u64 = 1;
    let mut upper: u64 = CARGO_ORE;
    while upper - lower >= 2 {
        let fuel = (lower + upper) / 2;
        let ore = minimum_amount_of_ore_for_fuel(&graph, &order, fuel)?;
        if ore <= CARGO_ORE {
            lower = fuel;
        } else {
            upper = fuel;
        }
    }
    Ok(lower)
}

fn main() -> Result<(), &'static str> {
    let reactions: Vec<_> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Reaction>())
        .collect::<Result<Vec<_>, _>>()?;

    let result1 = part1(&reactions)?;
    println!("Part1: minimum amount of ORE for one FUEL is {}", result1);

    let result2 = part2(&reactions)?;
    println!(
        "Part2: maximum amount of fuel that can be produced is {}",
        result2
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ore_to_fuel(input: &[&str], expected_ore: u64) {
        let reactions = input
            .iter()
            .map(|s| s.parse::<Reaction>().unwrap())
            .collect::<Vec<_>>();
        let needed = part1(&reactions).unwrap();
        assert_eq!(needed, expected_ore);
    }
    fn test_fuel_for_ore(input: &[&str], expected_fuel: u64) {
        let reactions = input
            .iter()
            .map(|s| s.parse::<Reaction>().unwrap())
            .collect::<Vec<_>>();
        let possible = part2(&reactions).unwrap();
        assert_eq!(possible, expected_fuel);
    }
    #[test]
    fn test_example1() {
        let input = vec![
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ];
        test_ore_to_fuel(&input, 31);
    }
    #[test]
    fn test_example2() {
        let input = vec![
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ];
        test_ore_to_fuel(&input, 165);
    }

    #[test]
    fn test_example3() {
        let input = vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ];
        test_ore_to_fuel(&input, 13312);
        test_fuel_for_ore(&input, 82_892_753);
    }

    #[test]
    fn test_example4() {
        let input = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ];
        test_ore_to_fuel(&input, 180_697);
        test_fuel_for_ore(&input, 5_586_022);
    }

    #[test]
    fn test_example5() {
        let input = vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ];
        test_ore_to_fuel(&input, 2_210_736);
        test_fuel_for_ore(&input, 460_664);
    }
}
