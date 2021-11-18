use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sub_command: String = match args.get(1) {
        Some(v) => v.to_owned(),
        None => String::from("")
    };
    match sub_command.as_str() {
        "run" => {
            let path = &args[2];
            let program_file = fs::read_to_string(path);
            match program_file {
                Ok(prog) => brain_rust::runtime::Runtime::new(&prog).run(),
                Err(_) => println!("ERROR: Invalid path: {}", path),
            }
        }
        _ => {
            println!("Invalid sub command!\n\nAll valid sub commands:\n\t run [PATH]")
        },
    }
}

