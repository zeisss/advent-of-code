fn main() {
    let input = include_str!("input.txt");
    println!("Day03");
    println!("Part 1: {}", part1(input, 3, 1)); // NOT: 27
    println!("Part 2: {}", part2(input)); // 544 < x
}

fn part2(s :&str) -> u64 {
    let slides : Vec<(usize, usize)>= vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    slides.iter().map(|(dx, dy)| part1(s, *dx,*dy) as u64).product()
}

#[test]
fn test_simple() {
    assert_eq!(0, part1("..##\n#...", 3, 1));
    assert_eq!(1, part1("..##\n#..#", 3, 1));
    assert_eq!(1, part1("...\n#..", 3, 1));
}

#[test]
fn test_part1_example() {
    let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    assert_eq!(2, part1(input, 1, 1));
    assert_eq!(7, part1(input, 3, 1));
    assert_eq!(3, part1(input, 5, 1));
    assert_eq!(4, part1(input, 7, 1));
    assert_eq!(2, part1(input, 1, 2));

    assert_eq!(336, part2(input));
}

fn part1(input: &str, slope_x: usize, slope_y: usize) -> i32 {
    let l : Vec<&str> = input.lines().collect();
    positions(slope_x, slope_y, l.len())
        .into_iter()
        .filter(|(x, y)| is_tree(&l, *x, *y))
        .count() as i32
}

fn is_tree(lines: &Vec<&str>, x: usize, y: usize) -> bool {
    lines[y].chars().cycle().nth(x).unwrap() == '#'
}

// positions calculates all available positions the slope will hit.
fn positions(dx: usize, dy: usize, max: usize) -> Vec<(usize, usize)> {
    let mut pos = vec![];
    let mut x = dx;
    let mut y = dy;
    while y < max {
        if y > 0 {
            pos.push((x, y));
        }
        x += dx;
        y += dy;
    }
    pos
}

#[test]
fn test_positions() {
    assert_eq!(vec![(3, 1)], positions(3, 1, 2));
    assert_eq!(vec![(3, 1), (6, 2)], positions(3, 1, 3));
    assert_eq!(vec![(3, 1), (6, 2), (9, 3)], positions(3, 1, 4));
}
