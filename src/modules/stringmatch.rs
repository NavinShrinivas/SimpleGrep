use super::modules::print_file_line;
use crate::Args;
use std::collections::HashMap;

struct Table {
    shift_table: HashMap<char, usize>,
}

impl Table {
    fn new() -> Table {
        Table {
            shift_table: HashMap::new(),
        }
    }
    fn preprocess_pattern(self: &mut Self, pattern: &String) {
        for (i, item) in pattern.chars().enumerate() {
            if i != pattern.len() - 1 {
                let value_ref = (*self).shift_table.entry(item).or_insert(0);
                *value_ref = usize::try_from(pattern.len() - 1 - i).unwrap();
            }
        }
    }
}

pub fn strict_match(args_struct: Args) {
    let mut table = Table::new();
    table.preprocess_pattern(&args_struct.pattern);
    println!("Algorithm used : Hosrspools string matching algorithm");
    println!("Computed table : {:#?}", table.shift_table);
    horspool_algorithm(&args_struct, &mut table, true);
}

fn horspool_algorithm(args_struct: &Args, table: &mut Table, strict: bool) {
    if strict {
        let pat_len = args_struct.pattern.len();
        let file_len = args_struct.file_content.len();
        if pat_len > file_len {
            return;
        }
        let mut matched_chars: usize; //unsinged size variable of number of chars matched
        let mut right_start_index: usize = pat_len;

        //Below two is done as strings are not indexable by default in rust
        let file_arr : Vec<char>;
        let raw_file_arr : Vec<char> = args_struct.file_content.chars().collect();
        if args_struct.file_content_ci.len()!=0{
            file_arr = args_struct.file_content_ci.chars().collect();
        }else{
            file_arr  = args_struct.file_content.chars().collect();
        }
        let pat_arr: Vec<char> = args_struct.pattern.chars().collect();

        while right_start_index <= file_len {
            //println!("Here {}", right_start_index);
            matched_chars = 0;
            while matched_chars != pat_len
                && file_arr[right_start_index - matched_chars - 1]
                    == pat_arr[pat_len - matched_chars - 1]
            {
                matched_chars += 1;
            }
            if matched_chars == pat_len {
                print_file_line(
                    right_start_index - matched_chars,
                    right_start_index,
                    &raw_file_arr,
                );
                right_start_index += pat_len;
            } else {
                let key_value_pair = table
                    .shift_table
                    .get(&file_arr[right_start_index - matched_chars - 1]);
                if key_value_pair == None {
                    right_start_index += pat_len;
                } else {
                    right_start_index += key_value_pair.unwrap();
                }
            }
        }
    } else {
        return;
    }
}
