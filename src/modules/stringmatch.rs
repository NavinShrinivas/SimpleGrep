use crate::Args;
use std::collections::HashMap;

struct Table{
    shift_table : HashMap<String,i32>
}

impl Table{
    fn new()->Table{
        Table { shift_table: HashMap::new() }
    }
    fn preprocess_pattern(self : &mut Self,pattern : &String){
        for(i,item) in pattern.chars().enumerate(){
            if i != pattern.len()-1{
                let value_ref = (*self).shift_table.entry(item.to_string()).or_insert(0);
                *value_ref = i32::try_from(pattern.len()-1-i).unwrap();
            }
        }
    }
}



pub fn strict_match(args_struct : Args){
    let mut table = Table::new();
    table.preprocess_pattern(&args_struct.pattern);
    println!("{:#?}",table.shift_table);

}
