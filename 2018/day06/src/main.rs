type P = i32;
type Distance = i32;
type Pos = (P, P);

fn manhatten_distance(from: Pos, to: Pos) -> Distance {
	(from.0 as i16 - to.0 as i16).abs() as Distance + (from.1 as i16 - to.1 as i16).abs() as Distance
}

#[test]
fn test_manhatten_distance() {
	assert_eq!(0, manhatten_distance((0,0), (0,0)));
	assert_eq!(2, manhatten_distance((0,0), (1,1)));
	assert_eq!(1, manhatten_distance((0,0), (1,0)));
	assert_eq!(1, manhatten_distance((0,0), (0,1)));

	assert_eq!(50 + 75, manhatten_distance((100,100), (50,25)));
	assert_eq!(50 + 75, manhatten_distance((50,25), (100,100)));
}

use std::collections::*;

fn calculate_distances(inputs: &Vec<Pos>) -> (HashMap<Pos, Distance>, HashSet<Pos>) {
	let mut areas = std::collections::HashMap::new();
	let mut infinites = std::collections::HashSet::new();

	for x in -400..=1000 {
		for y in -400..=1000 {
			let mut distances : Vec<(Distance, Pos)> = inputs.iter()
				// calculate the distance
				.map(|coord| {
					let distance = manhatten_distance((x,y), *coord);
					(distance, *coord)
				}).collect();
			distances.sort();

			if distances[0].0 != distances[1].0 { // make sure we have only one shortest path
				let shortest_coord = distances[0].1.clone();

				areas.entry(shortest_coord)
					.and_modify(|e| *e += 1)
					.or_insert(1);

				if x == 0 || y == 0 || x == 1000 || y == 1000 {
					infinites.insert(shortest_coord);
				}
			}


		}
	}
	(areas, infinites)
}

fn calculate_areas(limit: Distance, inputs: &Vec<Pos>) -> Distance {
	let mut result = 0;
	for x in -400..=1000 {
		for y in -400..=1000 {
			let s : Distance = inputs.iter().map(|&coord| {
				manhatten_distance(coord, (x, y))
			}).sum();

			if s < limit {
				result += 1;
			}
		}
	}
	result
}

fn largest_area(inputs: &Vec<Pos>) -> P {
	let (areas, infinites) = calculate_distances(inputs);
	
	areas.into_iter()
			.filter(|(coord, _dist)| {
				!infinites.contains(coord)
			})
			.map(|(coord, dist)| (dist, coord)) // reverse the entry structure in the map, so we can sort by distance
			.max_by_key(|(dist, _coord)| dist.clone())
			.unwrap().0
}

#[test]
fn test_examples() {
	let coords = vec![
		(1, 1), // A
		(1, 6), // B
		(8, 3), // C
		(3, 4), // D
		(5, 5), // E
		(8, 9), // F
	];

	let (a, infinites) = calculate_distances(&coords);
	assert_eq!(coords.len(), a.len());
	assert_eq!(9, a[&(3,4)]);
	assert_eq!(17, a[&(5,5)]);

	assert_eq!(4, infinites.len());
	assert!(infinites.contains(&(1,1)));
	assert!(infinites.contains(&(1,6)));
	assert!(infinites.contains(&(8,3)));
	assert!(infinites.contains(&(8,9)));
	assert!(!infinites.contains(&(3,4)));
	assert!(!infinites.contains(&(5,5)));

	assert_eq!(17, largest_area(&coords));
	assert_eq!(coords.clone(), parse("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"));

	assert_eq!(16, calculate_areas(32, &coords));

}


fn parse(s: &str) -> Vec<Pos> {
	s.lines().map(|l| {
		let t : Vec<P> = l.split(",").map(|c| c.trim().parse::<P>().unwrap()).collect();
		(t[0], t[1])
	}).collect()
}

#[test]
fn test_parse() {
	assert_eq!(vec![(0,0), (1,1)], parse("0, 0\n1, 1\n"));
}

fn main() {
    let input = include_str!("input.txt");
    let coords = parse(input);
    let r = largest_area(&coords);
    println!("result = {:?}", r);

    println!("area count = {:?}", calculate_areas(10_000, &coords));

    // 5246 - too high
    // 5105 - too high
    // 2271 - wrong
}
