use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

enum SubCommand {
    Run,
    Compile,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let sub_command: SubCommand = {
        let command_string = match args.get(1) {
            Some(v) => v.to_owned(),
            None => String::from(""),
        };
        match command_string.as_str() {
            "run" | "r" => SubCommand::Run,
            "compile" | "c" => SubCommand::Compile,
            _ => {
                println!(
                "Invalid sub command!\n\nAll valid sub commands:\n\t run [PATH]\n\t compile [PATH]"
            );
                std::process::exit(0)
            }
        }
    };

    let path = &args[2];

    let file_extension = {
        let dot_index = path.rfind(".");
        match dot_index {
            Some(x) => &path[x..],
            None => {
                println!("ERROR: Invalid path: {} does not exists", path);
                std::process::exit(0)
            }
        }
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

    match sub_command {
        SubCommand::Run => run_cmd(&program, file_extension),
        SubCommand::Compile => compile_cmd(&program, &path),
    }
}

fn run_cmd(program: &str, file_ext: &str) {
    let prg = if file_ext == ".bf" {
        brain_rust::compiler::compile(program)
    } else {
        brain_rust::parser::parse(program)
    };

    brain_rust::runtime::Runtime::new(prg).run()
}

fn compile_cmd(program: &str, path: &str) {
    let file_name = &path[0..path.rfind(".").unwrap()];
    let compiled_program = brain_rust::compiler::compile(program);
    let mut output =
        File::create(format!("{}.rbf", file_name)).expect("Was unable to create file.");
    for i in &compiled_program {
        output
            .write_all(format!("{}", i).as_bytes())
            .expect("Was unable to write data to file.");
    }
}
