use std::str::FromStr;

#[derive(Debug,Clone,PartialEq)]
struct Bank {
	fields: Vec<i32>,
}

impl Bank {
	fn from(input: Vec<i32>) -> Self {
		Bank{
			fields: input,
		}
	}

	fn sort(&mut self) {
		let len = self.fields.len();

		let (max_pos, max_val) : (i32, i32) = self.fields.iter().enumerate()
			.scan((0 as i32, 0 as i32), |state, (pos, val)| {
				//println!("{:?} {} {}", state, pos, val);
				if *val > state.1 {
					state.0 = pos as i32;
					state.1 = *val;
				};
				Some(*state)
			}).last().unwrap();

		//println!("pos={} val={}", max_pos, max_val);
		//println!("{:?} before", self.fields);
		self.fields[max_pos as usize] = 0;
		//println!("{:?} reset to 0", self.fields);
		for i in 0..max_val {
			self.fields[(1 + i + max_pos) as usize % len] += 1
		}

		// println!("{:?} after", self.fields);
	}
}

#[test]
fn test_sort() {
	let mut b = Bank::from(vec![0, 2, 7, 0]);
	b.sort();
	assert_eq!(vec![2, 4, 1, 2], b.fields);
	b.sort();
	assert_eq!(vec![3, 1, 2, 3], b.fields);
	b.sort();
	assert_eq!(vec![0, 2, 3, 4], b.fields);
	b.sort();
	assert_eq!(vec![1, 3, 4, 1], b.fields);
	b.sort();
	assert_eq!(vec![2, 4, 1, 2], b.fields);
}
fn redistribute(s: &str) -> i32 {
	let s : Vec<_> = s.split_whitespace().map(|s| i32::from_str(s).unwrap()).collect();
	let mut seen : Vec<Bank> = vec![];

	let mut bank = Bank::from(s);
	while !seen.contains(&bank) {
		seen.push(bank.clone());
		bank.sort();
		
		// println!("{:?} {:?}", seen, bank);
	}
	seen.len() as i32
}

#[test]
fn my_test() {
	assert_eq!(5, redistribute("0	2	7	0"));
}
fn main() {
	let input = "11	11	13	7	0	15	5	5	4	4	1	1	7	1	15	11";
    println!("{}", redistribute(input));
}
