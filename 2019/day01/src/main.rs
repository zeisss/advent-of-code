fn calc_required_fuel(mass: i32) -> i32 {
    let p1 = mass as f64 / 3.0;
    let p2 = p1 as i32;
    let p3 = p2 - 2;
    p3
}

#[test]
fn fuel_test()  {
    assert_eq!(2, calc_required_fuel(12));
    assert_eq!(2, calc_required_fuel(14));
    assert_eq!(654, calc_required_fuel(1969));
    assert_eq!(33583, calc_required_fuel(100756));
}

fn module_total_fuel(mass: i32) -> i32 {
    let mut total_mass : i32 = 0;
    let mut input = mass;
    loop {
        let fuel = calc_required_fuel(input);
        println!("{} + fuel({}) -> {}", total_mass, input, fuel);
        if fuel <= 0 {
            break
        }
        total_mass = total_mass + fuel;
        input = fuel;
    };
    println!("result = {}", total_mass);
    total_mass
    

}

#[test]
fn recursive_test() {
    // no additional fuel for these
    assert_eq!(2, module_total_fuel(12));
    assert_eq!(2, module_total_fuel(14));

    // but here
    assert_eq!(966, module_total_fuel(1969));
    assert_eq!(50346, module_total_fuel(100756));
}

fn main() {
    println!("1969: {}", module_total_fuel(1969));

    let input = include_str!("input.txt");
    let s : i32 = input.lines().
        flat_map(|s| s.parse::<i32>()).
        map(|i| calc_required_fuel(i)).
        map(|f| f+ module_total_fuel(f)).
        sum();
    println!("sum: {}", s);
}
// answer: 5322455
