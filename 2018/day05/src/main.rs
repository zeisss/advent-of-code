fn is_reacting(left : &str, right : &str) -> bool {
	let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	let lower = "abcdefghijklmnopqrstuvwxyz";

	upper
		.chars()
		.zip(lower.chars())
		.map(|(u, l)| (u.to_string(), l.to_string()))
		.filter(|(u, l)| (u == left || u == right) && (l == left || l == right))
		.next()
		.is_some()
}

#[test]
fn test_is_reacting() {
	assert_eq!(true, is_reacting("a", "A"));
	assert_eq!(true, is_reacting("z", "Z"));
	assert_eq!(true, is_reacting("A", "a"));
	assert_eq!(true, is_reacting("Z", "z"));

	assert_eq!(false, is_reacting("a", "a"));
	assert_eq!(false, is_reacting("A", "A"));
	assert_eq!(false, is_reacting("a", "z"));
	assert_eq!(false, is_reacting("a", "Z"));
}


fn react(s: &str) -> String {
	println!("input: {:?}", s);


	let mut input = s.to_string().into_bytes();


	let mut index : usize = 0;


	while true {
		if index + 1 >= input.len() {
			break;
		}

		let first = String::from_utf8(vec![input[index]]).unwrap();
		let second = String::from_utf8(vec![input[index + 1]]).unwrap();
		if is_reacting(&first, &second) {
			// println!("reacting! {:?} {:?}", first, second);

			// remove index twice, since the second value moves up
			input.remove(index);
			input.remove(index);

			if index > 0 {
				index = index - 1;
			}
		} else {
			index = index + 1;
		}
	}

	println!("result: {:?}", input);
	String::from_utf8(input).unwrap()
}

#[test]
fn test_examples() {
	assert_eq!("", react("aA"));
	assert_eq!("", react("abBA"));
	assert_eq!("abAB", react("abAB"));
	assert_eq!("aabAAB", react("aabAAB"));
	assert_eq!("dabCBAcaDA", react("dabAcCaCBAcCcaDA"));
}

fn main() {
    let input = include_str!("input.txt");
    let result = react(input);
    println!("result: {:?}", result);
    println!("len:    {:?}", result.len());
}
