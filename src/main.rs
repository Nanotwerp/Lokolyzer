use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use utf8_chars::BufReadCharsExt;

pub enum CharType {
    Letter(char),
    Digit(i32),
    Operator(char),
    Unknown(char),
}
pub enum TokenType {
    IntLit(i32),
    KeyWord(String),
    Ident(String),
    AssignOp(char),
    AddOp(char),
    SubOp(char),
    MultOp(char),
    DivOp(char),
    LeftParen(char),
    RightParen(char),
}

#[derive(Hash, Eq, PartialEq, Debug, Default)]
struct Symbol {
    lokotype: String,
    value: String,
}

trait Token {
    fn tokenize(&self) -> String;
}

trait Variable {
    fn variablize(&self) -> String;
}

struct Lexer {}

/*pub struct SymbolTable<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
} */

impl Symbol {
    /*fn insert(&mut self, lokotype: &str, value: char) -> Option<char> {
        self.base.insert()
    } */

    fn new(lokotype: &str, value: &str) -> Symbol {
        Default::default()
    }
}

pub static mut CHAR_CLASS: Vec<u8> = Vec::new();
pub static mut LEXEME: Vec<char> = Vec::new();
pub static mut NEXT_CHAR: char = '0';
pub static mut LEX_LEN: usize = 0;
pub static mut TOKEN: Vec<u8> = Vec::new();
pub static mut NEXT_TOKEN: Vec<u8> = Vec::new();
pub static mut LEX_TYPE: Vec<char> = Vec::new();
pub static mut SYMBOLTABLE: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut s = HashMap::new();
    s
});

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
    lookup();
    unsafe {
        for i in 0..LEXEME.len() - 1 {
            lex(i);
            let lex = lex(i);
            if !LEXEME[i].is_whitespace() {
                token.push(lex);
            } else {
                continue;
            }
            if LEXEME[i + 1].is_whitespace() {
                for i in 0..token.len() {
                    if token.chars().all(|x| x.is_alphabetic()) {
                        let clone = token.clone();
                        symbols.insert("rar", clone);
                        token = String::new();
                    }
                }
            }
        }
    }
}

// This function looks up the values in the array and assigns an appropriate value at its index-linked position in another array
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
        let mut f = BufReader::new(f);
        for c in f.chars().map(|x| x.unwrap()) {
            LEXEME.push(c);
        }
        Ok(())
    }
}

// This function does lexical analysis.
fn lex(mut i: usize) -> char {
    unsafe {
        let mut token = String::new();
        let mut charclass: char = LEX_TYPE[i];
        match charclass {
            'a' => {
                token.push(LEXEME[i]);
            }
            'N' => {
                token.push(LEXEME[i]);
            }
            '0' => {
                token.push(LEXEME[i]);
            }
            _ => print!("H"),
        }
        return LEXEME[i];
    }
}

fn lexer() {
    unsafe {}
}
