fn parse(s: &str) -> Vec<i32> {
    s.split("\n")
        .filter_map(|s| s.trim().parse::<i32>().ok())
        // .inspect(|x| println!("{:?}", x))
        .collect()
}

fn part1(s: &str) -> i32 {
    let values = parse(s);
    values.clone().into_iter()
        .flat_map(|a| {
            values.clone().into_iter().map(move |b| (a, b))
        })
        // .inspect(|x| println!("{:?}", x))
        .flat_map(|(a, b)| { if a + b == 2020 { Some(a * b) } else { None }})
        .next()
        .unwrap()
}

fn part2(s: &str) -> i32 {
    let values = parse(s);
    
    values.clone().into_iter()
        .flat_map(move |a| {
            let v2 = values.clone();
            values.clone().into_iter().flat_map(move |b| {
                v2.clone().into_iter().map(move |c| 
                    (a.clone(), b.clone(), c.clone())
                )
            })
        })
        // .for_each(|x| println!("{:?}", x))
        .flat_map(|(a, b, c)| { if a + b + c == 2020 { Some(a * b * c) } else { None }})
        .next()
        .unwrap()
}


#[test]
fn part1test() {
    let input = "1721
    979
    366
    299
    675
    1456";
    assert_eq!(514579, part1(input));
    assert_eq!(241861950, part2(input));
}

fn main() {
    println!("Day 01");
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part1: {}", part2(input));
}
