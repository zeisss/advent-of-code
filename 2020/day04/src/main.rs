#[derive(Debug,Eq,PartialEq,PartialOrd,Ord)]
struct Passport {
    byr: i64,
    iyr: i64,
    eyr: i64,     // expiration year
    hgt: String, // height
    hcl: String, // hair color
    ecl: String, // eye color
    pid: String,
    cid: i64,
}

impl std::str::FromStr for Passport {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let acc : Passport = Passport{
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
    fn is_valid(&self) -> bool {
        self.byr > 0
            && self.iyr > 0
            && self.eyr > 0
            && self.hgt != ""
            && self.hcl != ""
            && self.ecl != ""
            && self.pid != ""
    }
}

#[test]
fn test_examples() {
    let mut input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
    assert!(input.parse::<Passport>().unwrap().is_valid());

    assert_eq!(false, "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929".parse::<Passport>().unwrap().is_valid());

    input = "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm";
    assert!(input.parse::<Passport>().unwrap().is_valid());

    input = "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n";
    assert_eq!(false, input.parse::<Passport>().unwrap().is_valid());
}

fn part1(s: &str) -> i32 {
    s.split("\n\n")
        .map(|s| s.parse::<Passport>().unwrap())
        .inspect(|x| if !x.is_valid() { println!("invalid: {:?}", x); })
        .filter(|p| p.is_valid())
        .count() as i32
}

fn main() {
    println!("Day 04");
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}
