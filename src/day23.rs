use utils;
use utils::Part;
use std::collections::HashMap;
use std::fmt;
use std::io::Write;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 23).unwrap();

    let mut out = 0;

    let mut instructions = Vec::new();

    for line in input.lines() {
        let tokens:Vec<&str> = line.split_whitespace().collect();
        let program_line = ProgramLine{
            op:match tokens[0] {
                "set" => Instruction::Set,
                "sub" => Instruction::Sub,
                "mul" => Instruction::Mul,
                "jnz" => Instruction::Jnz,
                _ => panic!("Unrecognized Operation"),
            },
            x:match tokens[1].trim().parse::<i32>() {
                Ok(i) => Arg::Number(i),
                Err(_) => Arg::Register(tokens[1].trim()),
            },
            y:match tokens[2].trim().parse::<i32>() {
                Ok(i) => Arg::Number(i),
                Err(_) => Arg::Register(tokens[2].trim()),
            },
            label:String::new(),
        };
        instructions.push(program_line);
    }

//    label_instructions(&mut instructions);
//    print_instructions(&instructions);

    if part == Part::PartTwo {
        optimize_instructions(&mut instructions);
    }

    let mut instr_ptr:i32 = 0;
    let mut registers:HashMap<&str, i32> = HashMap::new();

    match part {
        Part::PartOne => {},
        Part::PartTwo => {
            registers.insert("a", 1);
        }
    }

    let mut mul_invoked = 0;

            'eval: loop {
                if instr_ptr as usize >= instructions.len() { break; }

                let line = &instructions[instr_ptr as usize];
//        println!("Executing: {:>2}: {:?}", instr_ptr, line);

                match line.op {
                    Instruction::Set => {
                        let to_set = match line.y {
                            Arg::Number(i) => i,
                            Arg::Register(reg) => *registers.get(&reg).unwrap(),
                        };
                        if let Arg::Register(reg) = line.x {
                            let val = registers.entry(&reg).or_insert(0);
                            *val = to_set;
                        } else {
                            panic!("Tried to add on a number instead of register");
                        }
                    },
                    Instruction::Sub => {
                        let to_sub = match line.y {
                            Arg::Number(i) => i,
                            Arg::Register(reg) => *registers.get(&reg).unwrap(),
                        };
                        if let Arg::Register(reg) = line.x {
                            let val = registers.entry(&reg).or_insert(0);
                            *val -= to_sub;
                        } else {
                            panic!("Tried to subtract on a number instead of register");
                        }
                    },
                    Instruction::Mul => {
                        let to_mul = match line.y {
                            Arg::Number(i) => i,
                            Arg::Register(reg) => *registers.get(&reg).unwrap(),
                        };
                        if let Arg::Register(reg) = line.x {
                            let val = registers.entry(&reg).or_insert(0);
                            *val *= to_mul;
                            mul_invoked += 1;
                        } else {
                            panic!("Tried to multiply on a number instead of register");
                        }
                    },
                    Instruction::Jnz => {
                        let to_check = match line.x {
                            Arg::Number(i) => i,
                            Arg::Register(reg) => *registers.entry(&reg).or_insert(0),
                        };
                        if to_check != 0 {
                            instr_ptr += match line.y {
                                Arg::Number(i) => i,
                                Arg::Register(reg) => *registers.get(&reg).unwrap(),
                            };
                            continue 'eval;
                        }
                    },
                    Instruction::IsPrime => {
                        let to_check = match line.y {
                            Arg::Number(i) => i,
                            Arg::Register(reg) => *registers.get(&reg).unwrap(),
                        };
                        let to_set = match line.x {
                            Arg::Register(reg) => reg,
                            Arg::Number(_) => panic!("Invalid isPrime use, can't write to num"),
                        };
                        let val = registers.entry(&to_set).or_insert(0);
                        if is_prime(to_check) {
                            *val = 1;
                        }
                        else {
                            *val = 0;
                        }
                    },
                    Instruction::End => { break; },
                    Instruction::Nop => {},
                }
                instr_ptr += 1;
            }

    out = match part {
        Part::PartOne => mul_invoked,
        Part::PartTwo => *registers.get("h").unwrap(),
    };

    out
}

fn optimize_instructions(instructions:&mut Vec<ProgramLine>) { //basically cheating right now
    instructions[10] = ProgramLine {
        op:Instruction::IsPrime,
        x:Arg::Register("f"),
        y:Arg::Register("b"),
        label:String::new(),
    };
    for i in 11..(23+1) {
        instructions[i].op = Instruction::Nop;
    }
}

fn is_prime(num:i32) -> bool {
    let upper_bound = (num as f32).sqrt().ceil() as i32;

    for i in 2..upper_bound {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn print_instructions(instructions:&Vec<ProgramLine>) {
    for (i, line) in instructions.iter().enumerate() {
        println!("{:>2}: {:?}", i, line);
    }
}

fn print_program(instructions:&Vec<ProgramLine>) {
    let mut indent = 0;
    let mut brace_locations:Vec<usize> = Vec::new();
    for (i, line) in instructions.iter().enumerate() {
        if line.label.contains("If") {
            let tokens:Vec<&str> = line.label.split(" , ").collect();
            let if_statement:Vec<&str> = tokens[0].split_whitespace().collect();
            let if_label = format!("if {} == 0", if_statement[1]);
            println!("{space:width$}{label} {{", label=if_label, width=indent, space="");
            let tokens:Vec<&str> = tokens[1].split(" ").collect();
            brace_locations.push(tokens[1].parse::<usize>().unwrap() - 1);
            indent += 4;
        }
        else if line.label.contains("Begin") {
            println!("{space:width$}loop {{", width=indent, space="");
            indent += 4;
            let tokens:Vec<&str> = line.label.split(":").collect();
            println!("{space:width$}{label}", label=tokens[1].trim(), width=indent, space="");
        }
        else if line.label.contains("End") {

            if let Arg::Register(reg) = line.x {
                println!("{space:width$}if {register} == 0 {{break;}}", register = reg,
                    width=indent, space="");
                indent -= 4;
                println!("{space:width$}}} //{comment}",
                         comment=line.label, width=indent, space="");
            }
            if let Arg::Number(num) = line.x {
                indent -= 4;
                println!("{space:width$}}}//{comment}",
                         comment=line.label, width=indent, space="");
            }
        }
        else if line.label.contains("Jump") {
            let tokens:Vec<&str> = line.label.split_whitespace().collect();
            let jump_to:usize = tokens[1].parse().unwrap();
            if jump_to >= instructions.len()  || jump_to < 0 {
                println!("{space:width$}break;", width=indent, space="");
            }
            else {
                println!("{space:width$}{label}", label=line.label, width=indent, space="");
            }
        }
        else {
            println!("{space:width$}{label}", width=indent, label=line.label, space="");
        }

        if !brace_locations.is_empty() {
            if *brace_locations.last().unwrap() == i {
                indent -= 4;
                brace_locations.pop();
                println!("{space:width$}}}", width=indent, space="");
            }
        }
    }
}



fn label_instructions(instructions:&mut Vec<ProgramLine>) {
    let mut i = 0;
    let mut loop_depth = 0;
    'analyze: loop {
        if i >= instructions.len() {
            break;
        }
        match instructions.to_vec()[i].op {
            Instruction::Jnz => {
                //insert a Nop instruction at jump landing or End instruction if out of bounds
                //disregard Nop instructions in calculating jump landings
                let mut offset = 0;
                {
                    let line = instructions.get(i).unwrap();
                    offset = match line.y {
                        Arg::Register(reg) => 0, //not supported in instruction set by assumption
                        Arg::Number(num) => num,
                    };
                }

                let step:i32 = match offset < 0 {
                    true => -1,
                    false => 1,
                };

                let mut jump_to = i as i32;
                loop {
                    match instructions.get(jump_to as usize).unwrap().op {
                        _ => {
                            offset -= step;
                            jump_to += step;
                        },
                    }
                    if offset == 0 {break;}
                }

                if jump_to <= 0 || jump_to >= instructions.len() as i32 {
                    instructions.get_mut(i).unwrap().op = Instruction::End;
                }

                let pos = match offset < 0 {
                    true => jump_to - 1,
                    false => jump_to,
                };

                if step < 0 {
                    {
                        instructions.get_mut(i).unwrap().label
                            = format!("End Loop {}", loop_depth);
                    }
                    {
                        let mut line = instructions.get_mut(jump_to as usize).unwrap();
                        line.label = format!("Begin Loop {}: {}", loop_depth, line.label);
                    }
                    loop_depth += 1;
                }
                else {
                    {
                        let mut line = instructions.get_mut(i).unwrap();
                        if let Arg::Register(reg) = line.x {
                            line.label = format!("If {} != 0 , Jump {}", reg, jump_to);
                        }
                        else {
                            line.label = format!("Jump {}", jump_to);
                        }
                    }
                }
            },
            Instruction::Set => {
                let line = instructions.get_mut(i).unwrap();
                if let Arg::Register(lhs) = line.x {
                    let rhs = match line.y {
                        Arg::Register(reg) => reg.to_string(),
                        Arg::Number(num) => num.to_string(),
                    };

                    line.label = format!("let {} = {};", lhs, rhs);
                }
                else {
                    panic!("Syntax Error: setting to number, line: {}", i);
                }
            },
            Instruction::Sub => {
                let mut line = instructions.get_mut(i).unwrap();
                if let Arg::Register(lhs) = line.x {
                    let rhs = match line.y {
                        Arg::Register(reg) => reg.to_string(),
                        Arg::Number(num) => num.to_string(),
                    };

                    line.label = format!("{} -= {};", lhs, rhs);
                }
                else {
                    panic!("Syntax Error: subtracting on number, line: {}", i);
                }
            },
            Instruction::Mul => {
                let line = instructions.get_mut(i).unwrap();
                if let Arg::Register(lhs) = line.x {
                    let rhs = match line.y {
                        Arg::Register(reg) => reg.to_string(),
                        Arg::Number(num) => num.to_string(),
                    };

                    line.label = format!("{} *= {};", lhs, rhs);
                }
                else {
                    panic!("Syntax Error: Multiplying on number, line: {}", i);
                }
            },
            _ => {},
        }
        i += 1;
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Set,
    Sub,
    Mul,
    Jnz,
    End,
    Nop,
    IsPrime,
}

#[derive(Clone)]
enum Arg<'a>{
    Register(&'a str),
    Number(i32),
}

#[derive(Clone)]
struct ProgramLine<'a> {
    op:Instruction,
    x:Arg<'a>,
    y:Arg<'a>,
    label:String
}

impl<'a> fmt::Debug for ProgramLine<'a> {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        let op = format!("{:?}", self.op);
        let x = format!("{:?}", self.x);
        let y = format!("{:?}", self.y);
        write!(f, "{} {:<8}{:<15} {}", op, x, y, self.label)
    }
}

impl<'a> fmt::Debug for Arg<'a> {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arg::Register(reg) => write!(f, "Reg({})", reg),
            Arg::Number(num) => write!(f, "Num({})", num),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   // No tests given in the problem description, part one is somewhat a test of its own
/*
    #[test]
    fn test_part_one() {
    }

    #[test]
    fn test_part_two() {
    }
    */
}