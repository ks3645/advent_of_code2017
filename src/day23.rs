use utils;
use utils::Part;
use std::collections::HashMap;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 23).unwrap();

    let mut out = 0;

    let mut instructions = Vec::new();

    for line in input.lines() {
        instructions.push(line);
    }

    let mut instr_ptr:i32 = 0;
    let mut registers:HashMap<&str, i32> = HashMap::new();

    let mut mul_invoked = 0;

    'eval: loop {
        if instr_ptr as usize >= instructions.len() {break;}

        let tokens:Vec<&str> = instructions[instr_ptr as usize].split_whitespace().collect();

        match tokens[0] {
            "set" => {
                let to_set =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *registers.get(&tokens[2]).unwrap(),
                };
                let val = registers.entry(&tokens[1]).or_insert(0);
                *val = to_set;
            },
            "sub" => {
                let to_sub =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *registers.get(&tokens[2]).unwrap(),
                };
                let val = registers.entry(&tokens[1]).or_insert(0);
                *val -= to_sub;
            },
            "mul" => {
                let to_mul =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *registers.get(&tokens[2]).unwrap(),
                };
                let val = registers.entry(&tokens[1]).or_insert(0);
                *val *= to_mul;
                mul_invoked += 1;
            },
            "jnz" => {
                let to_check = match tokens[1].trim().parse() {
                    Ok(i) => (i),
                    Err(_) => *registers.entry(&tokens[1]).or_insert(0),
                };

                if to_check != 0 {
                    instr_ptr += match tokens[2].trim().parse() {
                        Ok(i) => i,
                        Err(_) => *registers.get(&tokens[2]).unwrap(),
                    };
                    continue 'eval;
                }
            },
            _ => panic!("Unrecognized Instruction"),
        }
        instr_ptr += 1;
    }

    out = mul_invoked;

    out
}