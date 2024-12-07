use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn generate_operator_vec(num_operators: usize) -> Vec<Vec<char>> {
    let mut results = Vec::new();

    for i in 0..(1 << num_operators) {
        let mut operators = Vec::new();
        
        for j in 0..num_operators {
            if (i & (1 << j)) != 0 {
                operators.push('*');
            } else {
                operators.push('+');
            }
        }
        results.push(operators)
    }
    results
}

fn generate_operator_concatenator_vec(num_operators: usize) -> Vec<Vec<char>> {
    let operators = "+*|".chars().collect::<Vec<_>>();

    let results: Vec<Vec<char>> = std::iter::repeat(operators.iter().copied()) // convert to char from &char
        .take(num_operators)
        .multi_cartesian_product()
        .collect();

    //println!("Generated operators {:?}", results);
    results
}

fn find_equation_combinations(test_value: u64, equation_values: Vec<u64>, part2: bool) -> bool {
    if equation_values.is_empty() {
        return false;
    }

    // Number of operators is one less than the number of equation values
    let num_operators = equation_values.len() - 1;

    let combinations = if part2 { generate_operator_concatenator_vec(num_operators) } else { generate_operator_vec(num_operators) };
    for operator_combination in combinations {

        // Evaluate the equation for this combination of operators
        let mut result = equation_values[0] as i64;
        let mut equation = format!("{}", result);
        //println!("Operator Combinations: {:?}", operator_combination);

        for (&value, &operator) in equation_values.iter().skip(1).zip(operator_combination.iter()) {
            match operator {
                '+' => result += value as i64,
                '*' => result *= value as i64,
                '|' => result = concatenate(result as u64, value).unwrap_or(0) as i64,
                _ => unreachable!(),
            }
            equation.push_str(&format!(" {} {}", operator, value));
            if result > test_value as i64 {
                break;
            }
        }

        // Check if the result matches the test value
        if result == test_value as i64 {
            println!("Matching equation: {} = {}", equation, test_value);
            return true
        } else {
            //println!("Not Matching equation: {}", equation);
        }
            
    }

    false
}

fn concatenate(a: u64, b:u64) -> Option<u64> {
    let concatenated_value = format!("{}{}", a, b);
    concatenated_value.parse::<u64>().ok()
}


fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: u64 = 0;

    let mut part2_answer: u64 = 0;


    // Process each line
    for line in reader.lines() {
        let line = line?;
        if let Some((test_value_str, equation_string)) = line.split_once(": ") { 
            let test_value: u64 = test_value_str.parse().unwrap();
            let equation_values: Vec<u64> = equation_string.split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();
            let p2_equation_values: Vec<u64> = equation_values.clone();

            if find_equation_combinations(test_value, equation_values, false) {
                part1_answer += test_value
            }

            if find_equation_combinations(test_value, p2_equation_values, true) {
                part2_answer += test_value
            }

        }
    }
    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

