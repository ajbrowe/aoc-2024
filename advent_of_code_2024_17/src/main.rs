use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use regex::Regex;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
}

impl Computer {

    fn run_program ( mut self, program: &Vec<usize> ) -> Vec<usize> {
        let mut output: Vec<usize> = Vec::new();
        while self.ip < program.len() {
            let op = program[self.ip];
            let operand = *program.get(self.ip + 1).unwrap_or(&0);
            let mut increment: bool = true;

            match op {
                // adv A / (2^ operand)
                0 => self.a >>= self.operand_value(operand),
                // bxl B XOR operand
                1 => self.b ^= operand,
                // bst B = operand % 8
                2 => self.b = self.operand_value(operand) % 8,
                // jnz if A != 0 ip = operand
                3 => {
                    if self.a != 0 {
                        self.ip = operand;
                        // skip ip increment
                        increment = false;
                    }
                },
                4 => self.b ^= self.c ,
                5 => output.push(self.operand_value(operand) % 8),
                6 => self.b = self.a >> self.operand_value(operand),
                7 => self.c = self.a >> self.operand_value(operand),
                _ => (),
            }

            if increment {
                self.ip += 2;
            }
        }
        output

    }

    fn operand_value(self, operand: usize ) -> usize {
        let mut operand_value: usize = 0;
         match operand {
            0..=3 => operand_value = operand,
            4 => operand_value = self.a,
            5 => operand_value = self.b,
            6 => operand_value = self.c,
            7 => (),
            _ => (),
        }
        operand_value
    }


}

fn find_quine(program: &Vec<usize>) -> usize {

    // Process the program backwards, and attempt to solve the
    // value of A that will output each code in the program
    //
    // start with a = 0 since this is nececessary to finish the loop
    let mut to_find = vec![0];

    for &code in program.iter().rev() {
        let mut next = Vec::new();

        for i in to_find {
            for j in 0..8 {
                // solve the value 3-bits at a time
                let a = (i << 3) | j;

                // Create a new computer with A set to our test a value
                let computer = Computer{ a: a, b: 0, c: 0, ip: 0 };

                // Run the program and compare the first output with the value
                // we expect in the program. If it matches add it to the next
                // round of values to find.
                if computer.run_program(&program)[0] == code {
                    next.push(a);
                }
            }
        }
        to_find = next;
    }

    // The first value in to_find is the lowest match found
    to_find[0]
}


fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Process each line
    let mut reading_registers: bool = true;
    let mut cpu: Computer = Computer{ a: 0, b: 0, c: 0, ip: 0 };
    let mut program: Vec<usize> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            reading_registers = false;
            continue;
        }
        if reading_registers {
            let parts: Vec<&str> = line.split(": ").collect();
            let value: usize = parts[1].parse::<usize>().unwrap_or(0);
            match parts[0] {
                "Register A" =>  cpu.a = value,
                "Register B" =>  cpu.b = value,
                "Register C" =>  cpu.c = value,
                _ => (),
            }
        }
        else {
            if let Some(program_string) = line.strip_prefix("Program: ") {
                program = program_string.split(",")
                    .map(|s| s.parse::<usize>().expect("Invalid integer"))
                    .collect();
            }
        }
    }

    let part1_answer = cpu.run_program(&program).iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
    let part2_answer = find_quine(&program);

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

