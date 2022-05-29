use std::{env, fs, io};

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

#[derive(Debug)]
struct RatFlags {
    output_nums: bool,
    display_help: bool,
    display_version: bool,
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
            flags: RatFlags {
                output_nums: false,
                display_help: false,
                display_version: false,
            },
            paths: vec!(),
            error: None,
        }
    }
}

impl RatArgs {
    fn parse(args: env::Args) -> RatArgs {
        let mut r = RatArgs::new();

        let args_vec: Vec<String> = args.collect();

        for arg in &args_vec[1..] {
            if arg.starts_with("-") || arg.starts_with("--") {
                let flag = arg.trim_start_matches("-");
                match flag {
                    "n" | "number" => r.flags.output_nums = true,
                    "h" | "help" => r.flags.display_help = true,
                    "v" | "version" => r.flags.display_version = true,
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
        match fs::read_to_string(path) {
            Ok(data) => concatenated_files.push_str(data.as_str()),
            Err(err) => handle_error(RatError::new(RatErrorType::NoFileFound, format!("{}", err)))
        }
    }

    output_data(concatenated_files, args.flags)
}

fn output_data(data: String, flags: RatFlags) {
    if flags.display_help {
        display_help();
    } else if flags.display_version {
        display_version();
    } else {
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
        if args.flags.display_help {
            display_help()
        } else if args.flags.display_version {
            display_version()
        } else {
            enter_repl();
        }
    } else {
        run(args)
    }
}

fn display_help() {
    println!("Usage: rat [OPTION]... [FILE]...");
}

fn display_version() {
    println!("rat v{}", env!("CARGO_PKG_VERSION"));
    println!("Copyright (c) 2022 Harrison Grieve");
    println!("License MIT: https://opensource.org/licenses/MIT");
}

fn handle_error(error: RatError) {
    eprintln!("{}", error.message);
    match error.error {
        RatErrorType::InvalidFlag => eprintln!("Try 'rat --help' for more information."),
        _ => return
    }
}

fn main() {
    let args = RatArgs::parse(env::args());

    match args.error {
        Some(error) => handle_error(error),
        None => choose_your_adventure(args),
    }
}
