use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use regex::Regex;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    a: isize,
    b: isize,
    c: isize,
    ip: usize,
}

impl Computer {

    fn run_program ( mut self, program: &Vec<usize> ) -> Vec<isize> {
        let mut output: Vec<isize> = Vec::new();
        while self.ip < program.len() {
            let op = program[self.ip];
            let operand = *program.get(self.ip + 1).unwrap_or(&0);
            let mut increment: bool = true;

            match op {
                // adv A / (2^ operand)
                0 => self.a >>= self.operand_value(operand),
                // bxl B XOR operand
                1 => self.b ^= operand as isize,
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

    fn operand_value(self, operand: usize ) -> isize {
        let mut operand_value: isize = 0;
         match operand {
            0..=3 => operand_value = operand as isize,
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

    let mut to_find = vec![0];

    for &valid in program.iter().rev() {
        let mut next = Vec::new();

        for i in to_find {
            for j in 0..8 {
                let a = (i << 3) | j;
                let computer = Computer{ a: a, b: 0, c: 0, ip: 0 };
                if computer.run_program(&program)[0] == valid as isize {
                    next.push(a);
                }
            }
        }
        to_find = next;
    }
    to_find[0] as usize
}


fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample1");
    //let path = Path::new("sample2");
    //let path = Path::new("sample3");
    //let path = Path::new("sample4");
    //let path = Path::new("sample5");
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
            let value: isize = parts[1].parse::<isize>().unwrap_or(0);
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

    //println!("Computer: {:?}", cpu);
    //println!("Program: {:?}", program);
    let part1_answer = cpu.run_program(&program).iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    //println!("Computer {:?}", cpu);

    let part2_answer = find_quine(&program);

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

