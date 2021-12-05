use crate::Token;
use regex::Regex;

pub fn compile(prg: &str) -> Vec<Token> {
    let re_remove_comments = Regex::new(r"[^<>\+\-\.,\[\]]").unwrap();

    let prg_no_comments = re_remove_comments.replace_all(prg, "").into_owned();

    let re_seperate_prg_str = Regex::new(r"<+|>+|\-+|\++|\[|\]|,|\.").unwrap();

    let prg_as_string_vector: Vec<_> = re_seperate_prg_str
        .find_iter(prg_no_comments.as_str())
        .collect::<Vec<_>>()
        .iter()
        .map(|x| String::from(x.as_str()))
        .collect();

    let prg_as_char_vector: Vec<char> = prg_as_string_vector
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>()[0])
        .collect();

    prg_as_string_vector
        .iter()
        .enumerate()
        .map(|x| {
            let x_chars: Vec<char> = x.1.chars().collect();
            match x_chars[0] {
                '+' | '-' | '<' | '>' => Some(parse_char(0, &x_chars).unwrap()),
                '[' | ']' | '.' | ',' => Some(parse_char(x.0, &prg_as_char_vector).unwrap()),
                _ => None,
            }
            .unwrap()
        })
        .collect()
}

pub fn parse_char(index: usize, prg: &Vec<char>) -> Option<Token> {
    let char = prg[index];
    match char {
        x if x == '+' => Some(Token::Inc(count_chars(x, &prg[index..]))),
        x if x == '-' => Some(Token::Dec(count_chars(x, &prg[index..]))),
        x if x == '>' => Some(Token::LMov(count_chars(x, &prg[index..]))),
        x if x == '<' => Some(Token::RMov(count_chars(x, &prg[index..]))),
        '.' => Some(Token::OutStd),
        ',' => Some(Token::InStd),
        x if x == '[' => Some(Token::OpenBrk(
            find_matching_brk(x, &prg[index + 1..]).unwrap(),
        )),
        x if x == ']' => Some(Token::ClosedBrk(
            find_matching_brk(x, &prg[..index]).unwrap(),
        )),
        _ => None,
    }
}

fn count_chars(count: char, list: &[char]) -> usize {
    let mut counter = 0usize;
    for x in list {
        if x == &count {
            counter += 1;
        } else {
            break;
        }
    }
    counter
}

fn find_matching_brk(brk: char, list: &[char]) -> Option<usize> {
    let list_rev: Vec<char> = list.iter().copied().rev().collect();

    match brk {
        x if x == '[' => Some(matching_brk(x, list, ']')),
        x if x == ']' => Some(matching_brk(x, &list_rev, '[')),
        _ => None,
    }
}

fn matching_brk(brk: char, list: &[char], opposite_brk: char) -> usize {
    let mut counter = 1usize;
    let mut brk_counter = 1usize;
    for x in list {
        match x {
            x if *x == brk => {
                brk_counter += 1;
            }
            x if *x == opposite_brk => {
                brk_counter -= 1;
            }
            _ => (),
        }

        if brk_counter == 0usize {
            break;
        }
        counter += 1;
    }
    counter
}
