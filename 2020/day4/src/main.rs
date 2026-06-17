use anyhow::{Result, bail};
use std::collections::HashMap;
use std::io::BufRead;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let passports = parse_input()?;

    let p1 = passports.iter().filter(|p| p.valid()).count();
    println!("Part 1: {p1}");
    let p2 = passports
        .iter()
        .filter(|p| p.valid_strict().is_ok())
        .count();
    println!("Part 2: {p2}");

    Ok(())
}

fn parse_input() -> Result<Vec<Passport>> {
    let mut passports = Vec::new();

    let mut passport = Passport::default();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            passports.push(passport);
            passport = Passport::default();
        } else {
            let fields: Vec<_> = line.split_whitespace().collect();
            for field in fields {
                let key_value: Vec<_> = field.split(':').collect();
                passport
                    .fields
                    .insert(key_value[0].to_string(), key_value[1].to_string());
            }
        }
    }
    passports.push(passport);

    Ok(passports)
}

#[derive(Debug, Default)]
struct Passport {
    fields: HashMap<String, String>,
}

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    fn valid(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|&req| self.fields.contains_key(req))
    }

    fn valid_strict(&self) -> Result<()> {
        if !self.valid() {
            bail!("Invalid passport: not all fields provided");
        }

        for &field in REQUIRED_FIELDS {
            let val = &self.fields[field].trim();
            if val.is_empty() {
                bail!("Field {field} not given value")
            }

            match field {
                "byr" => valid_year(val, 1920..=2002)?,
                "iyr" => valid_year(val, 2010..=2020)?,
                "eyr" => valid_year(val, 2020..=2030)?,
                "hgt" => valid_height(val)?,
                "hcl" => valid_hair_color(val)?,
                "ecl" => valid_eye_color(val)?,
                "pid" => valid_passport_id(val)?,
                _ => {}
            }
        }

        Ok(())
    }
}

fn valid_year(year: &str, range: RangeInclusive<usize>) -> Result<()> {
    if year.len() != 4 {
        bail!("Year '{}' must be 4 digits", year);
    }

    if !range.contains(&year.parse()?) {
        bail!("Year not in range: {range:?}");
    }

    Ok(())
}

fn valid_height(height: &str) -> Result<()> {
    if let Some(height) = height.strip_suffix("cm") {
        let height: i32 = height.parse()?;
        if !(150..=193).contains(&height) {
            bail!("Invalid height: {height}");
        }
    } else if let Some(height) = height.strip_suffix("in") {
        let height: i32 = height.parse()?;
        if !(59..=76).contains(&height) {
            bail!("Invalid height: {height}");
        }
    } else {
        bail!("Invalid height: {height}");
    }

    Ok(())
}

fn valid_hair_color(color: &str) -> Result<()> {
    if !color.starts_with("#") {
        bail!("Color must start with \"#\"");
    }

    if color.chars().count() != 7 {
        bail!("Color must be 6 digits");
    }

    if !color
        .chars()
        .skip(1)
        .all(|c| matches!(c, '0'..='9' | 'a'..='f'))
    {
        bail!("Color must contain only hex digits")
    }

    Ok(())
}

fn valid_eye_color(color: &str) -> Result<()> {
    if !matches!(color, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
        bail!("Invalid color: {color}");
    }

    Ok(())
}

fn valid_passport_id(pid: &str) -> Result<()> {
    if pid.len() != 9 {
        bail!("PID must be 9 digits");
    }

    pid.parse::<usize>()?;

    Ok(())
}
