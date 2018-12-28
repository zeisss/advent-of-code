use std::collections::HashMap;

type Player = i32;

#[derive(Debug)]
struct Playground {
	data: Vec<i32>,
	current_marble_index: usize,
	scores: HashMap<Player, i32>
}

impl Playground {
	fn new() -> Self {
		Self{
			data:  vec![0],
			current_marble_index: 0,
			scores: HashMap::new(),
		}
	}

	fn to_string(&self) -> String {
		let mut s = "".to_string();

		self.data.iter().enumerate().for_each(|(i, v)| {
			if i == self.current_marble_index {
				s += &format!("({})", v).to_string();
			} else {
				s += &format!(" {} ", v).to_string();
			}
		});

		s
	}

	fn cycle_index(&self, delta: i32) -> usize {
		let index : i32 = (self.current_marble_index as i32 + delta);
		(if index < 0 {
			println!("UNDERFLOW! {}", index);
			(index + self.data.len() as i32) as usize
		} else {
			(index as usize % self.data.len())
		})
	}

	fn place(&mut self, player: Player, marble: i32) {
		//println!("before:\t{} + {}", self.to_string(), marble);

		if marble % 23 == 0 {
			self.score(player, marble);

			let marble_to_remove = self.cycle_index(-7);
			let marble_value = self.data.remove(marble_to_remove);
			self.score(player, marble_value);

			self.current_marble_index = marble_to_remove;

		} else {
			let mut pos : usize = self.cycle_index(1);
			self.data.insert(pos + 1, marble);
			self.current_marble_index = pos + 1;
		}
		//println!("after:\t{}", self.to_string());
	}

	fn score(&mut self, player: Player, marble_score: i32) {
		println!("Player {} gets +{}pt", player, marble_score);
		self.scores.entry(player).and_modify(|e| *e += marble_score).or_insert(marble_score);
	}

	fn highest_score(self) -> (Player, i32) {
		self.scores.into_iter().max_by_key(|(p, score)| *score).unwrap()
	}
}

#[test]
fn test_playground_scores() {
	let mut p = Playground::new();
	p.score(1, 100);

	assert_eq!(100, p.scores[&1]);
	assert_eq!((1, 100), p.highest_score());
}

#[test]
fn test_playground() {
	let mut p = Playground::new();
	assert_eq!("(0)", p.to_string());
	p.place(1, 1);
	assert_eq!(" 0 (1)", p.to_string());
	p.place(2, 2);
	assert_eq!(" 0 (2) 1 ", p.to_string());
	
	p.place(3, 3);
	assert_eq!(" 0  2  1 (3)", p.to_string());
	p.place(4, 4);
	assert_eq!(" 0 (4) 2  1  3 ", p.to_string());
	p.place(5, 5);
	assert_eq!(" 0  4  2 (5) 1  3 ", p.to_string());
}

fn play(players: i32, last_marble_points: i32) -> i32 {
	let mut player = (1..=players).cycle();

	let mut marble_pool : Vec<i32> = (1..=last_marble_points).collect();
	marble_pool.sort();

	let mut playground = Playground::new();
	println!("[-]\t{}", playground.to_string());
	while !marble_pool.is_empty() {
		let lowest_marble = marble_pool.remove(0);

		let current_player = player.next().unwrap();
		playground.place(current_player, lowest_marble);
		// println!("[{:?}]\t{}", current_player, playground.to_string());
	}

	println!("\nFinal\n[-]\t{}", playground.to_string());
	let (p, score) = playground.highest_score();
	score
}

#[test]
fn test_simple_example() {
	assert_eq!(32, play(9, 25));
}

#[test]
fn test_web_examples() {
	// https://www.reddit.com/r/adventofcode/comments/a4ipsk/day_9_part_one_my_code_works_for_the_worked/ebetbcr/
	assert_eq!(63, play(9, 48));
	assert_eq!(95, play(1, 48));
}

#[test]
fn test_more_examples() {
	assert_eq!(8317, play(10, 1618));
	assert_eq!(146373, play(13, 7999));
	assert_eq!(2764, play(17, 1104));
	assert_eq!(54718, play(21, 6111));
	assert_eq!(37305, play(30, 5807));
}

fn main() {
    let r = play(466, 71436);
    println!("result = {:?}", r);

    // 383795 -- too high
    // 382055 -- too high
}
