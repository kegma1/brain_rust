use crate::Token;

pub fn parse(prg: &str) -> Vec<Token> {
    let mut parsed_prg: Vec<Token> = vec![];
    let tokenized_prg: Vec<_> = prg
        .split("\n")
        .map(|x| {
            let split_lexeme: Vec<_> = x.split(" ").collect();
            let token = split_lexeme[0];
            let option = split_lexeme.get(1);
            match option {
                Some(x) => (token, *x),
                None => (token, "0"),
            }
        })
        .collect();
    for x in tokenized_prg {
        match (x.0, x.1) {
            ("+", v) => parsed_prg.push(Token::Inc(v.parse::<usize>().unwrap())),
            ("-", v) => parsed_prg.push(Token::Dec(v.parse::<usize>().unwrap())),
            ("<", v) => parsed_prg.push(Token::LMov(v.parse::<usize>().unwrap())),
            (">", v) => parsed_prg.push(Token::RMov(v.parse::<usize>().unwrap())),
            ("[", v) => parsed_prg.push(Token::OpenBrk(v.parse::<usize>().unwrap())),
            ("]", v) => parsed_prg.push(Token::ClosedBrk(v.parse::<usize>().unwrap())),
            (".", _) => parsed_prg.push(Token::OutStd),
            (",", _) => parsed_prg.push(Token::InStd),
            (_, _) => (),
        }
    }
    parsed_prg
}
