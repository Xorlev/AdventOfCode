use failure::_core::str::FromStr;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

use util::aoc::*;

lazy_static! {
    static ref ALLOWED_EYE_COLORS: HashSet<String> =
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
    static ref ALLOWED_HAIR_COLOR: Regex = Regex::new("^#[a-f0-9]{6}$").unwrap();
    static ref ALLOWED_PASSPORT_ID: Regex = Regex::new("^\\d{9}$").unwrap();
    static ref REQUIRED_FIELDS: HashSet<PassportPropertyType> = vec![
        PassportPropertyType::BirthYear,
        PassportPropertyType::IssueYear,
        PassportPropertyType::ExpirationYear,
        PassportPropertyType::Height,
        PassportPropertyType::HairColor,
        PassportPropertyType::EyeColor,
        PassportPropertyType::PassportId,
    ]
    .into_iter()
    .collect();
}

fn main() -> AocResult<()> {
    let input = input::read_all(4)?;
    let passports: Vec<Passport> = input
        .split("\n\n")
        .map(|passport_block| {
            passport_block
                .split(|c| c == '\n' || c == ' ')
                .collect::<Vec<_>>()
                .parse::<PassportProperty>()
                .unwrap()
        })
        .map(Passport::new)
        .collect();

    println!("{:?}", passports);

    result("Part 1", || part1(&passports));
    result("Part 2", || part2(&passports));

    Ok(())
}

fn part1(passports: &[Passport]) -> i32 {
    passports.iter().filter(|p| p.is_valid(false)).count() as i32
}

fn part2(passports: &[Passport]) -> i32 {
    passports.iter().filter(|p| p.is_valid(true)).count() as i32
}

#[derive(Debug)]
struct Passport {
    properties: Vec<PassportProperty>,
    property_types: HashSet<PassportPropertyType>,
}

impl Passport {
    fn new(properties: Vec<PassportProperty>) -> Passport {
        Passport {
            property_types: properties.iter().map(|p| p.to_property_type()).collect(),
            properties,
        }
    }

    fn is_valid(&self, validate_fields: bool) -> bool {
        let all_fields_present = REQUIRED_FIELDS.difference(&self.property_types).count() == 0;

        if validate_fields {
            all_fields_present && self.properties.iter().all(|p| p.is_valid())
        } else {
            all_fields_present
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum PassportPropertyType {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

#[derive(Debug, Clone)]
enum Height {
    Centimeters(u32),
    Inches(u32),
    Unknown(String),
}

#[derive(Debug, Clone)]
enum PassportProperty {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(Height),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String),
}

impl PassportProperty {
    fn to_property_type(&self) -> PassportPropertyType {
        match self {
            PassportProperty::BirthYear(_) => PassportPropertyType::BirthYear,
            PassportProperty::IssueYear(_) => PassportPropertyType::IssueYear,
            PassportProperty::ExpirationYear(_) => PassportPropertyType::ExpirationYear,
            PassportProperty::Height(_) => PassportPropertyType::Height,
            PassportProperty::HairColor(_) => PassportPropertyType::HairColor,
            PassportProperty::EyeColor(_) => PassportPropertyType::EyeColor,
            PassportProperty::PassportId(_) => PassportPropertyType::PassportId,
            PassportProperty::CountryId(_) => PassportPropertyType::CountryId,
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            PassportProperty::BirthYear(year) => *year >= 1920 && *year <= 2002,
            PassportProperty::IssueYear(year) => *year >= 2010 && *year <= 2020,
            PassportProperty::ExpirationYear(year) => *year >= 2020 && *year <= 2030,
            PassportProperty::Height(height) => match height {
                Height::Centimeters(height_cm) => *height_cm >= 150 && *height_cm <= 193,
                Height::Inches(height_in) => *height_in >= 59 && *height_in <= 76,
                Height::Unknown(_) => false,
            },
            PassportProperty::HairColor(color) => ALLOWED_HAIR_COLOR.is_match(color),
            PassportProperty::EyeColor(color) => ALLOWED_EYE_COLORS.contains(color),
            PassportProperty::PassportId(id) => ALLOWED_PASSPORT_ID.is_match(id),
            PassportProperty::CountryId(_) => true,
        }
    }
}

impl FromStr for PassportProperty {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<_>>();
        if parts.len() != 2 {
            failure::bail!("Expected key:value pair, found: {:?}", parts);
        }

        let kv = match parts[0] {
            "byr" => PassportProperty::BirthYear(parts[1].parse()?),
            "iyr" => PassportProperty::IssueYear(parts[1].parse()?),
            "eyr" => PassportProperty::ExpirationYear(parts[1].parse()?),
            "hgt" => PassportProperty::Height(parts[1].parse()?),
            "hcl" => PassportProperty::HairColor(parts[1].to_string()),
            "ecl" => PassportProperty::EyeColor(parts[1].to_string()),
            "pid" => PassportProperty::PassportId(parts[1].to_string()),
            "cid" => PassportProperty::CountryId(parts[1].to_string()),
            _ => failure::bail!("Unknown key: {}", parts[0]),
        };

        Ok(kv)
    }
}

impl FromStr for Height {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect_vec();

        let height = match chars[chars.len() - 2..].iter().join("").as_str() {
            "in" => Height::Inches(chars[..chars.len() - 2].iter().join("").parse()?),
            "cm" => Height::Centimeters(chars[..chars.len() - 2].iter().join("").parse()?),
            input @ _ => Height::Unknown(input.to_string()),
        };

        Ok(height)
    }
}
