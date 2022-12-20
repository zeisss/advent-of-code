#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Symbol {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

mod parser {
    use crate::{Outcome, Symbol};

    pub struct Line {
        pub left: char,
        pub right: char,
    }

    pub fn parse_symbol(input: char) -> Symbol {
        match input {
            'A' | 'X' => Symbol::Rock,
            'B' | 'Y' => Symbol::Paper,
            'C' | 'Z' => Symbol::Scissor,
            unknown => panic!("unsupported symbol: {}", unknown),
        }
    }

    pub fn parse_outcome(input: char) -> Outcome {
        match input {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            unknown => panic!("unsupported outcome: {}", unknown),
        }
    }

    pub fn parse(input: &str) -> Vec<Line> {
        input
            .lines()
            .map(|line| Line {
                left: line.chars().nth(0).unwrap(),
                right: line.chars().nth(2).unwrap(),
            })
            .collect()
    }
}

mod ruleset {
    use crate::{Outcome, Symbol};

    fn wins_against(s: Symbol) -> Symbol {
        use crate::Symbol::*;
        match s {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
        }
    }

    fn loses_against(s: Symbol) -> Symbol {
        use crate::Symbol::*;
        match s {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
        }
    }

    fn eval(player: Symbol, enemy: Symbol) -> Outcome {
        if enemy == player {
            Outcome::Draw
        } else if wins_against(enemy) == player {
            Outcome::Loss
        } else {
            Outcome::Win
        }
    }

    pub fn score_round(player: Symbol, enemy: Symbol) -> u32 {
        let score = match player {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissor => 3,
        };
        let score2 = match eval(player, enemy) {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        score + score2
    }

    pub fn for_outcome(enemy: Symbol, outcome: Outcome) -> Symbol {
        match outcome {
            Outcome::Win => loses_against(enemy),
            Outcome::Loss => wins_against(enemy), // pick the losing move
            Outcome::Draw => enemy,
        }
    }

    #[test]
    fn test_eval() {
        assert_eq!(Outcome::Loss, eval(Symbol::Rock, Symbol::Paper));
        assert_eq!(Outcome::Win, eval(Symbol::Rock, Symbol::Scissor));
        assert_eq!(Outcome::Draw, eval(Symbol::Rock, Symbol::Rock));

        assert_eq!(Outcome::Win, eval(Symbol::Paper, Symbol::Rock));
        assert_eq!(Outcome::Draw, eval(Symbol::Paper, Symbol::Paper));
        assert_eq!(Outcome::Loss, eval(Symbol::Paper, Symbol::Scissor));

        assert_eq!(Outcome::Loss, eval(Symbol::Scissor, Symbol::Rock));
        assert_eq!(Outcome::Win, eval(Symbol::Scissor, Symbol::Paper));
        assert_eq!(Outcome::Draw, eval(Symbol::Scissor, Symbol::Scissor));
    }

    #[test]
    fn test_score_round() {
        assert_eq!(8, score_round(Symbol::Paper, Symbol::Rock));
        assert_eq!(1, score_round(Symbol::Rock, Symbol::Paper));
        assert_eq!(6, score_round(Symbol::Scissor, Symbol::Scissor))
    }
}

fn part1(input: &Vec<parser::Line>) -> u32 {
    input
        .into_iter()
        .map(|line| {
            ruleset::score_round(
                parser::parse_symbol(line.right),
                parser::parse_symbol(line.left),
            )
        })
        .sum()
}

fn part2(input: &Vec<parser::Line>) -> u32 {
    input
        .into_iter()
        .map(|line| {
            let enemy = parser::parse_symbol(line.left);
            let player = ruleset::for_outcome(enemy, parser::parse_outcome(line.right));
            ruleset::score_round(player, enemy)
        })
        .sum()
}

#[test]
fn test_examples() {
    let input = "A Y
B X
C Z";
    let data = parser::parse(input);

    assert_eq!(15, part1(&data));
    assert_eq!(12, part2(&data));
}

fn main() {
    let input = include_str!("input.txt");
    let data = parser::parse(input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
