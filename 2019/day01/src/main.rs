fn mass_to_fuel(mass: i32) -> i32 {
    let p1 = mass as f64 / 3.0;
    let p2 = p1 as i32;
    let p3 = p2 - 2;
    p3
}

#[test]
fn fuel_test()  {
    assert_eq!(2, mass_to_fuel(12));
    assert_eq!(2, mass_to_fuel(14));
    assert_eq!(654, mass_to_fuel(1969));
    assert_eq!(33583, mass_to_fuel(100756));
}

fn module_total_fuel(mass: i32) -> i32 {
    let f = mass_to_fuel(mass);
    if f <= 0 {
        0
    } else {
        f + module_total_fuel(f)
    }
}

// fn module_total_fuel(mass: i32) -> i32 {
//     let mut total_mass : i32 = 0;
//     let mut input = mass;
//     loop {
//         let fuel = mass_to_fuel(input);
//         if fuel <= 0 {
//             break
//         }
//         total_mass = total_mass + fuel;
//         input = fuel;
//     };
//     total_mass
// }

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
    let input = include_str!("input.txt");
    let s : i32 = input.lines().
        flat_map(|s| s.parse::<i32>()).
        map(|i| mass_to_fuel(i)).
        map(|f| f + module_total_fuel(f)).
        sum();
    println!("sum: {}", s);
    assert_eq!(5322455, s);
}
// answer: 5322455
