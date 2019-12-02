use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
mod common;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    Begin { id: u32 },
    FallAsleep,
    WakeUp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    date: Date,
    event: Event,
}

impl FromStr for Event {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wakes up" => Ok(Event::WakeUp),
            "falls asleep" => Ok(Event::FallAsleep),
            _ if s.starts_with("Guard #") && s.ends_with(" begins shift") => {
                let id = s[7..s.len() - 13]
                    .parse::<u32>()
                    .map_err(|_| "could not parse guard id")?;
                Ok(Event::Begin { id })
            }
            _ => Err("unknown event"),
        }
    }
}

impl FromStr for Date {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e: Vec<_> = s
            .split(['[', ']', '-', ' ', ':'].as_ref())
            .filter(|t| !t.is_empty())
            .map(|t| {
                t.trim()
                    .parse::<u32>()
                    .map_err(|_| "Could not parse number")
            })
            .flatten()
            .collect();
        if e.len() < 5 {
            Err("date has not enough elements")
        } else {
            Ok(Date {
                year: e[0],
                month: e[1],
                day: e[2],
                hour: e[3],
                minute: e[4],
            })
        }
    }
}

impl FromStr for Entry {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 20 {
            Err("entry too short")
        } else {
            let date = Date::from_str(&s[1..17])?;
            let event = Event::from_str(&s[19..])?;
            Ok(Entry { date, event })
        }
    }
}

type SleepMap = std::collections::HashMap<u32, std::vec::Vec<u32>>;

fn build_map(entries: Vec<Entry>) -> SleepMap {
    let mut sleeping_minutes = HashMap::new();
    let mut guard = 0;
    let mut sleep_begin = 0;
    for e in &entries {
        match e.event {
            Event::Begin { id } => guard = id,
            Event::FallAsleep => sleep_begin = e.date.minute,
            Event::WakeUp => {
                for minute in sleep_begin..e.date.minute {
                    sleeping_minutes
                        .entry(guard)
                        .or_insert_with(|| vec![])
                        .push(minute);
                }
            }
        }
    }
    sleeping_minutes
}

fn frequency_map<K: Eq + Hash, I: IntoIterator<Item = K>>(iter: I) -> HashMap<K, u32> {
    let mut frequency = HashMap::new();
    for x in iter {
        *frequency.entry(x).or_insert(0) += 1;
    }
    frequency
}

fn choose_guard_strategy1(map: &SleepMap) -> u32 {
    let (chosen_guard, sleep_map) = map.iter().max_by_key(|(_, m)| m.len()).unwrap();

    let frequency = frequency_map(sleep_map);
    let (&chosen_minute, _) = frequency.iter().max_by_key(|(_, &days)| days).unwrap();

    chosen_guard * chosen_minute
}

fn choose_guard_strategy2(map: &SleepMap) -> u32 {
    let mut chosen_guard = 0;
    let mut chosen_minute = 0;
    let mut max_times = 0;

    for (&guard, sleep_map) in map {
        let frequency = frequency_map(sleep_map);
        let (&&max_minute, &days) = frequency.iter().max_by_key(|(_, &days)| days).unwrap();
        if days >= max_times {
            chosen_guard = guard;
            chosen_minute = max_minute;
            max_times = days;
        }
    }

    chosen_guard * chosen_minute
}

#[allow(dead_code)]
fn main() {
    let mut entries: Vec<Entry> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Entry>().expect("could not parse entry"))
        .collect();
    entries.sort();
    let map = build_map(entries);

    let result1 = choose_guard_strategy1(&map);
    println!(
        "Strategy 1: ID of chosen guard * chosen minute: {}",
        result1
    );

    let result2 = choose_guard_strategy2(&map);
    println!(
        "Strategy 2: ID of chosen guard * chosen minute: {}",
        result2
    );
}
