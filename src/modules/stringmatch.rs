use crate::Args;
use std::collections::HashMap;

struct Table {
    shift_table: HashMap<String, i32>,
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
                let value_ref = (*self).shift_table.entry(item.to_string()).or_insert(0);
                *value_ref = i32::try_from(pattern.len() - 1 - i).unwrap();
            }
        }
    }
}

pub fn strict_match(args_struct: Args) {
    let mut table = Table::new();
    table.preprocess_pattern(&args_struct.pattern);
    println!("Algorithm used : Hosrspools string matching algorithm");
    println!("Computed table : {:#?}", table.shift_table);
    if horspool_algorithm(&args_struct, &mut table, true) == false {
        println!("Match not found :(");
    } else {
        println!("MATCH FOUND! :)")
    }
}

fn horspool_algorithm(args_struct: &Args, table: &mut Table, strict: bool) -> bool {
    if strict {
        if args_struct.pattern.len() > args_struct.file_content.len() {
            return false;
        }
        let mut matched_chars: usize = 0; //unsinged size variable of number of chars matched
        let mut right_start_index: usize = args_struct.pattern.len();

        while matched_chars != args_struct.pattern.len()
            && right_start_index <= args_struct.file_content.len()
        {
            println!("Here {}",right_start_index);
            matched_chars = 0;
            while args_struct
                .file_content
                .chars()
                .nth(right_start_index - matched_chars - 1)
                .unwrap()
                == args_struct.pattern.chars().nth(matched_chars).unwrap()
            {
                matched_chars += 1;
            }
            let value_ref = table
                .shift_table
                .entry(
                    args_struct
                        .file_content
                        .chars()
                        .nth(right_start_index - matched_chars - 1)
                        .unwrap()
                        .to_string(),
                )
                .or_insert(i32::try_from(args_struct.pattern.len()).unwrap());
            right_start_index += usize::try_from(*value_ref).unwrap();
            println!("{:?}",usize::try_from(*value_ref).unwrap());

        }
        if matched_chars == args_struct.pattern.len() {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}
