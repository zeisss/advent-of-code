fn parse_line(s: &str) -> Option<(i32, i32, char, &str)> {
    let minus = s.find("-");
    let space = s.find(" ");
    let dc = s.find(": ");

    if minus.is_none() || space.is_none() || dc.is_none() {
        return None
    }

    let first = s[0..minus.unwrap()].parse::<i32>();
    let second = s[minus.unwrap()+1..space.unwrap()].parse::<i32>();
    let alphabet = &s[space.unwrap()+1..dc.unwrap()];
    let password = &s[dc.unwrap()+2..];

    if first.is_err() || second.is_err() {
        return None
    }

    Some((
        first.unwrap(),
        second.unwrap(),
        alphabet.chars().next().unwrap(),
        password,
    ))
}

#[test]
fn test_parse_line() {
    assert_eq!(Some((1,3, 'a', "abcde")), parse_line("1-3 a: abcde"));
    assert_eq!(Some((1,13, 'a', "abcdeabcdeabcdeabcdeabcde")), parse_line("01-13 a: abcdeabcdeabcdeabcdeabcde"));
    assert_eq!(None, parse_line("1-9 a asdafasdasfasd"))
}

fn parse(s: &str) -> Vec<(i32, i32, char, &str)> {
    s.split("\n")
        .flat_map(|s| parse_line(s.trim()))
        .collect::<Vec<_>>()
}

fn part1(s: &str) -> i32 {
    let p = parse(s);
    p.iter()
        .filter(|(a, b, c, password)| {
            let n = password.chars().filter(|c1| c1 == c).count();
            *a as usize <= n && n <= *b as usize
        })
        .count() as i32
}

// 206 358 x 463 492
fn part2(s: &str) -> i32 {
    let p = parse(s);
    p.iter()
        // .filter(|(a, _b, _c, password)| *a as usize <= password.len())
        // .filter(|(_a, b, _c, password)| *b as usize <= password.len())
        .filter(|(a, b, c, password)| {
            let a2 = *a - 1;
            let b2 = *b - 1;
            println!("{} {} {} {}", a, b, c, password);
            println!("{:?}", &password[a2 as usize..].chars().next().unwrap());
            println!("{:?}", &password[b2 as usize..].chars().next().unwrap());
            let f = password[a2 as usize..].chars().next().unwrap() == *c;
            let s = password[b2 as usize..].chars().next().unwrap() == *c;
            f && !s || s && !f 
        })
        .count() as i32
}



#[test]
fn part_test() {
    let input = "1-3 a: abcde
                1-3 b: cdefg
                2-9 c: ccccccccc";
    assert_eq!(2, part1(input));
    assert_eq!(1, part2(input));
    assert_eq!(0, part2("1-3 a: aba"));
    assert_eq!(1, part2("1-3 a: abb"));
    assert_eq!(1, part2("1-3 a: bba"));
}

fn main() {
    println!("Day 02");
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
