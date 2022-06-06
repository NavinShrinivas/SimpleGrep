pub fn print_logo() {
    //prints logo as a raw string, did not want to bloat package size by using ascii printer.

    let logo = String::from(
        r" 
 ____  _                 _       ____
/ ___|(_)_ __ ___  _ __ | | ___ / ___|_ __ ___ _ __
\___ \| | '_ ` _ \| '_ \| |/ _ \ |  _| '__/ _ \ '_ \
 ___) | | | | | | | |_) | |  __/ |_| | | |  __/ |_) |
|____/|_|_| |_| |_| .__/|_|\___|\____|_|  \___| .__/
                  |_|                         |_|",
    );
    blue_ln!("{}", logo);
}

pub fn print_help() {
    print_logo();
    yellow_ln!("\t General usage : executable -option1 -option2 -option3 patter_word filename");
    red_ln!(
        "NOTE : IF CONFLICTING OPTIONS EXISTS, ONES THAT APPEAR FIRST IN THIS HELP ARE EXECUTED."
    );
    red_ln!("Note : Simple grep cannot support regex, it is merely a string matching algorithm");
    println!();
    println!();
    println!("Valid Options : ");
    println!(" '-h' : To obtain this pretty nice helper :)");
    println!(" '-ci' : case insensitive search ");
    println!(" '-c' : case sensitive search (default)");
    println!(" '-s' : strict search");
    println!("\t description : Only return match if exact match is found");
    println!(" '-ns' : non-strict search, return closest matches (default)");
    println!(" '-r' : recurse search, searches all files in a given folder (only one layer)");
}

pub fn print_file_line(start: usize, end: usize, file_arr: &Vec<char>) {
    let mut line_start = start;
    let mut line_end = end;
    while line_start != 0 && file_arr[line_start] != '\n' {
        line_start -= 1;
    }
    while line_end != file_arr.len() && file_arr[line_end] != '\n' {
        line_end += 1;
    }

    for i in line_start + 1..line_end{
        //-1 to ignore the ending new line, +1 to ignore starting newline
        if i >= start && i < end {
            blue!("{}", file_arr[i]);
        } else {
            white!("{}", file_arr[i]);
        }
    }
    println!();
}
