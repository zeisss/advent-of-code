use std::num::ParseIntError;

// A Memory Address
pub type Position = usize;
pub type Value = i32;
pub type Memory = Vec<Value>;

const OP_CODE_ADD: i32 = 1;
const OP_CODE_MULTIPLY: i32 = 2;
const OP_CODE_QUIT_PROGRAM: i32 = 99;

#[derive(PartialEq, Debug)]
pub enum Error {
    ParseError(ParseIntError),
}

pub fn parse_program(input: &str) -> Result<Memory, Error> {
    input
        .split(",")
        .map(|s| s.parse::<i32>().map_err(|err| Error::ParseError(err)))
        .collect()
}

pub struct ExecuteResult {
    pub memory: Memory,
}

pub fn execute(memory: Memory) -> ExecuteResult {
    let mut ps: Position = 0;
    let mut mem = memory;

    loop {
        let op_code = mem.get(ps);
        match op_code {
            Some(&OP_CODE_ADD) => {
                let address1: usize = (*mem.get(ps + 1).unwrap()).try_into().unwrap();
                let address2: usize = (*mem.get(ps + 2).unwrap()).try_into().unwrap();
                let target_address: usize = (*mem.get(ps + 3).unwrap()).try_into().unwrap();

                let value1 = mem.get(address1).unwrap();
                let value2 = mem.get(address2).unwrap();

                // println!("SET {:?} <= {:?} + {:?}", ps + 3, value1, value2);
                mem[target_address] = value1 + value2;
                ps += 4;
            }
            Some(&OP_CODE_MULTIPLY) => {
                let address1: usize = (*mem.get(ps + 1).unwrap()).try_into().unwrap();
                let address2: usize = (*mem.get(ps + 2).unwrap()).try_into().unwrap();
                let target_address: usize = (*mem.get(ps + 3).unwrap()).try_into().unwrap();

                let value1 = mem.get(address1).unwrap();
                let value2 = mem.get(address2).unwrap();

                // println!("SET {:?} <= {:?} + {:?}", ps + 3, value1, value2);
                mem[target_address] = value1 * value2;
                ps += 4;
            }
            Some(&OP_CODE_QUIT_PROGRAM) => break,
            _ => panic!("unknown op code: {:?}", op_code),
        }

        // println!("MEMORY: {:?}", mem);
    }

    ExecuteResult { memory: mem }
}

#[cfg(test)]
mod tests {
    use crate::computer::*;

    fn run(input: &str) -> ExecuteResult {
        let memory = parse_program(input).unwrap();
        execute(memory)
    }

    #[test]
    fn test_parse_program() {
        let data = parse_program("1,1,2,3,99").unwrap();
        assert_eq!(data, vec![1, 1, 2, 3, 99]);
    }

    #[test]
    fn test_example_one() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let result = run(input);
        assert_eq!(
            result.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_example_additional() {
        let result = run("99");
        assert_eq!(result.memory, vec![99]);

        let result = run("1,0,0,0,99");
        assert_eq!(result.memory, vec![2, 0, 0, 0, 99]);

        let result = run("2,3,0,3,99");
        assert_eq!(result.memory, vec![2, 3, 0, 6, 99]);

        let result = run("2,4,4,5,99,0");
        assert_eq!(result.memory, vec![2, 4, 4, 5, 99, 9801]);

        let result = run("1,1,1,4,99,5,6,0,99");
        assert_eq!(result.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
