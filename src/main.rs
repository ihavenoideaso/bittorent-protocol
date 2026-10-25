use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

enum Info {
    SingleFileMode(SingleFileMode),
    MultipleFileMode(MultipleFileMode)
}

struct SingleFileMode {
    name: String,
    length: usize,
    md5sum: Option<[u8; 16]>
}

struct SingleFileForMFM {
    path: Option<String>,
    length: usize,
    md5sum: Option<[u8; 64]>  
}

struct MultipleFileMode {
    name: String,
    files: Vec<SingleFileForMFM>
}

struct File {
    info: Info,
    announce: String,
    announce_list: Option<Vec<Vec<String>>>,
    creation_date: Option<usize>,
    comment: Option<String>,
    created_by: Option<String>,
    encoding: Option<String>,
    piece_length: usize,
    pieces: Vec<[u8; 20]>,
    private: Option<i32>,
}

fn main() {
    let data = "4:spam".to_string();
    let t= bencoding(data);
    println!("{:?}", t);
}

#[derive(Debug)]
enum BencodingReturn {
    Int(i64),
    Str(String),
    List(Vec<String>),
    Dictonaries(HashMap<String, Vec<String>>)
}

fn bencoding(data: String) -> Vec<BencodingReturn> {
    let mut result: Vec<BencodingReturn> = Vec::new();
    let mut iter = data.chars().peekable();
    while let Some(&char) = iter.peek() {
        match char {
            '0'..='9' => {
                let number = extract_number(&mut iter, ':');
                let text: String = iter.by_ref().take(number as usize).collect();
                result.push(BencodingReturn::Str(text));
                
            }
            'i' => {
                iter.next();
                let number = extract_number(&mut iter, 'e');
                result.push(BencodingReturn::Int(number));

            }  
            'l' => {
                iter.next();
                let list = insert_list(&mut iter);
                result.push(BencodingReturn::List(list));
            }
            'd' => {
                iter.next();
                let mut h: HashMap<String, Vec<String>> = HashMap::new();
                loop {
                    let number = extract_number(&mut iter, ':');
                    let key: String = iter.by_ref().take(number as usize).collect();
                    match iter.peek() {
                        Some(&'l') => {
                            iter.next();
                            let list = insert_list(&mut iter);
                            h.insert(key, list);
                        }
                        Some(&'i') => {
                            iter.next();
                            let number = extract_number(&mut iter, 'e');
                            let mut v: Vec<String> = Vec::new();
                            v.push(number.to_string());
                            h.insert(key, v);
                        }
                        _ => {
                            let number2 = extract_number(&mut iter, ':');
                            let value: String = iter.by_ref().take(number2 as usize).collect();
                            let mut v: Vec<String> = Vec::new();
                            v.push(value);
                            h.insert(key, v);
                        }
                    }
                    if 'e' == *iter.clone().peek().unwrap() {
                        break;
                    }
                }
                result.push(BencodingReturn::Dictonaries(h));
            }
            _ => {
                iter.next();
            }
        }
    }
    result
}

fn insert_list(iter: &mut Peekable<Chars>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    loop {
        let number = extract_number(iter, ':');
        let text = iter.by_ref().take(number as usize).collect();
        v.push(text);
        if 'e' == *iter.clone().peek().unwrap() {
            break;
        }
    }
    v
}

fn extract_number(iter: &mut Peekable<Chars>, char: char) -> i64 {
    let mut number: i64 = 0;
    if let Some(index) = iter.clone().position(|c| c == char) {
        let number_str: String = iter.take(index).collect();
        number = number_str.parse().unwrap_or(0);
    }
    iter.next();
    number
}