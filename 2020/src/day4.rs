mod common;
use std::str::FromStr;

fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_strictly_valid()).count()
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn set_field(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "byr" => self.byr = Some(value.to_string()),
            "iyr" => self.iyr = Some(value.to_string()),
            "eyr" => self.eyr = Some(value.to_string()),
            "hgt" => self.hgt = Some(value.to_string()),
            "hcl" => self.hcl = Some(value.to_string()),
            "ecl" => self.ecl = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            "cid" => self.cid = Some(value.to_string()),
            _ => return Err("invalid key: ".to_string() + key),
        }
        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn validate_field(field: &Option<String>, pattern: &str) -> bool {
        match field {
            Some(value) => {
                let re = regex::Regex::new(pattern).unwrap();
                re.is_match(value)
            }
            None => false,
        }
    }

    fn is_strictly_valid(&self) -> bool {
        Self::validate_field(&self.byr, "^((19[2-9][0-9])|(200[0-2]))$")
            && Self::validate_field(&self.iyr, "^20(1[0-9]|20)$")
            && Self::validate_field(&self.eyr, "^20(2[0-9]|30)$")
            && Self::validate_field(
                &self.hgt,
                "^(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))$",
            )
            && Self::validate_field(&self.hcl, "^#[0-9a-f]{6}$")
            && Self::validate_field(&self.ecl, "^(amb|blu|brn|gry|grn|hzl|oth)$")
            && Self::validate_field(&self.pid, "^[0-9]{9}$")
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::new();
        for entry in s.split_whitespace() {
            let mut split = entry.splitn(2, ':');
            let key = split.next().ok_or("no key")?;
            let value = split.next().ok_or("no value")?;
            passport.set_field(key, value)?;
        }
        Ok(passport)
    }
}

fn parse_input(input: &str) -> Result<Vec<Passport>, String> {
    let mut passports = Vec::new();
    let mut cur_string = String::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(cur_string.parse::<Passport>()?);
            cur_string.clear()
        } else {
            cur_string.push_str(line);
            cur_string.push('\n');
        }
    }
    if !cur_string.is_empty() {
        passports.push(cur_string.parse::<Passport>()?);
    }
    Ok(passports)
}

fn main() {
    let input = parse_input(&common::get_content()).expect("could not parse input");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<Passport> {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\
        \n\
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\
        \n\
        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\
        \n\
        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in";
        parse_input(&input).unwrap()
    }

    fn test_invalid() -> Vec<Passport> {
        let input = "eyr:1972 cid:100\n\
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
        \n\
        iyr:2019\n\
        hcl:#602927 eyr:1967 hgt:170cm\n\
        ecl:grn pid:012533040 byr:1946\n\
        \n\
        hcl:dab227 iyr:2012\n\
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
        \n\
        hgt:59cm ecl:zzz\n\
        eyr:2038 hcl:74454a iyr:2023\n\
        pid:3556412378 byr:2007";
        parse_input(&input).unwrap()
    }

    fn test_valid() -> Vec<Passport> {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
        hcl:#623a2f\n\
        \n\
        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
        \n\
        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\
        \n\
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        parse_input(&input).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_input()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_invalid()), 0);
        assert_eq!(part2(&test_valid()), 4);
    }
}
