struct IntcodeMachine {
    memory: Vec<usize>,
}

impl IntcodeMachine {
    fn parse(s : &str) -> Self {
        let memory = s.split_terminator(",").flat_map(|s| s.parse::<usize>()).collect();
        Self{
            memory: memory,
        }
    }

    fn run(mut self) -> Self {
        let mut pos = 0;
        loop {
            println!("opcode: {}", self.memory[pos]);
            println!("memory: {}", self.serialize());
            match self.memory[pos] {
            1 => { // add
                let left = self.memory[pos + 1];
                let right = self.memory[pos + 2];
                let out = self.memory[pos + 3];
                self.memory[out] = self.memory[left] + self.memory[right];
                pos += 4;
            },
            2 => { // multiply
                let left = self.memory[pos + 1];
                let right = self.memory[pos + 2];
                let out = self.memory[pos + 3];
                self.memory[out] = self.memory[left] * self.memory[right];
                pos += 4;
            },
            99 => break, // exit
            _ => panic!("unsupported opcode: {}", self.memory[pos]),
            }
        }
        self
    }

    fn serialize(&self) -> String {
        let v : Vec<String> = self.memory.iter().map(|i| i.to_string()).collect();
        v.join(",")
    }
}

#[test]
fn test_program() {
    assert_eq!("1,0,2,2,99", IntcodeMachine::parse("1,0,0,2,99").run().serialize());

    assert_eq!("2,0,0,0,99", IntcodeMachine::parse("1,0,0,0,99").run().serialize());
    assert_eq!("2,3,0,6,99", IntcodeMachine::parse("2,3,0,3,99").run().serialize());
    assert_eq!("2,4,4,5,99,9801", IntcodeMachine::parse("2,4,4,5,99,0").run().serialize());
    assert_eq!("30,1,1,4,2,5,6,0,99", IntcodeMachine::parse("1,1,1,4,99,5,6,0,99").run().serialize());

    assert_eq!("3500,9,10,70,2,3,11,0,99,30,40,50", IntcodeMachine::parse("1,9,10,3,2,3,11,0,99,30,40,50").run().serialize());
    
}

fn main() {
    // low: 520625
    let i = include_str!("input.txt");
    let mut p = IntcodeMachine::parse(i);
    p.memory[1] = 12;
    p.memory[2] = 2;
    let p2 = p.run();
    println!("{}", p2.serialize());
}
