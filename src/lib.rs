use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;


pub fn get_line_counts(line:&str)-> HashMap<char, u32>{
    let counts:HashMap<char,u32> = line.chars()
        .filter(|x| x != &' ')
        .fold(HashMap::new(), |mut acc, c| {
            acc.entry(c.to_ascii_lowercase()).and_modify(|x| *x +=1).or_insert(1u32);

            acc
        });
    return counts
}



fn read_file_line_by_line(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            // line is a String
            Ok(line) => process_line(line),
            Err(err) => handle_error(err),
        }
    }

    Ok(())
}

#[cfg(test)]
mod libtest{
    use super::*;

    #[test]
    fn get_line_counts_test(){
        let line:&str = "aaa !! 11111 B";
        let test:HashMap<char,u32> = get_line_counts(line);
        let solution:HashMap<char,u32> = HashMap::from([
            ('a', 3),
            ('!', 2),
            ('1', 5),
            ('b', 1),
        ]);
        assert_eq!(test, solution);
   }
}
