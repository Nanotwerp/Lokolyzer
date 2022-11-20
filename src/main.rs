use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

pub enum CharType {
    Letter,
    Digit,
    Operator,
    Unknown,
}
pub enum TokenType {
    IntLit,
    Ident,
    AssignOp,
    AddOp,
    SubOp,
    MultOp,
    DivOp,
    LeftParen,
    RightParen,
}

pub static mut CHAR_CLASS: Vec<u8> = Vec::new();
pub static mut LEXEME: Vec<char> = Vec::new();
pub static mut NEXT_CHAR: char = '0';
pub static mut LEX_LEN: usize = 0;
pub static mut TOKEN: Vec<u8> = Vec::new();
pub static mut NEXT_TOKEN: Vec<u8> = Vec::new();

pub static mut LEX_TYPE: Vec<char> = Vec::new();

// This function invokes the other functions in the necessary order and reads the file as it is.
pub fn main() {
    let path = Path::new("mathy.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    addchar().err();
    lookup();
    lex();
}

// This function looks up the values in the array and assigns an appropriate  value at its index-linked position in another array
fn lookup() {
    unsafe {
        let mut chartype: char = LEXEME[0];
        for i in 0..LEXEME.len() - 1 {
            match chartype {
                '+' => {
                    LEX_TYPE.push('+');
                }
                '-' => {
                    LEX_TYPE.push('-');
                }
                '*' => {
                    LEX_TYPE.push('*');
                }
                '/' => {
                    LEX_TYPE.push('/');
                }
                '%' => {
                    LEX_TYPE.push('%');
                }
                '=' => {
                    LEX_TYPE.push('=');
                }
                '(' => {
                    LEX_TYPE.push('(');
                }
                ')' => {
                    LEX_TYPE.push(')');
                }
                '0' => {
                    LEX_TYPE.push('0');
                }
                '1' => {
                    LEX_TYPE.push('0');
                }
                '2' => {
                    LEX_TYPE.push('0');
                }
                '3' => {
                    LEX_TYPE.push('0');
                }
                '4' => {
                    LEX_TYPE.push('0');
                }
                '5' => {
                    LEX_TYPE.push('0');
                }
                '6' => {
                    LEX_TYPE.push('0');
                }
                '7' => {
                    LEX_TYPE.push('0');
                }
                '8' => {
                    LEX_TYPE.push('0');
                }
                '9' => {
                    LEX_TYPE.push('0');
                }
                'a' => {
                    LEX_TYPE.push('a');
                }
                'b' => {
                    LEX_TYPE.push('a');
                }
                'c' => {
                    LEX_TYPE.push('a');
                }
                'd' => {
                    LEX_TYPE.push('a');
                }
                'e' => {
                    LEX_TYPE.push('a');
                }
                'f' => {
                    LEX_TYPE.push('a');
                }
                'g' => {
                    LEX_TYPE.push('a');
                }
                'h' => {
                    LEX_TYPE.push('a');
                }
                'i' => {
                    LEX_TYPE.push('a');
                }
                'j' => {
                    LEX_TYPE.push('a');
                }
                'k' => {
                    LEX_TYPE.push('a');
                }
                'l' => {
                    LEX_TYPE.push('a');
                }
                'm' => {
                    LEX_TYPE.push('a');
                }
                'n' => {
                    LEX_TYPE.push('a');
                }
                'o' => {
                    LEX_TYPE.push('a');
                }
                'p' => {
                    LEX_TYPE.push('a');
                }
                'q' => {
                    LEX_TYPE.push('a');
                }
                'r' => {
                    LEX_TYPE.push('a');
                }
                's' => {
                    LEX_TYPE.push('a');
                }
                't' => {
                    LEX_TYPE.push('a');
                }
                'u' => {
                    LEX_TYPE.push('a');
                }
                'v' => {
                    LEX_TYPE.push('a');
                }
                'w' => {
                    LEX_TYPE.push('a');
                }
                'x' => {
                    LEX_TYPE.push('a');
                }
                'y' => {
                    LEX_TYPE.push('a');
                }
                'z' => {
                    LEX_TYPE.push('a');
                }
                ' ' => {
                    LEX_TYPE.push(' ');
                }
                '\n' => {
                    LEX_TYPE.push('N');
                    print!("Line!");
                }
                '_' => {
                    LEX_TYPE.push('_');
                }
                ';' => {
                    LEX_TYPE.push(';');
                }
                _ => {
                    LEX_TYPE.push('?');
                    print!(
                        "Error! This code has a character that cannot be processed by Lokolyzer."
                    )
                }
            }
            chartype = LEXEME[i + 1];
        }
    }
}

// This function reads the values in the given file and stores it as a character vector.
fn addchar() -> io::Result<()> {
    unsafe {
        let f = File::open("mathy.txt")?;
        let f = BufReader::new(f);

        for line in f.lines() {
            for c in line?.chars() {
                LEXEME.push(c);
            }
        }

        Ok(())
    }
}

// This function does lexical analysis.
fn lex() {
    unsafe {
        let mut charclass: char = LEX_TYPE[0];
        let mut lettercount = 0;
        for i in 0..LEX_TYPE.len() - 1 {
            match charclass {
                'a' => {
                    print!("{}", LEXEME[i]);
                    lettercount = lettercount + 1;
                }
                'N' => {
                    print!("{}", LEXEME[i]);
                    println!();
                }
                _ => print!("H"),
            }
            charclass = LEX_TYPE[i + 1];
        }
    }
}
