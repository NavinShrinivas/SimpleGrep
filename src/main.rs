use std::env;
use std::fs;

extern crate walkdir;
use walkdir::WalkDir;

mod modules;

pub struct Args {
    option: Vec<String>,
    pattern: String,
    file_content: String,
    file_content_ci: String,
}

impl Args {
    fn new(args: &Vec<String>) -> Args {
        let mut option: Vec<String> = Vec::new();
        let file: String;
        let mut pattern: String = String::new();
        let mut file_content: String = String::new();
        let file_content_ci: String = String::new();
        if args.len() >= 3 {
            for i in 1..args.len() - 2 {
                if &args[i] == "--help" {
                    option.push(String::from("-h"));
                } else if &args[i] == "--future" {
                    option.push(String::from("-f"));
                }else {
                    option.push(args[i].clone());
                }
            }
            file = String::from(args[args.len() - 1].clone());
            recurse_options_parser(&file, args.len().try_into().unwrap(), &mut file_content);
            file_content = fs::read_to_string(&file)
                .expect("Something went wrong readin file, possibly non existant file?");
            pattern = String::from(args[args.len() - 2].clone());
        } else {
            if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
                option.push(String::from("-h"));
            } else if args.len() > 1 && (args[1] == "--future" || args[1] == "-f"){
                option.push(String::from("-f"));
            } else {
                println!("Not enough option/fields, please use --help to see usage of this tool!");
                option.push(String::from("-invalid"));
                //Intentionally add unrecoginsable options to give correct error messages.
            }
        }
        Args {
            option,
            pattern,
            file_content,
            file_content_ci,
        }
    }
}

fn main() {
    println!("Note : This program can only parse relative paths");
    let (op, args_struct) = options_parser();
    match op {
        -1 => {
            println!("Invalid options detected!");
            println!("use option --help to see all valid/possible options.");
            println!("use option --future to see all options that will be added in the future."); 
        }
        0 => {
            modules::modules::print_help();
        }
        1 => {
            //Normal strcict search
            modules::stringmatch::strict_match(args_struct);
        }
        2 => {
            //non-strict searching
        }
        3 => {
            //Do nothing as invalid options detected
        }
        4 => {
            modules::modules::print_future(); // future flags 
        }
        _ => {
            panic!("Something went wrong on our side.");
        }
    }
}

fn recurse_options_parser(myfile: &String, n_args:i32, file_content: &mut String){
    for file in WalkDir::new(myfile).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("{}", file.path().display());
            if n_args >= 3 {
                let content = fs::read_to_string(&file.path())
                .expect("Something went wrong readin file, possibly non existant file?");
                file_content.push_str(&content);
            }
        }
    }
}

fn options_parser() -> (i32, Args) {
    let args: Vec<String> = env::args().collect();
    let mut args_struct = Args::new(&args);
    if args_struct.option.contains(&String::from("-invalid")) {
        return (3, args_struct);
    } else {
        println!("Options detected : {:#?}", args_struct.option);
    }
    let mut detected_options = 0;

    //options matching in decresing order. lower is higher priorty
    let mut ret = 1;

    if args_struct.option.contains(&String::from("-s")) {
        ret = 1;
        detected_options += 1;
    }
    if args_struct.option.contains(&String::from("-ns")) {
        detected_options += 1;
        ret = 2;
    }
    if args_struct.option.contains(&String::from("-c")) {
        detected_options += 1;
    }
    if args_struct.option.contains(&String::from("-ci")) {
        detected_options += 1;
        args_struct.file_content_ci = args_struct.file_content.to_lowercase().clone();
        args_struct.pattern = args_struct.pattern.to_lowercase().to_string();
    }
    if args_struct.option.contains(&String::from("-h")) {
        detected_options += 1;
        ret = 0;
    }
    if args_struct.option.contains(&String::from("-f")) {
        detected_options += 1;
        ret = 4;
    }

    if detected_options < args_struct.option.len() {
        ret = -1;
    }
    return (ret, args_struct);
}
