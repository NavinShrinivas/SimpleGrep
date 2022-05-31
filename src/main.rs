use std::env;
mod modules;


struct Args{
    option : Vec<String>,
    file : String,
    pattern : String
}


impl Args{
    fn new(args : &Vec<String>) -> Args{

        let mut option : Vec<String> = Vec::new();
        let mut file : String = String::new();
        let mut pattern : String = String::new();
        if args.len() >= 3 {
            for i  in 1..args.len()-2{
                option.push(args[i].clone());
            }
            file = String::from(args[args.len()-1].clone());
            pattern = String::from(args[args.len()-2].clone());
        }else{
            if args[1] == "--help" || args[1] == "-h"{
                option.push( args[1].clone());
            }else
            {
                println!("Not enough option, please use --help to see usage of this tool!");
            }
        }
        Args { option, file, pattern}
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
        _ => {
            panic!("Something went wrong on our side.");
        }

    }
}

fn options_parser()->i32{
    let args : Vec<String> = env::args().collect();
    let mut args_struct = Args::new(&args);
    println!("{:#?}",args_struct.option);
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
        args_struct.pattern = args_struct.pattern.to_lowercase().to_string();
    }
    if args_struct.option.contains(&String::from("-h")){
        detected_options+=1;
        ret = 0;
    }


    if detected_options < args_struct.option.len()
    {
        ret = -1;
    }
    return ret;

}
