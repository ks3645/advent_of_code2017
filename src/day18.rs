use utils;
use utils::Part;
use std::collections::HashMap;

enum Program {
    Zero,
    One,
}

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 18).unwrap();

    let mut out = 0;

    let mut registers:(HashMap<&str, i64>, HashMap<&str,i64>) = (HashMap::new(), HashMap::new());
    let mut message_queues:(Vec<i64>,Vec<i64>) = (Vec::new(), Vec::new());
    let mut waiting = (false, false);
    let mut last_played = 0;
    let mut instructions = Vec::new();

    for line in input.lines() {
        instructions.push(line);
    }

    let mut instr_ptr:(i32, i32) = (0, 0);

    let mut active_program = Program::Zero;
    registers.0.insert("p", 0);
    registers.1.insert("p", 1);

    let mut sent_count = 0;

    'eval: loop {
        if waiting == (true, true)
            && (message_queues.0.is_empty(), message_queues.1.is_empty()) == (true, true) {
            break;
        }

        let tokens:Vec<&str> = match active_program {
            Program::Zero => instructions[instr_ptr.0 as usize].split_whitespace().collect(),
            Program::One => instructions[instr_ptr.1 as usize].split_whitespace().collect(),
        };

        let mut active_registers = match active_program {
            Program::Zero => &mut registers.0,
            Program::One => &mut registers.1,
        };

        let mut active_ptr = match active_program {
            Program::Zero => &mut instr_ptr.0,
            Program::One => &mut instr_ptr.1,
        };

        let reg = tokens[1];
        if !active_registers.contains_key(&reg) { active_registers.insert(reg, 0);}

        match tokens[0] {
            "snd" => {
                match part {
                    Part::PartOne => last_played = * active_registers.get(&tokens[1]).unwrap(),
                    Part::PartTwo => {
                        match active_program {
                            Program::Zero => {
                                message_queues.1.push(*active_registers.get(&tokens[1]).unwrap());
                            },
                            Program::One => {
                                sent_count += 1;
                                message_queues.0.push(*active_registers.get(&tokens[1]).unwrap());
                            },
                        }
                    },
                }
            },
            "set" => {
                let to_set =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *active_registers.get(&tokens[2]).unwrap(),
                };
                let val = active_registers.entry(tokens[1]).or_insert(0);
                *val = to_set;
            },
            "add" => {
                let to_add =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *active_registers.get(&tokens[2]).unwrap(),
                };
                let val = active_registers.entry(tokens[1]).or_insert(0);
                *val += to_add;
            },
            "mul" => {
                let to_mul =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *active_registers.get(&tokens[2]).unwrap(),
                };
                let val = active_registers.entry(tokens[1]).or_insert(0);
                *val *= to_mul;
            },
            "mod" => {
                let to_mod =  match tokens[2].trim().parse() {
                    Ok(i) => i,
                    Err(_) => *active_registers.get(&tokens[2]).unwrap(),
                };
                let val = active_registers.entry(tokens[1]).or_insert(0);
                *val %= to_mod;
            },
            "rcv" => {
                match part {
                    Part::PartOne => if *active_registers.get(&tokens[1]).unwrap() != 0 { break; },
                    Part::PartTwo => {
                        match active_program {
                            Program::Zero => {
                                if message_queues.0.is_empty() {
                                    waiting.0 = true;
                                    active_program = Program::One;
                                    continue 'eval;
                                }
                                else {
                                    let val = active_registers.entry(tokens[1]).or_insert(0);
                                    *val = message_queues.0.remove(0);
                                    waiting.0 = false;
                                }
                            },
                            Program::One => {
                                if message_queues.1.is_empty() {
                                    waiting.1 = true;
                                    active_program = Program::Zero;
                                    continue 'eval;
                                }
                                else {
                                    let val = active_registers.entry(tokens[1]).or_insert(0);
                                    *val = message_queues.1.remove(0);
                                    waiting.0 = false;
                                }
                            },
                        }
                    },
                }
            },
            "jgz" => {
                if *active_registers.get(&tokens[1]).unwrap() > 0 {
                    let to_jmp = match tokens[2].trim().parse::<i64>() {
                        Ok(i) => i,
                        Err(_) => *active_registers.get(&tokens[2]).unwrap(),
                    };
                    *active_ptr += to_jmp as i32;
                    continue 'eval;
                }
            },
            _ => panic!("Unrecognized Instruction"),
        }
        *active_ptr += 1;
        println!("(Zero: {}, One: {})", message_queues.0.len(), message_queues.1.len());
    }

    out = match part {
        Part::PartOne => last_played as i32,
        Part::PartTwo => sent_count,
    };

    out
}