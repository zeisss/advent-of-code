use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::LinkedList;

type Player = i32;
type MarbleScore = i64;

#[derive(Debug)]
struct Playground {
	data: LinkedList<MarbleScore>,
	scores: HashMap<Player, MarbleScore>
}

impl Playground {
	fn new() -> Self {
		let mut data = LinkedList::new();
		data.push_back(0);
		Self{
			data:  data,
			scores: HashMap::new(),
		}
	}

	fn to_string(&self) -> String {
		let mut s = "".to_string();

		let current_marble : i32 = self.data.len() as i32 - 2;
		self.data.iter().enumerate().for_each(|(i, v)| {
			if i == current_marble as usize {
				s += &format!("({})", v).to_string();
			} else {
				s += &format!(" {} ", v).to_string();
			}
		});

		s
	}

	fn place(&mut self, player: Player, marble: MarbleScore) {
		//println!("before:\t{} + {}", self.to_string(), marble);

		if marble % 23 == 0 {
			// the player keeps the marble itself
			self.score(player, marble);

			// Then the marble -7 positions is removed as well 
			let pos = self.data.len() - 6;
			let mut a = self.data.split_off(pos); // the new front

			let pos2 = self.data.len() - 2;
			let mut d = self.data.split_off(pos2); // the new insertion position
			let c = self.data.pop_back().unwrap(); // the marble to be removed

			// Build the new marble playground from a + b + d
			a.append(&mut self.data);
			a.append(&mut d);
			self.data = a;

			// and scored by the player
			self.score(player, c);
		} else {
			self.data.push_back(marble);

			let first = self.data.pop_front().unwrap();
			self.data.push_back(first);
		}
		//println!("after:\t{}", self.to_string());
	}

	fn score(&mut self, player: Player, marble_score: MarbleScore) {
		// println!("Player {} gets +{}pt", player, marble_score);
		self.scores.entry(player).and_modify(|e| *e += marble_score).or_insert(marble_score);
	}

	fn highest_score(self) -> (Player, MarbleScore) {
		self.scores.into_iter().max_by_key(|(_p, score)| *score).unwrap()
	}
}

#[test]
fn test_playground_scores() {
	let mut p = Playground::new();
	p.score(1, 100);

	assert_eq!(100, p.scores[&1]);
	assert_eq!((1, 100), p.highest_score());
}
/*
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
*/
fn play(players: Player, last_marble_points: MarbleScore) -> MarbleScore {
	let mut player = (1..=players).cycle();

	let mut marble_pool : VecDeque<MarbleScore> = {
		let bag : Vec<MarbleScore> = (1..=last_marble_points).collect();
		let mut t = VecDeque::new();
		t.extend(bag.into_iter());
		t
	};

	let mut playground = Playground::new();
	println!("Marble\tPlayer\tPlayground");
	println!("#0\t[/]\t{}", playground.to_string());
	while !marble_pool.is_empty() {
		let next_marble = marble_pool.pop_front().unwrap();

		let current_player = player.next().unwrap();
		playground.place(current_player, next_marble);
		// println!("#{}\t[{:?}]\t{}", next_marble, current_player, playground.to_string());
	}

	// println!("\nFinal\n[-]\t{}", playground.to_string());
	let (_p, score) = playground.highest_score();
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
    let r = play(466, 71436 * 5);
    println!("result = {:?}", r);

    // 383795 -- too high
    // 382055 -- too high

    let r = play(466, 71436 * 100);
    println!("result = {:?}", r);
}
