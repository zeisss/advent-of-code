use std::collections::HashMap;

fn time_for_char(s: &str) -> i32 {
	let keys = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	keys.chars().enumerate().filter(|(time, c)| c.to_string() == s).next().unwrap().0 as i32 + 1
}

#[test]
fn test_time() {
	assert_eq!(time_for_char("A"), 1);
	assert_eq!(time_for_char("C"), 3);
	assert_eq!(time_for_char("M"), 13);
	assert_eq!(time_for_char("Z"), 26);
}


#[derive(Debug)]
struct StateMachine<'a> {
	// assignments stores remaining work
	// <item string => ticks remaining>
	assignments: HashMap<&'a str, i32>,
	max_workers: usize,
}

impl<'a> StateMachine<'a> {
	fn new(workers: usize) -> Self {
		Self{
			max_workers: workers,
			assignments: HashMap::with_capacity(workers as usize),
		}
	}

	// is_empty returns true, if currently any assignment exists.
	fn is_empty(&self) -> bool {
		self.assignments.is_empty()
	}

	// assign tries to create a new assignment. Returns true on 
	// success.
	fn assign(&mut self, work_item: &'a str, ticks: i32) -> bool {
		if self.assignments.len() >= self.max_workers {
			return false
		}

		if self.is_inprogress(work_item) {
			return false
		}

		self.assignments.insert(work_item, ticks);
		true
	}

	// Tick decreases all remaining work and 
	// returns those that have finished.
	fn tick(&mut self) -> Vec<&'a str> {
		let mut result = vec![];
		// decrease all ticks by 1
		for (key, val) in self.assignments.iter_mut() {
			*val = *val - 1;
			if *val <= 0 {
				result.push(*key)
			}
		}

		// drop all done work
		self.assignments.retain(|_, &mut v| v > 0);

		result
	}

	fn is_inprogress(&self, worker_item: &str) -> bool {
		self.assignments.get(worker_item).is_some()
	}
}
#[test]
fn test_statemachine() {
	let mut s = StateMachine::new(1);
	assert_eq!(s.is_empty(), true);
	assert_eq!(s.assign("A", 1), true);
	assert_eq!(s.assign("B", 2), false);
	assert_eq!(s.is_empty(), false);

	assert_eq!(s.tick(), vec!["A"]);
	assert_eq!(s.is_empty(), true);

	let empty_result : Vec<&str> = vec![];
	assert_eq!(s.tick(), empty_result);
	assert_eq!(s.is_empty(), true);
}

#[test]
fn test_statemachine_two_jobs() {
	let mut s = StateMachine::new(2);
	assert_eq!(s.assign("A", 1), true);
	assert_eq!(s.assign("B", 2), true);

	assert_eq!(s.tick(), vec!["A"]);
	assert_eq!(s.tick(), vec!["B"]);

	let empty_result : Vec<&str> = vec![];
	assert_eq!(s.tick(), empty_result);
}

fn compile_with_time(workers: usize, time_mod: i32, steps: Vec<(&str, &str)>) -> (String, i32) {
	// let mut timed_steps : Vec<(&str, &str, i32)> = steps.iter().map(|(a,b)| (*a,*b, time_for_char(b))).collect();
	let mut id_pool : Vec<&str> = steps.iter().flat_map(|(a,b)| vec![a.clone(),b.clone()]).collect();
	id_pool.sort();
	// id_pool.reverse();
	id_pool.dedup();


	let mut result = String::from("");
	let mut all_dependencies_done : Vec<&str> = vec![];
	let mut workers = StateMachine::new(workers);

	let mut seconds = 0;
	while !workers.is_empty() || !id_pool.is_empty() {
		seconds = seconds + 1;
		if seconds > 50000 {
			break;
		}

		// let the workers work.
		let finished = workers.tick();
		all_dependencies_done.extend(finished);
		id_pool.retain(|i| !all_dependencies_done.contains(i));
		
		println!("# Second {}", seconds);
		println!("remaining:	{:?}", id_pool);
		println!("in progress:	{:?}", workers);
		println!("finished:	{:?}", all_dependencies_done);
		println!("");

		// look for the first ID with no dependencies
		for id in id_pool.iter() {
			// list dependencies, that are not already resolved
			let dependencies : Vec<&(&str, &str)> = steps
				.iter().filter(|(a, b)| {
					b == id &&
					!all_dependencies_done.contains(a)
				}).collect();
			if dependencies.is_empty() {
				if workers.is_inprogress(id) {
					println!("{:?} is in progress.", id);
					continue
				}
				println!("{:?} has no dependencies, adding to worker list", id);
				workers.assign(id, time_mod + time_for_char(id));

				// all_dependencies_done.push(id);
				// result = format!("{}{}", result, id);
			} else {
				println!("{:?} has {} dependencies: {:?}", id, dependencies.len(), dependencies);
			}
		}

		println!("\n\n");
	}
	(all_dependencies_done.join(""), seconds-1)
}

#[test]
fn test_compile_with_time_example() {
	let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
	let steps = parse(input);
	let (order, seconds) = compile_with_time(2, 0, steps);
	assert_eq!("CABFDE", order);
	assert_eq!(seconds, 15);
}

#[test]
fn test_compile_with_time_chain() {
	// B depends on A, but A depends on nothing
	let input = vec![("A", "B")];
	let (result, seconds) = compile_with_time(2, 0, input);
	assert_eq!("AB", result);
	assert_eq!(seconds, 3);
}

fn parse(s : &str) -> Vec<(&str, &str)> {
	s.lines().map(|line| {
		let w : Vec<&str> = line.split_whitespace().collect();
		(w[1], w[7])
	}).collect()
}

#[test]
fn test_parse() {
	let input = "Step C must be finished before step A can begin.";
	let r = parse(input);
	assert_eq!(1, r.len());
	assert_eq!(("C", "A"), r[0]);
}

// steps = Vec<(a, b)>
// a = str() - The dependency
// b = str() - the object that can be started after the dependency finished
fn compile<'a>(steps: Vec<(&str, &str)>) -> String {
	let mut id_pool : Vec<&str> = steps.iter().flat_map(|(a,b)| vec![a.clone(),b.clone()]).collect();
	id_pool.sort();
	// id_pool.reverse();
	id_pool.dedup();
	println!("All IDS: {:?}", id_pool);

	let mut result = String::from("");
	let mut all_dependencies_done : Vec<&str> = vec![];
	while !id_pool.is_empty() {
	// for _ in (0..10) {
		for id in id_pool.iter() {
			let dependencies : Vec<&(&str, &str)> = steps
				.iter().filter(|(a, b)| {
					b == id && 
					!all_dependencies_done.contains(a)
				}).collect();
			if dependencies.is_empty() {
				println!("{:?} has no dependencies, adding to done list", id);
				all_dependencies_done.push(id);
				result = format!("{}{}", result, id);
				break
			} else {
				println!("{:?} has {} dependencies: {:?}", id, dependencies.len(), dependencies);
			}
		}

		id_pool.retain(|i| !all_dependencies_done.contains(i));

		println!("remaining: {:?}", id_pool);
		
	}
	result
}

#[test]
fn test_compile_simple() {
	// B depends on A, but A depends on nothing
	let input = vec![("A", "B")];
	let result = compile(input);
	assert_eq!("AB", result);
}

#[test]
fn test_compile_chain() {
	// A depends on B, but B depends on nothing
	let input = vec![("B", "A"), ("C", "B")];
	let result = compile(input);
	assert_eq!("CBA", result);
}

fn main() {
    let input = include_str!("input.txt");
    let steps = parse(input);
	println!("{:?}", compile_with_time(5, 60, steps));
}

#[test]
fn test_example() {
	let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
	let steps = parse(input);
	let order = compile(steps);
	assert_eq!("CABDFE", order);
}