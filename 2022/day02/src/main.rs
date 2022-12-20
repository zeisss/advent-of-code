
#[derive(PartialEq, Debug)]
pub enum Symbol {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq)]
pub struct Round {
    Enemy: Symbol,
    Player: Symbol,
}

mod parser {
    pub fn parse(input: &str) -> Vec<crate::Round> {
        use crate::{Round, Symbol};

        println!("Hello, world!");

        input.lines().map(|line| {
            Round{
                Enemy: match line.chars().nth(0).unwrap() {
                'A' => Symbol::Rock,
                'B' => Symbol::Paper,
                'C' => Symbol::Scissor,
                unknown => panic!("unsupported symbol: {}", unknown),
                },
                Player: match line.chars().nth(2).unwrap() {
                'X' => Symbol::Rock,
                'Y' => Symbol::Paper,
                'Z' => Symbol::Scissor,
                unknown => panic!("unsupported symbol: {}", unknown),
                },
            }
        }).collect()
    }


    #[test]
    fn test_examples() {
        use crate::Symbol;

        let input = "A Y
B X
C Z";
        let data = parse(input);

        assert_eq!(Symbol::Rock, data[0].Enemy);
        assert_eq!(Symbol::Paper, data[0].Player);

        assert_eq!(Symbol::Paper, data[1].Enemy);
        assert_eq!(Symbol::Rock, data[1].Player);

        assert_eq!(Symbol::Scissor, data[2].Enemy);
        assert_eq!(Symbol::Scissor, data[2].Player);
    }
}

mod ruleset {
    use crate::{Round, Symbol, Outcome};

    fn eval(player: Symbol, enemy: Symbol) -> Outcome {
        if enemy == player {
            Outcome::Draw
        } else if enemy == Symbol::Rock && player == Symbol::Scissor || enemy == Symbol::Paper && player == Symbol::Rock || enemy == Symbol::Scissor && player == Symbol::Paper {
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

fn part1(input: Vec<Round>) -> u32 {
    input.into_iter().map(|r| ruleset::score_round(r.Player, r.Enemy)).sum()
}

#[test]
fn test_examples() {
    let input = "A Y
B X
C Z";
    let data = parser::parse(input);

    assert_eq!(15, part1(data))
}

fn main() {
    let input = include_str!("input.txt");
    let data = parser::parse(input);

    println!("Part 1: {}", part1(data));
}
