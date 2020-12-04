use std::{collections::HashMap, str::FromStr};

use crate::lib::advent::Result;
use crate::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Field {
    BirthYear,
    IssueYear,
    ExpiryYear,
    Height,
    HairColour,
    EyeColour,
    PassportID,
    CountryID,
}

impl FromStr for Field {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Field::*;
        match s {
            "byr" => Ok(BirthYear),
            "iyr" => Ok(IssueYear),
            "eyr" => Ok(ExpiryYear),
            "hgt" => Ok(Height),
            "hcl" => Ok(HairColour),
            "ecl" => Ok(EyeColour),
            "pid" => Ok(PassportID),
            "cid" => Ok(CountryID),
            _ => Err("Invalid Field"),
        }
    }
}

#[derive(Debug, Default)]
struct Passport<'a> {
    fields: HashMap<Field, &'a str>,
}

impl<'a> Passport<'a> {
    const REQUIRED: [Field; 7] = [
        Field::BirthYear,
        Field::IssueYear,
        Field::ExpiryYear,
        Field::Height,
        Field::HairColour,
        Field::EyeColour,
        Field::PassportID,
    ];

    fn valid_part1(&self) -> bool {
        Passport::REQUIRED
            .iter()
            .all(|f| self.fields.contains_key(f))
    }

    fn validate_field(f: Field, s: &str) -> bool {
        match f {
            Field::BirthYear => s
                .parse()
                .map(|year| 1920 <= year && year <= 2002)
                .unwrap_or_default(),
            Field::IssueYear => s
                .parse()
                .map(|year| 2010 <= year && year <= 2020)
                .unwrap_or_default(),
            Field::ExpiryYear => s
                .parse()
                .map(|year| 2020 <= year && year <= 2030)
                .unwrap_or_default(),
            Field::Height => s[..s.len() - 2]
                .parse()
                .map(|v| {
                    if s.ends_with("cm") {
                        150 <= v && v <= 193
                    } else if s.ends_with("in") {
                        59 <= v && v <= 76
                    } else {
                        false
                    }
                })
                .unwrap_or_default(),
            Field::HairColour => match s.as_bytes() {
                [b'#', end @ ..] => end.iter().all(|c| c.is_ascii_hexdigit()),
                _ => false,
            },
            Field::EyeColour => matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
            Field::PassportID => s.len() == 9 && s.chars().all(|c| c.is_ascii_digit()),
            _ => true,
        }
    }

    fn valid_part2(&self) -> bool {
        self.valid_part1() && self.fields.iter().all(|(f, s)| Self::validate_field(*f, s))
    }
}

#[derive(Default)]
pub struct Day04<'a> {
    passports: Vec<Passport<'a>>,
}

impl<'a> Day<'a> for Day04<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        for entry in input.split("\r\n\r\n") {
            let mut p = Passport::default();

            for field in entry.split_whitespace() {
                let kv: Vec<_> = field.split(':').collect();
                p.fields.insert(kv[0].parse()?, kv[1]);
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
