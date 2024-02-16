use std::collections::HashMap;
mod lib;
use lib::*;


fn main() {
    let line = "Hello, world!";
    let mut result:HashMap<String, i32> = HashMap::new(); 
    count_specials(line, &mut result);
    println!("{:?}", result);
}
