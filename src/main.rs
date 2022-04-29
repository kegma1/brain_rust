use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
//main
fn main() {
    let args: Vec<String> = env::args().collect();
    let sub_command: String = match args.get(1) {
        Some(v) => v.to_owned(),
        None => String::from(""),
    };
    match sub_command.as_str() {
        "run" | "r" => {
            let path = &args[2];
            let file_extension = {
                let dot_index = path.find(".").unwrap();
                &path[dot_index..]
            };
            match file_extension {
                ".bf" => {
                    let program_file = fs::read_to_string(path);
                    match program_file {
                        Ok(prog) => {
                            let prg = brain_rust::compiler::compile(&prog);
                            brain_rust::runtime::Runtime::new(prg).run()
                        }
                        Err(_) => println!("ERROR: Invalid path: {}", path),
                    }
                }
                ".rbf" => {
                    let program_file = fs::read_to_string(path);
                    match program_file {
                        Ok(prog) => {
                            let prg = brain_rust::parser::parse(&prog);
                            brain_rust::runtime::Runtime::new(prg).run()
                        }
                        Err(_) => println!("ERROR: Invalid path: {}", path),
                    }
                }
                _ => println!("ERROR: Invalid file extension"),
            }
        }
        "compile" | "c" => {
            let path = &args[2];
            let program_file = fs::read_to_string(path);
            match program_file {
                Ok(prog) => {
                    let file_name = &path[0..path.len() - 3];
                    let compiled_prog = brain_rust::compiler::compile(&prog);
                    let mut output = File::create(format!("{}.rbf", file_name))
                        .expect("Was unable to create file.");
                    for i in &compiled_prog {
                        output
                            .write_all(format!("{}", i).as_bytes())
                            .expect("Was unable to write data to file.");
                    }
                }
                Err(_) => println!("ERROR: Invalid path: {}", path),
            }
        }
        _ => {
            println!(
                "Invalid sub command!\n\nAll valid sub commands:\n\t run [PATH]\n\t compile [PATH]"
            )
        }
    }
}
