#[derive(Debug, PartialEq)]
struct Node {
	children: Vec<Node>,
	metadata: Vec<i32>,
}

impl Node {
	fn from_iterator<I>(it : &mut I) -> Self
		where I: Iterator<Item=i32> {

		let child_count = it.next().expect("Expected children count");
		let metadata_count = it.next().expect("Invalid format - expected metadata counter");

		let mut children = vec![];
		for _ in 0..child_count {
			let child = Node::from_iterator(it);
			children.push(child);
		};

		let metadata = it.take(metadata_count as usize).collect();

		Node{
			children: children,
			metadata: metadata,
		}
	}
	fn from_vec(n : Vec<i32>) -> Self {
		let mut it = n.into_iter();
		Node::from_iterator(&mut it)
	}
	fn sum(&self) -> i32 {
		let mut s = 0;
		for c in self.children.iter() {
			s = s + c.sum();
		}
		for m in self.metadata.iter() {
			s = s + m;
		}
		s
	}

	fn value(&self) -> i32 {
		println!("-> value(self)");

		// If a node has no child nodes, its value is 
		// the sum of its metadata entries. 
		let value : i32 = if self.children.is_empty() {
			self.metadata.iter().sum()
		} else {
			let mut v = 0;
			for m in self.metadata.iter() {
				// the metadata indexes are 1-based, our self.children is 0-based
				let c = self.children.get(*m as usize - 1);
				if c.is_some() {
					let m_value = c.unwrap().value();
					println!("	adding value {} for metadata {}", m_value, m);
					v = v + m_value;
				} else {
					println!("	metadata {} referenced non-child", m);
				}
			};
			v
		};
		println!("<- value is {:?}", value);
		value
	}
}

#[test]
fn test_value() {
	/*let d = parse_str("0 1 99");
	assert_eq!(d.value(), 99);

	let c = parse_str("1 1 0 1 99 2");
	assert_eq!(c.value(), 0);

	let b = parse_str("0 3 10 11 12");
	assert_eq!(b.value(), 33);*/

	let a = parse_str("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
	assert_eq!(a.value(), 66);

}
#[test]
fn test_sum() {
	let n = Node{children: vec![], metadata: vec![1,2,3]};
	assert_eq!(n.sum(), 6);

	let p = Node{children: vec![n], metadata: vec![10]};
	assert_eq!(p.sum(), 16);
}

#[test]
fn test_from_vec_metadata() {
	let n = Node::from_vec(vec![0,1,10]);
	assert_eq!(n.sum(), 10);

	let n2 = Node::from_vec(vec![0,2,10, 13]);
	assert_eq!(n2.sum(), 23);
}

#[test]
fn test_from_vec_children() {
	let n = Node::from_vec(vec![0,0]);
	assert_eq!(n, Node{
		children: vec![],
		metadata: vec![],
	});

	let n2 = Node::from_vec(vec![1,0,0,0]);
	assert_eq!(n2, Node{
		children: vec![
			Node{
				children: vec![],
				metadata: vec![],
			}
		],
		metadata: vec![],
	});

}

fn parse_str(s : &str) -> Node {
	let n : Vec<i32> = s.split_whitespace().flat_map(|s| s.parse::<i32>()).collect();
	Node::from_vec(n)
}

fn main() {
	let input = include_str!("input.txt");
    let r = parse_str(input);
    println!("sum={:?}", r.sum());
    println!("value={:?}", r.value());
}

#[test]
fn test_example() {
	let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
	let r = parse_str(input);
	assert_eq!(r.sum(), 138);
	assert_eq!(r.value(), 66);
}