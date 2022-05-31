pub fn print_logo(){
    //prints logo as a raw string, did not want to bloat package size by using ascii printer.


    let logo = String::from(r" 
 ____  _                 _       ____
/ ___|(_)_ __ ___  _ __ | | ___ / ___|_ __ ___ _ __
\___ \| | '_ ` _ \| '_ \| |/ _ \ |  _| '__/ _ \ '_ \
 ___) | | | | | | | |_) | |  __/ |_| | | |  __/ |_) |
|____/|_|_| |_| |_| .__/|_|\___|\____|_|  \___| .__/
                  |_|                         |_|");
    println!("{}",logo);
}


pub fn print_help(){
    print_logo();
    println!("\t General usage : executable -option1 -option2 -option3 patter_word filename");
    println!(" NOTE : IF CONFLITING OPTIONS EXISTS, ONES THAT APPEAR FIRST IN THIS HELP ARE EXECUTED.");
    println!("Valid Options : ");
    println!(" '-h' : To obtain this pretty nice helper :)");
    println!(" '-ci' : case insensitive search ");
    println!(" '-c' : case sensitive search (default)");
    println!(" '-ns' : non-strict search, return closest match");
    println!(" '-s' : strict search (default)");
    println!("\t description : Only return match if exact match is found");
    println!(" '-r' : recurse search, searchs all files in a given folder (only one layer)");
   
}
