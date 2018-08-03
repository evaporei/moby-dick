use std::env;
use std::process::Command;

fn main() {
    println!("DOCKER WANNABE!");
    let args: Vec<String> = env::args().collect();
    process_args(&args);
}

fn process_args(args: &Vec<String>) {
    match args.get(1) {
        Some(command) => match command.as_ref() {
            "run" => run(&args),
            _ => panic!("Not implemented :("),
        }
        None => panic!("No command provided"),
    };
}

fn run(args: &Vec<String>) {
    println!("Running");

    let program_name = args.get(2);

    if let None = program_name {
        panic!("No program name provided");
    }
    let program_name = program_name.unwrap();

    let program_args = &args[3..];

    Command::new(program_name)
        .args(program_args)
        .status()
        .expect("Problem running program");
}
