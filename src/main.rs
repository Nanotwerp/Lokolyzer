use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use utf8_chars::BufReadCharsExt;

pub enum CharType {
    Letter(char),
    Digit(i32),
    Operator(char),
    Unknown(char),
}
pub enum TokenType {
    IntLit,
    KeyWord,
    Ident,
    Eq,
    NotEq,
    AddOp,
    SubOp,
    MultOp,
    DivOp,
    GreaterThan,
    LessThan,
    GorEQ,
    LorEQ,
    LeftParen,
    RightParen,
    Semicolon,
}

pub static mut CHAR_CLASS: Vec<u8> = Vec::new();
pub static mut LEXEME: Vec<char> = Vec::new();
pub static mut SYMBOLS: Vec<String> = Vec::new();
pub static mut LEX_LEN: usize = 0;

// This function invokes the other functions in the necessary order and reads the file as it is.
pub fn main() {
    /*let path = Path::new("mathy.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    } */
    let mut symbols: HashMap<&str, String> = HashMap::default();
    let mut token = String::new();

    addchar().err();
    tokenize();
    parser();
    let x = String::from("rnt");
    if is_type(&x) {
        println!("Yay!");
    }
}

fn is_variable(var: &str) -> bool {
    let regex = Regex::new("[a-zA-Z]+(_[a-zA-Z]+)*$").unwrap();
    return regex.is_match(&var) && !is_type(&var) && has_varcount(&var);
}

fn has_varcount(county: &str) -> bool {
    if county.chars().count() == 6 || county.chars().count() == 7 || county.chars().count() == 8 {
        return true;
    } else {
        return false;
    }
}

fn is_operator(op: &str) -> bool {
    if op.eq("+")
        || op.eq("-")
        || op.eq("*")
        || op.eq("/")
        || op.eq("%")
        || op.eq(">")
        || op.eq(">=")
        || op.eq("<")
        || op.eq("<=")
        || op.eq("=")
        || op.eq("!=")
    {
        return true;
    } else {
        return false;
    }
}

fn is_paren(par: &str) -> bool {
    return par.eq("(") || par.eq(")");
}

fn is_number(num: &str) -> bool {
    let regex = Regex::new("^[0-9]+$").unwrap();
    return regex.is_match(&num);
}

fn is_type(typeid: &str) -> bool {
    return typeid.eq("rnt") || typeid.eq("rort") || typeid.eq("rong");
}

// This function reads the values in the given file and stores it as a character vector.
fn addchar() -> io::Result<()> {
    unsafe {
        let f = File::open("mathy.txt")?;
        let mut f = BufReader::new(f);
        for c in f.chars().map(|x| x.unwrap()) {
            LEXEME.push(c);
        }
        Ok(())
    }
}

fn tokenize() {
    unsafe {
        let mut typevec: Vec<String> = Vec::new();
        let mut token = String::new();
        for i in 0..LEXEME.len() - 1 {
            if !LEXEME[i].is_whitespace() {
                token.push(LEXEME[i]);
                if is_type(&token) {
                    println!("Token type of {} is type id.", &token);
                    SYMBOLS.push(token);
                    token = String::new();
                }
                if is_operator(&token) {
                    if token.eq("+") {
                        println!("Token type of {} is add operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("-") {
                        println!("Token type of {} is minus operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("*") {
                        println!("Token type of {} is multiplication operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("/") {
                        println!("Token type of {} is division operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("%") {
                        println!("Token type of {} is modulus operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("=") && !LEXEME[i + 1].to_string().eq("==") {
                        println!("Token type of {} is equal operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("==") {
                        println!("Token type of {} is assignment operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("!=") {
                        println!("Token type of {} is not equal operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq(">") && !LEXEME[i + 1].to_string().eq("=") {
                        println!("Token type of {} is greater than operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("<") && !LEXEME[i + 1].to_string().eq("=") {
                        println!("Token type of {} is less than operator.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq(">=") {
                        println!(
                            "Token type of {} is greater than or equal to operator.",
                            &token
                        );
                        SYMBOLS.push(token);
                        token = String::new();
                    } else if token.eq("<=") {
                        println!(
                            "Token type of {} is less than or equal to operator.",
                            &token
                        );
                        SYMBOLS.push(token);
                        token = String::new();
                    }
                }
                if is_paren(&token) {
                    if token.eq("(") {
                        println!("Token type of {} is open parenthesis.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    }
                    if token.eq(")") {
                        println!("Token type of {} is closed parenthesis.", &token);
                        SYMBOLS.push(token);
                        token = String::new();
                    }
                }
                if is_variable(&token) && !LEXEME[i + 1].is_ascii_alphabetic() {
                    println!("Token type of {} is variable.", &token);
                    SYMBOLS.push(token);
                    token = String::new();
                }
                if is_number(&token) && !LEXEME[i + 1].is_ascii_digit() {
                    println!("Token type of {} is number.", &token);
                    SYMBOLS.push(token);
                    token = String::new();
                }
                if token.eq(";") {
                    println!("Token type of {} is semicolon.", &token);
                    SYMBOLS.push(token);
                    token = String::new();
                }
            }
        }
    }
}

fn parser() {
    unsafe {
        for i in 0..SYMBOLS.len() {
            if SYMBOLS[i].eq("rnt") && is_variable(&SYMBOLS[i + 1]) {
                if SYMBOLS[i + 2].eq("=") && is_paren(&SYMBOLS[i + 3]) {
                    if is_number(&SYMBOLS[i + 4]) && SYMBOLS[i + 5].eq("+") {
                        let a: i32 = SYMBOLS[i + 4].parse().unwrap();
                        let b: i32 = SYMBOLS[i + 6].parse().unwrap();
                        let c = a + b;
                        println!("{}",c);
                    }
                    if is_number(&SYMBOLS[i + 4]) && SYMBOLS[i + 5].eq("-") {
                        let a: i32 = SYMBOLS[i + 4].parse().unwrap();
                        let b: i32 = SYMBOLS[i + 6].parse().unwrap();
                        let c = a - b;
                        println!("{}",c);
                    }
                    if is_number(&SYMBOLS[i + 4]) && SYMBOLS[i + 5].eq("*") {
                        let a: i32 = SYMBOLS[i + 4].parse().unwrap();
                        let b: i32 = SYMBOLS[i + 6].parse().unwrap();
                        let c = a * b;
                        println!("{}",c);
                    }
                    if is_number(&SYMBOLS[i + 4]) && SYMBOLS[i + 5].eq("/") {
                        let a: i32 = SYMBOLS[i + 4].parse().unwrap();
                        let b: i32 = SYMBOLS[i + 6].parse().unwrap();
                        let c = a / b;
                        println!("{}",c);
                    }
                }
            }
        }
    }
}
