use std::num::ParseIntError;

// A Memory Address
pub type Position = usize;
pub type Value = i32;
pub type Memory = Vec<Value>;

const OP_CODE_ADD: i32 = 1;
const OP_CODE_MULTIPLY: i32 = 2;
const OP_CODE_READ_INPUT: i32 = 3;
const OP_CODE_WRITE_OUTPUT: i32 = 4;
const OP_CODE_JMP_IF_TRUE: i32 = 5;
const OP_CODE_JMP_IF_FALSE: i32 = 6;
const OP_CODE_LESS_THAN: i32 = 7;
const OP_CODE_EQUAL: i32 = 8;
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
    pub output: Vec<Value>,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

// decode_instruction decodes the value and returns the op code and parameter mode
// for each argument (see day05).
fn decode_instruction(value: Value) -> (i32, ParameterMode, ParameterMode, ParameterMode) {
    use ParameterMode::*;

    // ABCDE
    // DE - opcode
    let op_code = value % 100;

    match value - op_code {
        0 => (op_code, Position, Position, Position),
        100 => (op_code, Immediate, Position, Position),
        1000 => (op_code, Position, Immediate, Position),
        1100 => (op_code, Immediate, Immediate, Position),
        10000 => (op_code, Position, Position, Immediate),
        10100 => (op_code, Immediate, Position, Immediate),
        11000 => (op_code, Position, Immediate, Immediate),
        11100 => (op_code, Immediate, Immediate, Immediate),
        _ => panic!("unknwon op_code modifiers"),
    }
}

fn read_address(mem: &Memory, addr: Position, mode: ParameterMode) -> Value {
    let raw_value = mem.get(addr).unwrap();

    if mode == ParameterMode::Immediate {
        *raw_value
    } else {
        let addr: usize = (*raw_value).try_into().unwrap();
        *mem.get(addr).unwrap()
    }
}

fn read_target(mem: &Memory, addr: Position) -> Value {
    let raw_value = mem.get(addr).unwrap();
    *raw_value
}

pub fn execute(memory: Memory, inputs: Vec<Value>) -> ExecuteResult {
    let mut ps: Position = 0;
    let mut mem = memory;
    let mut inputs = inputs.clone();
    inputs.reverse(); // So we can pop() them in order
    let mut output = Vec::new();

    loop {
        println!("MEMORY @ {}: {:?}", ps, mem);
        let (op_code, mode1, mode2, _mode3) = decode_instruction(*mem.get(ps).unwrap());
        match op_code {
            OP_CODE_ADD => {
                let value1 = read_address(&mem, ps + 1, mode1);
                let value2 = read_address(&mem, ps + 2, mode2);
                let target = read_target(&mem, ps + 3) as usize;

                // println!("SET {:?} <= {:?} + {:?}", ps + 3, value1, value2);
                mem[target] = value1 + value2;
                ps += 4;
            }
            OP_CODE_MULTIPLY => {
                let value1 = read_address(&mem, ps + 1, mode1);
                let value2 = read_address(&mem, ps + 2, mode2);
                let target = read_target(&mem, ps + 3) as usize;

                // println!("SET {:?} <= {:?} * {:?}", ps + 3, value1, value2);
                mem[target] = value1 * value2;
                ps += 4;
            }
            OP_CODE_READ_INPUT => {
                let next_input = inputs.pop().unwrap();
                let target = read_target(&mem, ps + 1) as usize;
                // println!("SET {:?} <= {:?}", target, next_input);
                mem[target] = next_input;
                ps += 2;
            },
            OP_CODE_WRITE_OUTPUT => {
                let value = read_address(&mem, ps + 1, mode1);
                output.push(value);
                ps += 2;
            },

            OP_CODE_JMP_IF_TRUE => {
                let value1 = read_address(&mem, ps + 1, mode1);
                let target = read_address(&mem, ps + 2,mode2) as usize;

                if value1 != 0 {
                    ps = target;
                } else {
                    ps += 3;
                }
            },
            OP_CODE_JMP_IF_FALSE => {
                let value1 = read_address(&mem, ps + 1, mode1);
                let target = read_address(&mem, ps + 2, mode2) as usize;

                if value1 == 0 {
                    ps = target;
                } else {
                    ps += 3;
                }
            },

            OP_CODE_LESS_THAN => {
                let value1 = read_address(&mem, ps + 1, mode1);
                let value2 = read_address(&mem, ps + 2, mode2);
                let target = read_target(&mem, ps + 3) as usize;

                // println!("SET {:?} <= {:?} == {:?}", ps + 3, value1, value2);
                mem[target] = if value1 < value2 { 1 } else { 0 };
                ps += 4;
            }
            OP_CODE_EQUAL => {
                let value1 = read_address(&mem, ps + 1, mode1);
                let value2 = read_address(&mem, ps + 2, mode2);
                let target = read_target(&mem, ps + 3) as usize;

                // println!("SET {:?} <= {:?} == {:?}", ps + 3, value1, value2);
                mem[target] = if value1 == value2 { 1 } else { 0 };
                ps += 4;
            }
            OP_CODE_QUIT_PROGRAM => break,
            _ => panic!("unknown op code: {:?}", op_code),
        }
    }
    // println!("MEMORY: {:?}", mem);

    ExecuteResult {
        memory: mem,
        output: output,
    }
}

#[cfg(test)]
mod tests {
    use crate::computer::*;

    fn run(input: &str) -> ExecuteResult {
        let memory = parse_program(input).unwrap();
        execute(memory, vec![])
    }
    fn run_with_input(input: &str, i: Vec<Value>) -> ExecuteResult {
        let memory = parse_program(input).unwrap();
        execute(memory, i)
    }

    fn run_single_io(input: &str, i: i32) -> i32 {
        let memory = parse_program(input).unwrap();
        let result = execute(memory, vec![i]);
        result.output[0]
    }

    #[test]
    fn test_parse_program() {
        let data = parse_program("1,1,2,3,99").unwrap();
        assert_eq!(data, vec![1, 1, 2, 3, 99]);
    }

    #[test]
    fn test_example_day02() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let result = run(input);
        assert_eq!(
            result.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_example_day02_extra() {
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

    #[test]
    fn test_example_day05_extra() {
        let result = run("1101,100,-1,4,0");
        assert_eq!(result.memory, vec![1101, 100, -1, 4, 99]);

        let result = run_with_input("3,2,0", vec![99]);
        assert_eq!(result.memory, vec![3, 2, 99]);

        let result = run("4,2,99");
        assert_eq!(result.output, vec![99]);
    }

    #[test]
    fn test_day05_op_code_equal() {
        // Position Mode
        // Compare input with 8 and print 1 if true or zero if false
        let input = "3,9,8,9,10,9,4,9,99,-1,8";

        let result = run_with_input(input, vec![7]);
        assert_eq!(result.output, vec![0]);

        let result = run_with_input(input, vec![8]);
        assert_eq!(result.output, vec![1]);

        let result = run_with_input(input, vec![9]);
        assert_eq!(result.output, vec![0]);

        // Immediate Mode
        let input = "3,3,1108,-1,8,3,4,3,99";
        let result = run_with_input(input, vec![7]);
        assert_eq!(result.output, vec![0]);

        let result = run_with_input(input, vec![8]);
        assert_eq!(result.output, vec![1]);

        let result = run_with_input(input, vec![9]);
        assert_eq!(result.output, vec![0]);
    }

    #[test]
    fn test_day05_op_code_less_than() {
        // Position Mode
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let result = run_with_input(input, vec![7]);
        assert_eq!(result.output, vec![1]);

        let result = run_with_input(input, vec![8]);
        assert_eq!(result.output, vec![0]);

        let result = run_with_input(input, vec![9]);
        assert_eq!(result.output, vec![0]);

        // Immediate Mode
        let input = "3,3,1107,-1,8,3,4,3,99";
        let result = run_with_input(input, vec![7]);
        assert_eq!(result.output, vec![1]);

        let result = run_with_input(input, vec![8]);
        assert_eq!(result.output, vec![0]);

        let result = run_with_input(input, vec![9]);
        assert_eq!(result.output, vec![0]);
    }

    #[test]
    fn test_day05_op_code_jmp() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"; // position mode
        let input2 = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"; // immediate mode
        
        assert_eq!(0, run_single_io(input, 0));
        assert_eq!(1, run_single_io(input, 1));
        assert_eq!(1, run_single_io(input, 2));
        
        assert_eq!(0, run_single_io(input2, 0));
        assert_eq!(1, run_single_io(input2, 1));
        assert_eq!(1, run_single_io(input2, 2));
    }

    #[test]
    fn test_day05_complex() {
        let large = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, run_single_io(large, 7));
        assert_eq!(1000, run_single_io(large, 8));
        assert_eq!(1001, run_single_io(large, 9));
    }

    #[test]
    fn test_decode_instruction() {
        let (op_code, b, c, d) = decode_instruction(99);
        assert_eq!(op_code, 99);
        assert_eq!(b, ParameterMode::Position);
        assert_eq!(c, ParameterMode::Position);
        assert_eq!(d, ParameterMode::Position);

        let (op_code, b, c, d) = decode_instruction(1022);
        assert_eq!(op_code, 22);
        assert_eq!(b, ParameterMode::Position);
        assert_eq!(c, ParameterMode::Immediate);
        assert_eq!(d, ParameterMode::Position);

        let (op_code, b, c, d) = decode_instruction(11120);
        assert_eq!(op_code, 20);
        assert_eq!(b, ParameterMode::Immediate);
        assert_eq!(c, ParameterMode::Immediate);
        assert_eq!(d, ParameterMode::Immediate);

        let (op_code, b, c, d) = decode_instruction(10120);
        assert_eq!(op_code, 20);
        assert_eq!(b, ParameterMode::Immediate);
        assert_eq!(c, ParameterMode::Position);
        assert_eq!(d, ParameterMode::Immediate);

        let (op_code, b, c, d) = decode_instruction(10020);
        assert_eq!(op_code, 20);
        assert_eq!(b, ParameterMode::Position);
        assert_eq!(c, ParameterMode::Position);
        assert_eq!(d, ParameterMode::Immediate);
    }
}
