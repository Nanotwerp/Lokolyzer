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

struct Char {
    lokochar: char,
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
    let x = String::from("rnt");
    if is_type(&x) {
        println!("Yay!");
    }
}

fn is_variable(var: &str) -> bool {
    let regex = Regex::new("[a-zA-Z]+(_[a-zA-Z]+)*").unwrap();
    return regex.is_match(&var);
}

fn is_operator(op: &str) -> bool {
    let regex = Regex::new("^\\+|-|\\*|^/$|^%$|^>(=)?$|^<(=)?&|(!|=)?=$").unwrap();
    return regex.is_match(&op);
}

fn is_paren(par: &str) -> bool {
    let regex = Regex::new("^(|)$").unwrap();
    return regex.is_match(&par);
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
        let mut symbols: HashMap<&str, String> = HashMap::default();
        let mut token = String::new();
        for i in 0..LEXEME.len() - 1 {
            if !LEXEME[i].is_whitespace()
                && (is_operator(&LEXEME[i + 1].to_string())
                || is_paren(&LEXEME[i + 1].to_string())
                || LEXEME[i + 1].is_whitespace())
            {
                token.push(LEXEME[i]);
            }
            if LEXEME[i + 1].is_whitespace()
                && is_variable(&token)
                && !is_type(&token)
                && is_operator(&LEXEME[i + 1].to_string())
                && is_paren(&LEXEME[i + 1].to_string())
            {
                symbols.insert("lokovar", token);
                token = String::new();
            } else if LEXEME[i + 1].is_whitespace()
                && is_type(&token)
                && is_operator(&LEXEME[i + 1].to_string())
                && is_paren(&LEXEME[i + 1].to_string())
            {
                symbols.insert("typeid", token);
                token = String::new();
            } else if LEXEME[i + 1].is_whitespace()
                && is_number(&token)
                && is_operator(&LEXEME[i + 1].to_string())
                && is_paren(&LEXEME[i + 1].to_string())
            {
                symbols.insert("enby", token);
                token = String::new();
            } else if !is_number(&token)
                && !is_operator(&LEXEME[i + 1].to_string())
                && !is_paren(&LEXEME[i + 1].to_string())
            {
                symbols.insert("unknown", token);
                token = String::new();
                println!("Token {i} ({token}) is invalid.");
            } else {
                continue;
            }
        }
        let clum = "+";
        if is_operator(&clum) {
            println!("Oohwee");
        }
    }
}

fn lexer() {
    unsafe {}
}
