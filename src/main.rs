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

    let path = &args[2];

    let file_extension = {
        let dot_index = path.rfind(".").unwrap();
        &path[dot_index..]
    };

    if file_extension != ".bf" && file_extension != ".rbf" {
        println!("ERROR: Invalid file extension");
        std::process::exit(0)
    }

    let program = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => {
            println!("ERROR: Invalid path: {} does not exists", path);
            std::process::exit(0)
        }
    };

    match sub_command.as_str() {
        "run" | "r" => {
            let prg = if file_extension == ".bf" {
                brain_rust::compiler::compile(&program)
            } else {
                brain_rust::parser::parse(&program)
            };

            brain_rust::runtime::Runtime::new(prg).run()
        }
        "compile" | "c" => {
            let file_name = &path[0..path.rfind(".").unwrap()];
            let compiled_program = brain_rust::compiler::compile(&program);
            let mut output =
                File::create(format!("{}.rbf", file_name)).expect("Was unable to create file.");
            for i in &compiled_program {
                output
                    .write_all(format!("{}", i).as_bytes())
                    .expect("Was unable to write data to file.");
            }
        }
        _ => {
            println!(
                "Invalid sub command!\n\nAll valid sub commands:\n\t run [PATH]\n\t compile [PATH]"
            )
        }
    }
}
