use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file!");

    let empty_line_regex = Regex::new(r"[\n|\r\n]{3,}").unwrap();
    let passports: Vec<&str> = empty_line_regex.split(&contents).collect();

    let re = regex::Regex::new(r"\s+").unwrap();

    let field_values: Vec<HashMap<&str, &str>> = passports
        .iter()
        .map(|passport| {
            // Unsplit fields with their values. Formatted as "field:value".
            let fields_with_values: Vec<&str> = re.split(passport).map(|s| s.trim()).collect();

            let fields_minus_cid: Vec<&str> = fields_with_values
                .iter()
                .filter(|field| field.split(":").next().unwrap() != "cid")
                .map(|field| *field)
                .collect();

            let mut map = HashMap::new();
            fields_minus_cid.iter().for_each(|pair| {
                let split: Vec<&str> = pair.split(':').collect();
                map.insert(split[0], split[1]);
            });

            map
        })
        .collect();

    let num_valid: usize = field_values
        .iter()
        .map(|pair| verify_map(pair))
        .filter(|valid| *valid)
        .count();

    println!("{}", num_valid);
}

fn verify_map(field_values: &HashMap<&str, &str>) -> bool {
    if field_values.len() < 7 {
        return false;
    }

    // Verify birthday year field "byr". Must be a 4 digit integer between 1920 and 2002, inclusive.
    let byr = field_values
        .get("byr")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0);

    if byr > 2002 || byr < 1920 {
        return false;
    }

    // Verify issue year field "iyr". Must be a 4 digit integer between 2010 and 2020, inclusive.
    let iyr = field_values
        .get("iyr")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0);

    if iyr > 2020 || iyr < 2010 {
        return false;
    }

    // Verify expiration year field "eyr". Must be a 4 digit integer between 2020 and 2030, inclusive.
    let eyr = field_values
        .get("eyr")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0);

    if eyr > 2030 || eyr < 2020 {
        return false;
    }

    // Verify height field "hgt".
    let hgt = field_values.get("hgt").unwrap();

    // Height must end with "cm" or "in".
    if !hgt.ends_with("cm") && !hgt.ends_with("in") {
        return false;
    }

    if hgt.ends_with("cm") {
        // Centimeters must be between 150 and 193
        let height_num = hgt
            .get(0..hgt.len() - 2)
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        if height_num < 150 || height_num > 193 {
            return false;
        }
    } else {
        // Inches must be between 59 and 76
        let height_num = hgt
            .get(0..hgt.len() - 2)
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        if height_num < 59 || height_num > 76 {
            return false;
        }
    }

    // Verify hair color field "hcl". Must be # followed by 6 characters [0-9 | a-f] (valid hex color code)
    let hair_color_regex = Regex::new(r"#[a-f|0-9]{6}$").unwrap();
    let hcl = field_values.get("hcl").unwrap();
    if !hair_color_regex.is_match(hcl) {
        return false;
    }

    // Verify eye color field "ecl". Must be one of the following exactly: "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    let ecl = field_values.get("ecl").unwrap();
    let is_valid_ecl = match ecl {
        &"amb" | &"blu" | &"brn" | &"gry" | &"grn" | &"hzl" | &"oth" => true,
        _ => false,
    };
    if !is_valid_ecl {
        return false;
    }

    // Verify passport id field "pid". Must be a 9 digit number (including leading zeroes).
    let passport_id_regex = Regex::new(r"[0-9]{9}$").unwrap();
    let pid = field_values.get("pid").unwrap();
    if !passport_id_regex.is_match(pid) {
        return false;
    }

    true
}
