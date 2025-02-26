use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::io::BufRead;
// 3-bit computer:
// * 3 registers - Not limited to 3 bits. Each can hold any integer: A, B, C.
// * 8 instructions each with operand: [3-bits] [3-bits] -> opcode operand
// * Instruction Pointer(IP) - position in program to read next instruction from: starting at 0
//   * Sans JUMP, increases by 2 after each instruction (+ opcode + operand)
// * Halts if it tries to read opcode past end of program
//   * e.g. 0,1,2,3 -> perform opcode 0 on 1, perform opcode 2 on 3, halt
// * Operands: literal (e.g. literal 7 is 7) or combo
//     Combo operands 0 through 3 represent literal values 0 through 3.
//     Combo operand 4 represents the value of register A.
//     Combo operand 5 represents the value of register B.
//     Combo operand 6 represents the value of register C.
//     Combo operand 7 is reserved and will not appear in valid programs.
// * Operators / instructions:
//    * 0 -> adv: division
//      * Numerator is value in A register.
//      * Denominator is found by raising 2 to the power of the instruction's COMBO operand.
//      * The result of the division operation is truncated to an integer and then written to the A register.
//      * e.g. An operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.
//    * 1 -> bxl: the bitwise XOR of register B and the instruction's LITERAL operand
//      * Result stored in register B.
//    * 2 -> bst: calculates the value of its COMBO operand modulo 8 (thereby keeping only its lowest 3 bits),
//      * Result written to the B register.
//    * 3 -> jnz: no-op if A is zero.
//      * Otherwise, jumps by setting the IP to value of its LITERAL operand
//        * If this instruction jumps, the IP IS NOT INCREASED by 2 after this instruction.
//    * 4 -> bxc: calculates the bitwise XOR of register B and register C, then stores the result in register B.
//      * (For legacy reasons, this instruction reads an operand but ignores it.)
//    * 5 -> out: calculates the value of its COMBO operand modulo 8, then outputs that value.
//      * If a program outputs multiple values, they are separated by commas.
//    * 6 -> bdv: works exactly like the adv instruction except the result is stored in the B register.
//      * The numerator is still read from the A register.
//    * 7 -> cdv: works exactly like the adv instruction except that the result is stored in the C register.
//      * The numerator is still read from the A register.
fn main() -> Result<()> {
    let mut lines = std::io::stdin().lock().lines();

    let reg_regex = Regex::new(r"Register ([ABC]): ([0-9]+)")?;
    let mut registers = Registers::default();
    for _ in 0..3 {
        let line = lines.next().context("Failed to read line")??;
        let (_full, [reg, val]) = reg_regex
            .captures(line.as_ref())
            .context("Register regex doesn't match")?
            .extract();
        match reg {
            "A" => registers.reg_a = val.parse()?,
            "B" => registers.reg_b = val.parse()?,
            "C" => registers.reg_c = val.parse()?,
            _ => return Err(anyhow!("Unknown register {}", reg)),
        }
    }
    dbg!(&registers);

    let _blank = lines.next().context("Failed to read line")??;

    let prog_regex = Regex::new(r"Program: (.*)$")?;
    let mut program: Vec<u8> = vec![];
    let prog_line = lines.next().context("Failed to read program")??;
    let (_full, [prog]) = prog_regex
        .captures(prog_line.as_ref())
        .context("Program regex doesn't match")?
        .extract();
    for token in prog.split(",") {
        program.push(token.parse()?);
    }
    dbg!(&program);
    // Make immutable after parsing for safety
    let program = program;

    let mut outputs = Vec::new();
    loop {
        if registers.ip >= program.len() {
            break;
        }

        let instruction = Instruction::new(program[registers.ip], program[registers.ip + 1])?;
        if let Some(output) = instruction.exec(&mut registers) {
            outputs.push(output.to_string());
        }
    }
    println!("{}", outputs.join(","));

    Ok(())
}

#[derive(Debug, Default)]
struct Registers {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    ip: usize,
}

#[derive(Debug)]
enum Instruction {
    ADV(u8),
    BXL(u8),
    BST(u8),
    JNZ(u8),
    BXC(u8),
    OUT(u8),
    BDV(u8),
    CDV(u8),
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Result<Self> {
        if operand > 7 {
            return Err(anyhow::anyhow!("Invalid operand: {}", operand));
        }

        match opcode {
            0 => Ok(Instruction::ADV(operand)),
            1 => Ok(Instruction::BXL(operand)),
            2 => Ok(Instruction::BST(operand)),
            3 => Ok(Instruction::JNZ(operand)),
            4 => Ok(Instruction::BXC(operand)),
            5 => Ok(Instruction::OUT(operand)),
            6 => Ok(Instruction::BDV(operand)),
            7 => Ok(Instruction::CDV(operand)),
            _ => Err(anyhow::anyhow!("Invalid opcode: {}", opcode)),
        }
    }

    fn exec(&self, registers: &mut Registers) -> Option<u32> {
        let mut output = None;

        match self {
            Instruction::ADV(operand) => {
                let num = registers.reg_a;
                let denom = 2u32.pow(operand.combo(&registers));
                registers.reg_a = num / denom;
            }
            Instruction::BXL(operand) => {
                let literal = *operand as u32;
                registers.reg_b ^= literal;
            }
            Instruction::BST(operand) => {
                registers.reg_b = operand.combo(&registers) % 8;
            }
            Instruction::JNZ(operand) => {
                if registers.reg_a != 0 {
                    registers.ip = *operand as usize;
                    return output;
                }
            }
            Instruction::BXC(_operand) => {
                registers.reg_b ^= registers.reg_c;
            }
            Instruction::OUT(operand) => {
                output = Some(operand.combo(&registers) % 8);
            }
            Instruction::BDV(operand) => {
                let num = registers.reg_a;
                let denom = 2u32.pow(operand.combo(&registers));
                registers.reg_b = num / denom;
            }
            Instruction::CDV(operand) => {
                let num = registers.reg_a;
                let denom = 2u32.pow(operand.combo(&registers));
                registers.reg_c = num / denom;
            }
        }

        registers.ip += 2;

        output
    }
}

trait Combo {
    fn combo(&self, registers: &Registers) -> u32;
}

impl Combo for u8 {
    fn combo(&self, registers: &Registers) -> u32 {
        match self {
            0..=3 => *self as u32,
            4 => registers.reg_a,
            5 => registers.reg_b,
            6 => registers.reg_c,
            // TODO Handle 7 and beyond?
            _ => unreachable!(),
        }
    }
}
