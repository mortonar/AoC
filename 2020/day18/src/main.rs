use anyhow::{Error, Result, bail};
use std::io::{BufRead, stdin};
use std::mem;
use std::str::FromStr;

fn main() -> Result<()> {
    let raw_equations = parse_input()?;

    println!("Part 1: {}", eval(&raw_equations, false));
    println!("Part 2: {}", eval(&raw_equations, true));

    Ok(())
}

fn parse_input() -> Result<Vec<String>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l.map_err(Error::from))
        .collect()
}

fn eval(raw_equations: &[String], with_precedence: bool) -> usize {
    raw_equations
        .iter()
        .map(|re| RpnEquation::from_raw(re, with_precedence).unwrap())
        .map(|eq| eq.eval())
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
struct RpnEquation {
    tokens: Vec<Token>,
}

impl RpnEquation {
    fn from_raw(s: &str, with_precedence: bool) -> std::result::Result<Self, Error> {
        let raw_eq = s
            .trim()
            .split_ascii_whitespace()
            .flat_map(|s| {
                // Further "sub-token" combined tokens (e.g. "(3", "42)")
                let mut tokens = Vec::new();
                let mut num = String::new();
                for c in s.chars() {
                    match c {
                        '0'..='9' => num.push(c),
                        '(' | ')' => {
                            if !num.is_empty() {
                                tokens.push(mem::take(&mut num));
                            }
                            tokens.push(c.to_string());
                        }
                        '+' | '*' => {
                            if !num.is_empty() {
                                tokens.push(mem::take(&mut num));
                            }
                            tokens.push(c.to_string())
                        }
                        _ => panic!("Unrecognized character {c}"),
                    }
                }
                if !num.is_empty() {
                    tokens.push(mem::take(&mut num));
                }
                tokens.into_iter()
            })
            .map(|t| t.parse())
            .collect::<Result<Vec<Token>>>()?;
        RpnEquation::from_tokens(raw_eq, with_precedence)
    }

    fn from_tokens(tokens: Vec<Token>, with_precedence: bool) -> Result<Self> {
        let mut operators = Vec::new();
        let mut output = Vec::new();
        let get_prec = if with_precedence {
            |t: &Token| -> u8 {
                match t {
                    // Part 2 intentionally reverses the normal precedence (Aunt sally, forgive us)
                    Token::Add => 2,
                    Token::Mul => 1,
                    _ => 0,
                }
            }
        } else {
            |_t: &Token| -> u8 { 1 }
        };

        for token in tokens {
            match token {
                Token::Num(_) => output.push(token),
                Token::OpenParen => operators.push(Token::OpenParen),
                Token::CloseParen => {
                    let mut found_open = false;
                    while let Some(top) = operators.pop() {
                        if top == Token::OpenParen {
                            found_open = true;
                            break;
                        }
                        output.push(top);
                    }

                    if !found_open {
                        return Err(Error::msg("mismatched closing parenthesis"));
                    }
                }
                token if token.is_op() => {
                    let current_prec = get_prec(&token);
                    while let Some(last) = operators.last() {
                        if last.is_op() && get_prec(last) >= current_prec {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push(token);
                }
                _ => {}
            }
        }

        while let Some(top) = operators.pop() {
            if top == Token::OpenParen {
                bail!("mismatched opening parenthesis");
            }
            output.push(top);
        }

        Ok(RpnEquation { tokens: output })
    }

    fn eval(&self) -> usize {
        let mut stack = Vec::new();
        for token in &self.tokens {
            match token {
                Token::Num(n) => stack.push(*n),
                Token::Add => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op1 + op2);
                }
                Token::Mul => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op1 * op2);
                }
                _ => panic!("Unrecognized token {token:?}"),
            }
        }
        stack.pop().unwrap()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Num(usize),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Token::Add),
            "*" => Ok(Token::Mul),
            "(" => Ok(Token::OpenParen),
            ")" => Ok(Token::CloseParen),
            _ => Ok(Token::Num(s.parse()?)),
        }
    }
}

impl Token {
    fn is_op(&self) -> bool {
        matches!(self, Token::Add | Token::Mul)
    }
}
