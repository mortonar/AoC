use anyhow::Result;
use std::process::exit;

// Solving this for the general case is...beyond my reach. But solving for my personalized input
// is absolutely possible!
//
// Here's my program/input: 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
//
// If we Hand-disassemble it, we get:
// -------------------------------------------------------------------------------------------------
// do {
//   b = a % 8                 // 2,4 -> 000 - 111 -> This tells us what the lowest 3 bits of COULD BE.
//                             // Solve what they could be for this iteration, add them to a (a <<= 3), then try to solve for the next iteration.
//   b ^= 7                    // 1,7 -> bbb ^ 111
//   c = a / (2^b)             // 7,5 -> aaa....aaa >> (2^b)
//   a /= 8                    // 0,3 (a /= 8 = a >> 3) -> aaa...aaa >> 3
//   b ^= c                    // 4,4 ->
//   b ^= 7                    // 1,7
//   out(b % 8)                // 5,5
// } while (while a % 8 != 0); // 3,0
// -------------------------------------------------------------------------------------------------
//
// * Every iteration of the loop is:
//   * Taking AND CONSUMING the lowest 3 bits of A (B = A % 8, ..., A /= 8)
//   * Then doing a gnarly computation based on those 3 bits
//   * And outputting an expression based on those 3 bits (B ^= 7, C = A / (2^B), ... out(B % 8))
//
// To solve:
// * Start with A = 0: the last run of the loop is a 3,0 check/jnz instruction
// * Work backwards through each program/output value
// * For that value, figure out a value of B = A % 8 (0-7) that works for the output value
// * Shift A left by 3 bits, tack on B -> that's the new A (or previous rather :)).
// * Solve for the next (originally previous) output value.
// * Backtrack if we eventually find no value of B works.
fn main() -> Result<()> {
    let mut program = vec![2, 4, 1, 7, 7, 5, 0, 3, 4, 4, 1, 7, 5, 5, 3, 0];
    program.reverse();

    next(0, 0, &program);

    Ok(())
}

// A is BIG. We MUST use a 64 bit value to track it.
fn next(a: u64, ip: usize, program: &Vec<u64>) {
    if ip == program.len() {
        println!("{}", a);
        exit(0);
    }

    let a = a << 3;
    for a_lower_3 in 0..=7 {
        // b = a % 8
        let mut b = a_lower_3;
        let a_prev = a | a_lower_3;

        b ^= 7;
        // B is safe as a pow() param since B is only the lower 3 bits of A
        let c = a_prev / 2u64.pow(b as u32);
        b ^= c;
        b ^= 7;
        if b % 8 == program[ip] {
            next(a_prev, ip + 1, program);
        }
    }
}
