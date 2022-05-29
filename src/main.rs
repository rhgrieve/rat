use std::{env, fs, io, process::exit};

#[derive(Debug)]
enum RatErrorType {
    InvalidFlag,
    NoFileFound,
}

#[derive(Debug)]
struct RatError {
    error: RatErrorType,
    message: String,
}

impl RatError {
    fn new(error: RatErrorType, message: String) -> RatError {
        RatError { error, message }
    }
}

#[derive(Debug, Clone, Copy)]
struct RatFlags {
    output_nums: bool,
}

#[derive(Debug)]
struct RatArgs {
    flags: RatFlags,
    paths: Vec<String>,
    error: Option<RatError>,
}

impl RatArgs {
    fn new() -> RatArgs {
        RatArgs {
            flags: RatFlags { output_nums: false },
            paths: vec![],
            error: None,
        }
    }
}

impl RatArgs {
    fn parse(args: env::Args) -> RatArgs {
        let mut r = RatArgs::new();

        let args_vec: Vec<String> = args.collect();

        for arg in &args_vec[1..] {
            if arg.eq("-") {
                r.paths.push(arg.to_string())
            } else if arg.starts_with("-") || arg.starts_with("--") {
                let flag = arg.trim_start_matches("-");
                match flag {
                    "n" | "number" => r.flags.output_nums = true,
                    "h" | "help" => display_help(),
                    "v" | "version" => display_version(),
                    default => {
                        r.error = Some(RatError::new(
                            RatErrorType::InvalidFlag,
                            format!("Invalid flag '{}'", default),
                        ))
                    }
                }
            } else {
                r.paths.push(arg.to_string());
            }
        }

        return r;
    }
}

fn run(args: RatArgs) {
    let mut concatenated_files = String::new();
    for path in args.paths {
        if path.eq("-") {
            print_concatenated_files(concatenated_files.clone(), args.flags);
            enter_repl();
        } else {
            match fs::read_to_string(path) {
                Ok(data) => concatenated_files.push_str(data.as_str()),
                Err(err) => {
                    handle_error(RatError::new(RatErrorType::NoFileFound, format!("{}", err)))
                }
            }
        }
    }

    print_concatenated_files(concatenated_files, args.flags)
}

fn print_concatenated_files(data: String, flags: RatFlags) {
    let mut line_count = 1;
    for line in data.lines() {
        if flags.output_nums {
            println!("{}    {}", line_count, line);
        } else {
            println!("{}", line);
        }
        line_count += 1;
    }
}

fn enter_repl() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                println!("{}", buffer);
                buffer.clear();
            }
            Err(err) => panic!("{}", err),
        }
    }
}

fn choose_your_adventure(args: RatArgs) {
    if args.paths.is_empty() {
        enter_repl();
    } else {
        run(args)
    }
}

fn display_help() {
    println!("Usage: rat [OPTION]... [FILE]...");
    exit(0)
}

fn display_version() {
    println!("rat v{}", env!("CARGO_PKG_VERSION"));
    println!("Copyright (c) 2022 Harrison Grieve");
    println!("License MIT: https://opensource.org/licenses/MIT");
    exit(0)
}

fn handle_error(error: RatError) {
    eprintln!("{}", error.message);
    match error.error {
        RatErrorType::InvalidFlag => eprintln!("Try 'rat --help' for more information."),
        _ => return,
    }
}

fn main() {
    let args = RatArgs::parse(env::args());

    match args.error {
        Some(error) => handle_error(error),
        None => choose_your_adventure(args),
    }
}
