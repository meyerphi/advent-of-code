use std::collections::HashMap;
use std::str::FromStr;
#[path = "common.rs"]
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

fn choose_guard(entries: Vec<Entry>) -> u32 {
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
    let (chosen_guard, _) = sleeping_minutes.iter().max_by_key(|(_, m)| m.len()).unwrap();

    let mut frequency = HashMap::new();
    for minute in sleeping_minutes.get(&chosen_guard).unwrap() {
        *frequency.entry(minute).or_insert(0) += 1;
    }
    let (&chosen_minute, _) = frequency.iter().max_by_key(|(_, &days)| days).unwrap();

    chosen_guard * chosen_minute
}

#[allow(dead_code)]
fn main() {
    let mut entries: Vec<Entry> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Entry>().expect("could not parse entry"))
        .collect();
    entries.sort();
    let result = choose_guard(entries);
    println!("ID of chosen guard * chosen minute: {}", result);
}
