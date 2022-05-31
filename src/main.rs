use std::env;
use std::fs;



mod modules;


struct Args{
    option : Vec<String>,
    file : String,
    pattern : String,
    file_content : String,
    file_content_ci : String
}


impl Args{
    fn new(args : &Vec<String>) -> Args{

        let mut option : Vec<String> = Vec::new();
        let mut file : String = String::new();
        let mut pattern : String = String::new();
        let mut file_content : String = String::new();
        let file_content_ci : String = String::new();
        if args.len() >= 3 {
            for i  in 1..args.len()-2{
                if &args[i] == "--help"{
                    option.push(String::from("-h"));
                }else
                {
                    option.push(args[i].clone());
                }

            }
            file = String::from(args[args.len()-1].clone());
            file_content = fs::read_to_string(&file).expect("Something went wrong readin file, possibly non existant file?");
            pattern = String::from(args[args.len()-2].clone());

        }else{
            if args.len() > 1 &&  ( args[1] == "--help" || args[1] == "-h" ){
                option.push(String::from("-h"));
            }else
            {
                println!("Not enough option/fields, please use --help to see usage of this tool!");
                option.push(String::from("-invalid")); 
                //Intentionally add unrecoginsable options to give correct error messages.
            }
        }
        Args { option, file, pattern , file_content , file_content_ci}
    }
}


fn main() {
    println!("Note : This program can only parse relative paths");

    match options_parser(){
        -1 => {
            println!("Invalid options detected!");
            println!("use option --help to see all valid/possible options.");
        }
        0 => {
            modules::modules::print_help();
        }
        1 => {
            //Normal strcict search

        }
        2 => {
            //non-strict searching 

        }
        3 => {
            //Do nothing as invalid options detected

        }
        _ => {
            panic!("Something went wrong on our side.");
        }

    }
}

fn options_parser()->i32{
    let args : Vec<String> = env::args().collect();
    let mut args_struct = Args::new(&args);
    if args_struct.option.contains(&String::from("-invalid")){
        return 3;
    }else{
        println!("Options detected : {:#?}",args_struct.option);
    }
    let mut detected_options = 0;

    //options matching in decresing order. lower is higher priorty
    let mut ret = 1;

    if args_struct.option.contains(&String::from("-s")){
        ret = 1;
        detected_options+=1;
    }
    if args_struct.option.contains(&String::from("-ns")){
        detected_options+=1;
        ret = 2;
    }
    if args_struct.option.contains(&String::from("-c")){
        detected_options+=1;
    }
    if args_struct.option.contains(&String::from("-ci")){
        detected_options+=1;
        args_struct.file_content_ci = args_struct.file_content.to_lowercase().clone();
        args_struct.pattern = args_struct.pattern.to_lowercase().to_string();
    }
    if args_struct.option.contains(&String::from("-h")){
        detected_options+=1;
        ret = 0;
    }
    println!("{}",args_struct.file_content);


    if detected_options < args_struct.option.len()
    {
        ret = -1;
    }
    return ret;

}
