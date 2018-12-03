use std::collections::HashMap;

trait Reducer {
	fn process<'a, I>(input: I) -> i32
		where I: Iterator<Item=&'a str> + std::clone::Clone;
}

struct FirstReducer {}

impl FirstReducer {
	fn has_duplicate_letters(input: &str) -> (bool, bool) {
		let mut counter = [0; 256];
		let mut has_twos = false;
		let mut has_threes = false;

		for c in input.chars() {
			// println!("c={}", c);
			let idx = c as usize;
			counter[idx] = counter[idx] + 1;
		}

		for value in counter.iter() {
			match *value {
			2 => has_twos = true,
			3 => has_threes = true,
			_ => {},
			}
		}
		(has_twos, has_threes)
	}
}
impl Reducer for FirstReducer {
	fn process<'a, I>(input: I) -> i32
		where I: Iterator<Item=&'a str> + std::clone::Clone {
		
		let mut twos = 0;
		let mut threes = 0;

		for line in input {
			let (has_twos, has_threes) = Self::has_duplicate_letters(line);
			if has_twos {
				twos = twos + 1;
			}
			if has_threes {
				threes = threes + 1;
			}
		}

		twos * threes

	}
}

struct SecondReducer {}
impl SecondReducer {
	fn equality(left: &str, right: &str) -> i32 {
		(left.len() as i32) - left.chars().zip(right.chars()).map(|(x,y)| {
			if x == y {
				1
			} else {
				0
			}
		}).sum::<i32>()
	}
}
impl Reducer for SecondReducer {
	fn process<'a, I>(input: I) -> i32
		where I: Iterator<Item=&'a str> + std::clone::Clone {
			let c = input.clone();

			for line_left in input {
				for line_right in c.clone() {
					let m = SecondReducer::equality(line_left, line_right);

					if m == 1 {
						println!("{}", line_left);
					}
				}
			}



			-1
	}
}
#[test]
fn test_equality() {
	assert_eq!(SecondReducer::equality("abcde", "axcye"), 2);
	assert_eq!(SecondReducer::equality("fguij", "fghij"), 1);
	assert_eq!(SecondReducer::equality("abcde", "fghij"), 5);
	// assert_eq!(process_str::<SecondReducer>("+1 -1"), 0);
}

fn process_str<T>(input: &str) -> i32 
	where T: Reducer {
	let it = input
			.split_whitespace()
			.filter(|x| x.len() > 0)
			;
			// .map(|x| x.parse::<i32>().unwrap());

	T::process(it)
}

fn main() {
	let input = include_str!("input.txt");
	let result = process_str::<FirstReducer>(input);
	println!("checksum: {}", result);

	let dup = process_str::<SecondReducer>(input);
	println!("checksum: {}", dup);
}


#[test]
fn test_examples() {
	let input = "abcdef bababc abbcde abcccd aabcdd abcdee ababab";
	assert_eq!(process_str::<FirstReducer>(input), 12);
}