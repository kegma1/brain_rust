pub mod compiler;
pub mod parser;
pub mod runtime;

#[derive(Debug, Clone)]
pub enum Token {
    Inc(usize),
    Dec(usize),
    LMov(usize),
    RMov(usize),
    OutStd,
    InStd,
    OpenBrk(usize),
    ClosedBrk(usize),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Inc(x) => write!(f, "+ {}\n", x),
            Token::Dec(x) => write!(f, "- {}\n", x),
            Token::LMov(x) => write!(f, "< {}\n", x),
            Token::RMov(x) => write!(f, "> {}\n", x),
            Token::OutStd => write!(f, ".\n"),
            Token::InStd => write!(f, ",\n"),
            Token::OpenBrk(x) => write!(f, "[ {}\n", x),
            Token::ClosedBrk(x) => write!(f, "] {}\n", x),
        }
    }
}
