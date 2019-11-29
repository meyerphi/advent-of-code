use std::str::FromStr;
#[path = "common.rs"]
mod common;

#[derive(Debug)]
enum Event {
    Begin { id: u32 },
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
struct Entry {
    date: Date,
    event: Event,
}

impl FromStr for Event {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wakes up" => Ok(Event::WakeUp),
            "falls asleep" => Ok(Event::FallAsleep),
            _ if s.starts_with("Guard #") => {
                let id = s[7..(s.find(" begins").ok_or("could not find begins"))?]
                    .parse::<u32>()
                    .map_err(|_| "Could not parse guard id")?;
                Ok(Event::Begin { id })
            }
            _ => Err("unknown event".to_string()),
        }
    }
}

impl FromStr for Date {
    type Err = String;
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
            Err("date has not enough elements".to_string())
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
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date_s, rest) = s.split_at(s.find("] ").ok_or_else(|| "] not found".to_string())? + 1);
        let event_s = &rest[1..];
        let date = Date::from_str(date_s)?;
        let event = Event::from_str(event_s)?;
        Ok(Entry { date, event })
    }
}

#[allow(dead_code)]
fn main() {
    let entries: Vec<Entry> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Entry>().expect("could not parse entry"))
        .collect();
    for e in entries {
        println!("{:?}", e);
    }
    println!("ID of chosen guard: {}", 0);
}
