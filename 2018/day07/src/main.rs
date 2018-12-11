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
	let result = compile(parse(input));
	println!("{:?}", result);
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