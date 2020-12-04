use regex::Regex;

use crate::lib::advent::Result;
use crate::Day;

#[derive(Debug, Default)]
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
    fn valid_part1(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }

    fn valid_number_range(field: &Option<String>, min: usize, max: usize) -> bool {
        if let Some(s) = field {
            let num = s.parse().unwrap();
            (min..=max).contains(&num)
        }
        else {
            false
        }
    }

    fn valid_pattern(field: &Option<String>, r: &str) -> bool {
        if let Some(f) = field {
            let r = Regex::new(r).unwrap();
            r.is_match(&f)
        }
        else {
            false
        }
    }

    fn valid_part2(&self) -> bool {
        if !Passport::valid_number_range(&self.byr, 1920, 2002) {
            return false;
        }

        if !Passport::valid_number_range(&self.iyr, 2010, 2020) {
            return false;
        }

        if !Passport::valid_number_range(&self.eyr, 2020, 2030) {
            return false;
        }

        if let Some(hgt) = &self.hgt {
            let v: String = hgt[0..hgt.len() - 2].to_string();
            if hgt.ends_with("cm") {
                if !Passport::valid_number_range(&Some(v), 150, 193) {
                    return false;
                }
            }
            else if hgt.ends_with("in") {
                if !Passport::valid_number_range(&Some(v), 59, 76) {
                    return false
                }
            }
            else {
                return false;
            }
        } else {
            return false;
        }

        if !Passport::valid_pattern(&self.hcl, r"^(#[0-9a-f]{6})$") {
            return false;
        }

        if !Passport::valid_pattern(&self.ecl, r"^(amb|blu|brn|gry|grn|hzl|oth)$") {
            return false;
        }

        if !Passport::valid_pattern(&self.pid, r"^\d{9}$") {
            return false;
        }

        true
    }
}

#[derive(Default)]
pub struct Day04 {
    passports: Vec<Passport>,
}

impl Day for Day04 {
    fn number(&self) -> u8 {
        4
    }

    fn setup(&mut self, input: &str) -> Result<()> {
        for entry in input.split("\r\n\r\n") {
            let mut p = Passport::default();

            for field in entry.split_whitespace() {
                let kv: Vec<_> = field.split(':').collect();
                let value = Some(kv[1].to_string());
                match kv[0] {
                    "byr" => p.byr = value,
                    "iyr" => p.iyr = value,
                    "eyr" => p.eyr = value,
                    "hgt" => p.hgt = value,
                    "hcl" => p.hcl = value,
                    "ecl" => p.ecl = value,
                    "pid" => p.pid = value,
                    "cid" => p.cid = value,
                    _ => return Err("Invalid key".into()),
                }
            }

            self.passports.push(p);
        }

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let count = self.passports.iter().filter(|p| p.valid_part1()).count();
        Ok(count.to_string())
    }

    fn part2(&self) -> Result<String> {
        let count = self.passports.iter().filter(|p| p.valid_part2()).count();
        Ok(count.to_string())
    }
}
