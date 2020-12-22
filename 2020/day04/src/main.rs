#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Passport {
    byr: i64,
    iyr: i64,
    eyr: i64,    // expiration year
    hgt: String, // height
    hcl: String, // hair color
    ecl: String, // eye color
    pid: String,
    cid: i64,
}

impl std::str::FromStr for Passport {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let acc: Passport = Passport {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: "".into(),
            hcl: "".into(),
            ecl: "".into(),
            pid: "".into(),
            cid: 0,
        };
        let p = s
            .split_ascii_whitespace()
            .map(|s| {
                let mut i = s.split(":");
                let key = i.next().unwrap();
                let value = i.next().unwrap();

                (key, value)
            })
            .fold(acc, |mut acc, (key, value)| {
                match key {
                    "byr" => acc.byr = value.parse::<i64>().unwrap_or(0),
                    "iyr" => acc.iyr = value.parse::<i64>().unwrap_or(0),
                    "eyr" => acc.eyr = value.parse::<i64>().unwrap_or(0),
                    "hgt" => acc.hgt = value.into(),
                    "hcl" => acc.hcl = value.into(),
                    "ecl" => acc.ecl = value.into(),
                    "pid" => acc.pid = value.into(), // .parse::<i64>().unwrap_or(0),
                    "cid" => acc.cid = value.parse::<i64>().unwrap_or(0),
                    _ => println!("{} {} unexpected field", key, value),
                };
                acc
            });
        Ok(p)
    }
}
impl Passport {
    fn all_required_fields(&self) -> bool {
        self.byr > 0
            && self.iyr > 0
            && self.eyr > 0
            && self.hgt != ""
            && self.hcl != ""
            && self.ecl != ""
            && self.pid != ""
    }

    fn is_valid(&self) -> bool {
        self.all_required_fields()
            && (1920..2003).contains(&self.byr)
            && (2010..2021).contains(&self.iyr)
            && (2020..2031).contains(&self.eyr)
            && Passport::is_valid_height(&self.hgt)
            && Passport::is_valid_eyecolor(&self.ecl)
            && Passport::is_valid_color(&self.hcl)
            && Passport::is_valid_passportcode(&self.pid)
    }

    fn is_valid_eyecolor(s: &str) -> bool {
        let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        colors.contains(&s)
    }
    fn is_valid_color(s: &str) -> bool {
        let alphabet = vec![
            'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ];
        let length = s
            .char_indices()
            .filter(|x| match x {
                (0, '#') => true,
                (n, c) if (1..7).contains(n) && alphabet.contains(c) => true,
                _ => false,
            })
            .count();
        length == 7
    }
    fn is_valid_height(s: &str) -> bool {
        let unit: &str = if s.ends_with("cm") { "cm" } else { "in" };
        let size: i32 = s.strip_suffix(unit).unwrap_or("0").parse().unwrap();

        match (size, unit) {
            (size, "cm") if size >= 150 && size <= 193 => true,
            (size, "in") if size >= 59 && size <= 76 => true,
            _ => false,
        }
    }
    fn is_valid_passportcode(s: &str) -> bool {
        let alphabet = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let length = s.chars().filter(|c| alphabet.contains(c)).count();
        length == 9
    }
}

#[test]
fn test_examples() {
    let mut input =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
    assert!(input.parse::<Passport>().unwrap().all_required_fields());

    assert_eq!(
        false,
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"
            .parse::<Passport>()
            .unwrap()
            .all_required_fields()
    );

    input = "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
    assert!(input.parse::<Passport>().unwrap().all_required_fields());

    input = "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n";
    assert_eq!(
        false,
        input.parse::<Passport>().unwrap().all_required_fields()
    );
}

fn part1(s: &str) -> i32 {
    s.split("\n\n")
        .map(|s| s.parse::<Passport>().unwrap())
        .filter(|p| p.all_required_fields())
        .count() as i32
}

#[test]
fn part2_field_examples() {
    assert!(Passport::is_valid_height("60in"));
    assert!(Passport::is_valid_height("190cm"));
    assert!(!Passport::is_valid_height("190in"));
    assert!(!Passport::is_valid_height("190"));

    assert!(Passport::is_valid_color("#123abc"));
    assert!(!Passport::is_valid_color("#123abz"));
    assert!(!Passport::is_valid_color("123abc"));

    assert!(Passport::is_valid_eyecolor("brn"));
    assert!(!Passport::is_valid_eyecolor("wat"));

    assert!(Passport::is_valid_passportcode("000000001"));
    assert!(!Passport::is_valid_passportcode("0123456789"));
    assert!(!Passport::is_valid_passportcode("186cm"));
}

#[test]
fn part2_test_invalid() {
    assert_eq!(
        false,
        "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            .parse::<Passport>()
            .unwrap()
            .is_valid()
    );

    assert_eq!(
        false,
        "hcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946"
            .parse::<Passport>()
            .unwrap()
            .is_valid()
    );

    assert_eq!(
        false,
        "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            .parse::<Passport>()
            .unwrap()
            .is_valid()
    );

    assert_eq!(
        false,
        "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"
            .parse::<Passport>()
            .unwrap()
            .is_valid()
    );
}

#[test]
fn part2_test_valid() {
    let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f

    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022

    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(4, part2(input));
}

fn part2(s: &str) -> i32 {
    s.split("\n\n")
        .map(|s| s.parse::<Passport>().unwrap())
        .filter(|p| p.is_valid())
        .count() as i32
}

fn main() {
    println!("Day 04");
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input)); // 154 too low
}
