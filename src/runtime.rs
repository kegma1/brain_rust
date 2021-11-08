use crate::parser::parse;
use crate::Token;
use read_input::prelude::*;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub prg: Vec<Token>,
    pub prg_pos: usize,
    pub mem: Vec<u8>,
    pub mem_pos: usize,
}

impl Runtime {
    pub fn new(parsed_prg: &str) -> Runtime {
        Runtime {
            prg: parse(parsed_prg),
            prg_pos: 0usize,
            mem: vec![0u8],
            mem_pos: 0usize,
        }
    }

    pub fn run(mut self) {
        loop {
            if self.prg_pos < self.prg.len() {
                // println!("Ja: {:?}\n{}\n", self, self.prg.len());
                match self.execute() {
                    Err(e) =>{println!("{}", e);break},
                    Ok(v) => {self = v.iterate().to_owned();},                
                };
            } else {
                break
            }
            

        }
        // println!("End: {:?}\n{}\n", self, self.prg.len());
    }

    pub fn execute(&mut self) -> Result<&mut Runtime, &'static str> {
        let current_token = &self.prg[self.prg_pos];


        match current_token {
            Token::Inc(x) => {
                self.mem[self.mem_pos] = ((self.mem[self.mem_pos] as usize + x) & 255) as u8;
                Ok(self)
            }
            Token::Dec(x) => {
                self.mem[self.mem_pos] =
                    ((self.mem[self.mem_pos] as isize - *x as isize) & 255) as u8;
                Ok(self)
            }
            Token::LMov(x) => {
                self.mem_pos += x;
                if self.mem_pos < self.mem.len() {
                    Ok(self)
                } else {
                    self.mem.append(&mut vec![0u8; *x]);
                    Ok(self)
                }
            }
            Token::RMov(x) => {
                let y = self.mem_pos.checked_sub(*x);
                match y {
                    None => Err("ERROR: Head moved off tape!\n"),
                    Some(v) => {
                        self.mem_pos = v;
                        Ok(self)
                    }
                }
            }
            Token::OutStd => {
                print!("{}", self.mem[self.mem_pos] as char);
                Ok(self)
            }
            Token::InStd => {
                self.mem[self.mem_pos] = input::<char>()
                    .add_err_test(|x| *x as usize <= 255, "Not a ascii value")
                    .get() as u8;
                Ok(self)
            }
            Token::OpenBrk(_) => {
                todo!()
            }
            Token::ClosedBrk(_) => {
                todo!()
            }
        }
    }

    fn iterate(&mut self) -> &mut Runtime {
        self.prg_pos += 1;
        self
    }
}
