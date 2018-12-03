use std::collections::HashMap;

trait Reducer {
	fn process<I>(input: I) -> i32
		where I: Iterator<Item=i32> + std::clone::Clone;
}

struct SumReducer {}

impl Reducer for SumReducer {
	fn process<I>(input: I) -> i32
		where I: Iterator<Item=i32> + std::clone::Clone {
		let mut freq = 0;
		// process the iterator once, then finish
		for i in input {
			freq += i;
		};

		freq
	}
}

struct DuplicateFreqReducer {}
impl Reducer for DuplicateFreqReducer {
	fn process<I>(input: I) -> i32
		where I: Iterator<Item=i32> + std::clone::Clone {

		let mut freq = 0;
		let mut lookup = HashMap::new();
		lookup.insert(freq, true); // mark the 0 frequency too

		for i in input.cycle() {
			freq = freq + i;

			let r = lookup.insert(freq, true);
			match r {
			Some(_) => { return freq },
			_ => continue
			}
		};

		panic!("Endless loop ended. cycle() broken?")

	}
}
#[test]
fn test_duplicate_finder_simple() {
	assert_eq!(process_str::<DuplicateFreqReducer>("+1 -1"), 0);
}

fn process_str<T>(input: &str) -> i32 
	where T: Reducer {
	let it = input
			.split_whitespace()
			.filter(|x| x.len() > 0)
			.map(|x| x.parse::<i32>().unwrap());

	T::process(it)
}

fn main() {
	let input = include_str!("input.txt");
	let result = process_str::<SumReducer>(input);
	println!("sum frequency: {}", result);

	let dup = process_str::<DuplicateFreqReducer>(input);
	println!("duplicate frequency: {}", dup);
}


#[test]
fn test_simple_sums() {
	assert_eq!(process_str::<SumReducer>("+1 +2"), 3);
	assert_eq!(process_str::<SumReducer>("+1 +2 -4"), -1);
}

#[test]
fn test_input_sum() {
	let input = include_str!("input.txt");
	assert_eq!(process_str::<SumReducer>(input), 416);	
}