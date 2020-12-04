use std::collections::HashMap;

use regex::Regex;

use crate::lib::advent::Result;
use crate::Day;

#[derive(Debug, Default)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn valid_part1(&self) -> bool {
        let keys: [String; 7] = [
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        ];

        keys.iter()
            .filter(|k| self.fields.contains_key(k.to_owned()))
            .count()
            == keys.len()
    }

    fn valid_part2(&self) -> bool {
        {
            let k_byr = "byr".to_string();
            if let Some(byr) = self.fields.get(&k_byr) {
                let byr: usize = byr.parse().unwrap();
                if byr < 1920 || byr > 2002 {
                    println!("{} byr", byr);
                    return false;
                }
            } else {
                return false;
            }
        }

        {
            let k_iyr = "iyr".to_string();
            if let Some(iyr) = self.fields.get(&k_iyr) {
                let iyr: usize = iyr.parse().unwrap();
                if iyr < 2010 || iyr > 2020 {
                    println!("{} iyr", iyr);
                    return false;
                }
            } else {
                return false;
            }
        }

        {
            let k_eyr = "eyr".to_string();
            if let Some(eyr) = self.fields.get(&k_eyr) {
                let eyr: usize = eyr.parse().unwrap();
                if eyr < 2020 || eyr > 2030 {
                    println!("{} eyr", eyr);
                    return false;
                }
            } else {
                return false;
            }
        }

        {
            let k_hgt = "hgt".to_string();
            if let Some(hgt) = self.fields.get(&k_hgt) {
                match hgt {
                    cm if hgt.ends_with("cm") => {
                        let cm: usize = cm[0..cm.len() - 2].parse().unwrap();
                        if cm < 150 || cm > 193 {
                            println!("{} cm", cm);
                            return false;
                        }
                    }
                    inches if hgt.ends_with("in") => {
                        let inches: usize = inches[0..inches.len() - 2].parse().unwrap();
                        if inches < 59 || inches > 76 {
                            println!("{} inches", inches);
                            return false;
                        }
                    }
                    _ => return false,
                }
            } else {
                return false;
            }
        }

        {
            let k_hcl = "hcl".to_string();
            if let Some(hcl) = self.fields.get(&k_hcl) {
                let r = Regex::new("#[0-9a-f]{6}").unwrap();
                if !r.is_match(hcl) {
                    println!("{} hcl", hcl);
                    return false;
                }
            } else {
                return false;
            }
        }

        {
            let k_ecl = "ecl".to_string();
            if let Some(ecl) = self.fields.get(&k_ecl) {
                let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                if !valid.contains(&ecl.as_str()) {
                    println!("{} ecl", ecl);
                    return false;
                }
            } else {
                return false;
            }
        }

        {
            let k_pid = "pid".to_string();
            if let Some(pid) = self.fields.get(&k_pid) {
                let r = Regex::new(r"\d{9}").unwrap();
                if !r.is_match(pid) {
                    println!("{} pid", pid);
                    return false;
                }
            } else {
                return false;
            }
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
                p.fields.insert(kv[0].to_string(), kv[1].to_string());
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
