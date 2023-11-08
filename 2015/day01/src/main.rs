fn elevator_level(s: &str) -> (i32, i32) {
    let r = 0;
    let s = s.chars().map(|c| if c == '(' { 1 } else { -1 } ).sum();
    (s,r)
}

#[test]
fn elevator() {
    // assert_eq!(1, elevator_level("("));
    // assert_eq!(0, elevator_level("(())"));
    // assert_eq!(0, elevator_level("()()"));
    // assert_eq!(3, elevator_level("))((((("));
    // assert_eq!(3, elevator_level("((("));
    // assert_eq!(3, elevator_level("(()(()("));

    // assert_eq!(-1, elevator_level("())"));
    // assert_eq!(-1, elevator_level("))("));

    // assert_eq!(-3, elevator_level(")))"));
    // assert_eq!(-3, elevator_level(")())())"));
}

fn main() {
    let i = include_str!("input.txt");
    println!("day01: {}", elevator_level(i));
}
