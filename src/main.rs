use std::collections::HashMap;

mod lib;
use lib::*;


fn main() {
    let line = "Hello! my name is FlocFlocFloc!1! And this is a f*|*|g test 100%";
    let test:HashMap<char,u32> = get_line_counts(line);
    let path = "./test.txt";
    let test:HashMap<char,u32> = read_lines_and_counts(path);
    println!("{:?}", test);
}
