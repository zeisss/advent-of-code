use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Claim {
	id: i32,
	x: i32,
	y: i32,
	w: i32,
	h: i32,
}

use std::num::ParseIntError;

impl std::str::FromStr for Claim {
	type Err = ParseIntError;
	fn from_str(line: &str) -> Result<Self,Self::Err> {
		let words : Vec<i32> = line
			.split_whitespace()
			.filter(|x| x.len() > 0)
			// split each word by these additional separators into more words
			.flat_map(|w| {
				w.split(|p| p == '#' || p == 'x' || p == '@' || p == ',' || p == '#' || p == ':')
			})
			.filter(|x| x.len() > 0)
			.map(|w| w.parse::<i32>())
			.collect::<Result<Vec<i32>, Self::Err>>()?;

		Ok(Self{
			id: words[0],
			x: words[1],
			y: words[2],
			w: words[3],
			h: words[4],
		})
	}
}

impl Claim {
	fn to_hashset(&self) -> HashSet<(i32, i32)> {
		let mut hs = HashSet::new();
		for w in 0..self.w {
			for h in 0..self.h {
				let coords = (self.x + w, self.y + h);
				hs.insert(coords);
			}
		}
		hs
	}
}

#[test]
fn parse_line() {
	assert_eq!("#1 @ 1,3: 4x4".parse::<Claim>(), Ok(Claim{
		id: 1,
		x: 1,
		y: 3,
		w: 4,
		h: 4,
	}));

	assert_eq!("						#3 @ 5,5: 2x2".parse::<Claim>(), Ok(Claim{
		id: 3,
		x: 5,
		y: 5,
		w: 2,
		h: 2,
	}));
}

fn process_str(input: &str) -> i32 {
	let claims : Vec<Claim> = input
			.lines()
			.map(|x| x.trim())
			.filter(|x| x.len() > 0)
			.flat_map(|l| l.parse::<Claim>())
			.collect()
			;

	let mut used_coords = HashSet::new();
	let mut conflicted_coords = HashSet::new();
	for claim in claims.iter() {
		let coords = claim.to_hashset();

		// mark conflicts, if both coordinate sets intersect
		for conflict in used_coords.intersection(&coords) {
			conflicted_coords.insert(conflict.clone());
		}
		// store coords for next checks
		for c in coords {
			used_coords.insert(c);
		}
	}

	// search for the one unique Claim
	for claim in claims.iter() {
		let coords = claim.to_hashset();
		if conflicted_coords.is_disjoint(&coords) {
			println!("No conflicts for {:?}", claim);
		}
	}
	conflicted_coords.len() as i32
}

fn main() {
	let input = include_str!("input.txt");
	let result = process_str(input);
	println!("conflicts: {}", result);
}

#[test]
fn test_input_1() {
	let input = include_str!("input.txt");
	let result = process_str(input);
	assert_eq!(result, 116140);

}
#[test]
fn test_examples() {
	let input = "#1 @ 1,3: 4x4
						#2 @ 3,1: 4x4
						#3 @ 5,5: 2x2";
	assert_eq!(process_str(input), 4);
}