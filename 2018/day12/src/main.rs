// we use Option for the plant state
// # => Option::Some(())
// . => Option::None

type PotId = i64;

#[derive(Eq,PartialEq,Clone,Copy,Debug)]
enum Pot {
	Plant,
	EmptyPot,
}

use Pot::*;

impl Pot {
	fn to_string(&self) -> String {
		match self {
		Pot::Plant => "#".into(),
		Pot::EmptyPot => ".".into(),
		}
	}

	fn parse(s: &str) -> Vec<Pot> {
		s.chars().flat_map(|ref c| match c {
		'#' => Some(Plant),
		'.' => Some(EmptyPot),
		_ => None,
		}).collect()
	}
}

#[derive(Debug,Eq, PartialEq)]
struct Rule {
	matcher: Vec<Pot>,
	result: Pot
}

#[derive(Debug,Eq, PartialEq)]
enum RuleErr {
	FieldCountMismatch,
	InitialStateMissing
}
impl Rule {
	fn parse(s: &str) -> Result<Rule, RuleErr> {
		let rules : Vec<Pot> = Pot::parse(s);

		if rules.len() != 6 {
			return Err(RuleErr::FieldCountMismatch)
		}

		let r = Rule{
			matcher: rules[0..5].to_vec(),
			result: rules[5],
		};
		// Err(RuleErr::ParsingFailed)
		Ok(r)
	}

	fn to_string(&self) -> String {
		let matcher = self.matcher.iter().map(|p| p.to_string()).collect::<String>();

		let result = match self.result {
		Plant => "#",
		EmptyPot => ".",
		};

		format!("{} => {}", matcher, result)
	}
}

#[test]
fn test_rule_parsing() {
	let foo = Rule::parse("...## => #");
	assert_ne!(Err(RuleErr::FieldCountMismatch), foo);
	assert_eq!(true, foo.is_ok());

	let rule = foo.unwrap();
	assert_eq!(EmptyPot, rule.matcher[0]);
	assert_eq!(EmptyPot, rule.matcher[1]);
	assert_eq!(EmptyPot, rule.matcher[2]);
	assert_eq!(Plant, rule.matcher[3]);
	assert_eq!(Plant, rule.matcher[4]);
	assert_eq!("...## => #", rule.to_string());

	let r = Rule::parse("..#.. => #");
	assert!(r.is_ok());
	assert_eq!("..#.. => #", r.unwrap().to_string());
}


use std::collections::HashMap;

struct State {
	turn: i32,
	pots: HashMap<PotId, Pot>,
	rules: Vec<Rule>,
}

impl State {
	fn parse(input: &str) -> Result<State, RuleErr> {
		let mut it = input.lines();
		let initial_state = it.next();

		if initial_state.is_none() {
			return Err(RuleErr::InitialStateMissing)
		}

		let rules : Vec<Rule> = it.skip(1).flat_map(Rule::parse).filter(|r| r.result == Plant).collect();

		let pots = {
			let mut pots : HashMap<PotId, Pot> = HashMap::new();
			Pot::parse(initial_state.unwrap()).iter().enumerate().for_each(|(i, p)| {
				pots.insert(i as PotId, *p);
			});
			pots
		};
		
		Ok(State{
			turn: 0,
			pots: pots,
			rules: rules,
		})
	}

	// finding_matching_rule looks in 
	fn find_matching_rule(&self, pot: PotId) -> Pot {
		let pots = vec![
			self.pots.get(&(pot-2)).map(|p| *p).unwrap_or(EmptyPot),
			self.pots.get(&(pot-1)).map(|p| *p).unwrap_or(EmptyPot),
			self.pots.get(&(pot)).map(|p| *p).unwrap_or(EmptyPot),
			self.pots.get(&(pot+1)).map(|p| *p).unwrap_or(EmptyPot),
			self.pots.get(&(pot+2)).map(|p| *p).unwrap_or(EmptyPot)
		];

		for r in self.rules.iter() {
			if r.matcher == pots {
				return r.result
			}
		}
		EmptyPot
	}

	fn min_pot_id(&self) -> PotId {
		let smallest_pot_id = self.pots.keys().min().unwrap();
		*smallest_pot_id
	}

	fn max_pot_id(&self) -> PotId {
		let max_pot_id = self.pots.keys().max().unwrap();
		*max_pot_id
	}

	fn pot_range(&self) -> std::ops::RangeInclusive<PotId> {
		let smallest_pot_id = self.min_pot_id();
		let max_pot_id = self.max_pot_id();

		((smallest_pot_id - 4)..=(max_pot_id+4))
	}

	fn turn(&mut self) {
		let mut new_pots : HashMap<PotId, Pot> = HashMap::with_capacity(self.pots.len() + 2);

		for pot_id in self.pot_range() {
			let result = self.find_matching_rule(pot_id);
			if Plant == result {
				new_pots.insert(pot_id, Plant);
			}
		}

		self.turn += 1;
		self.pots = new_pots;
	}

	fn turns(&mut self, turns: i64) {
		println!("{} ({} <-> {})", self.to_string(), self.min_pot_id(), self.max_pot_id());
		(0..turns).for_each(|_| {
			self.turn();
				println!("{} - {}", self.to_string(), self.sum_pots())
			
		});
		println!("{} ({} <-> {})", self.to_string(), self.min_pot_id(), self.max_pot_id())
	}

	fn sum_pots(&self) -> PotId {
		self.pots.keys().sum()
	}

	fn to_string(&self) -> String {
		let line = self.pot_range().map(|id| {
			self.pots.get(&id).unwrap_or(&EmptyPot).to_string()
		}).collect::<String>();
		format!("{}:  {}", self.turn, line)
	}
}

#[test]
fn test_find_matching_rule() {
	let state = State{
		turn: 0,
		pots: {
			let mut p : HashMap<PotId, Pot> = HashMap::new();
			p.insert(-2, EmptyPot);
			p.insert(-1, EmptyPot);
			p.insert(0, Plant);
			p.insert(1, EmptyPot);
			p.insert(2, EmptyPot);
			p
		},
		rules: vec![
			Rule{
				matcher: vec![EmptyPot, Plant, EmptyPot, EmptyPot, EmptyPot],
				result: Plant,
			}
		]
	};

	let r2 = state.find_matching_rule(1);
	assert!(r2.is_some());
	assert_eq!(Plant, r2.unwrap().result);

	let r = state.find_matching_rule(0);
	assert!(r.is_none());
}

#[test]
fn test_state_parse() {
	let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

	let mut state = State::parse(input).unwrap();
	assert_eq!(25, state.pots.len());
	assert_eq!(14, state.rules.len());
	assert_eq!(Plant, state.pots[&0]);
	assert_eq!("..#.. => #", state.rules[1].to_string());
	assert_eq!(".#... => #", state.rules[2].to_string());

	state.turn();
	assert_eq!("1:  ....#...#....#.....#..#..#..#....", state.to_string());
	state.turn();
	assert_eq!("2:  ....##..##...##....#..#..#..##....", state.to_string());

	state.turns(18);

	assert_eq!(325, state.sum_pots());

}

fn main() {
	let input = include_str!("input.txt");
	let mut state = State::parse(input).unwrap();
	state.turns(50_000_000_000); // 50_000_000_000
	println!("result: {:?}", state.sum_pots());
}
