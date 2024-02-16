use std::collections::HashMap;

pub fn count_specials(line:&str, char_hm:&mut HashMap<String, i32>) {

    for chr in line.chars(){
        if !chr.is_alphanumeric() && chr != ' '{
           char_hm.entry(chr.to_string()).and_modify(|x| *x += 1).or_insert(1); 
        }
    }
}

#[cfg(test)]
mod check_characters{
    use super::*;

    #[test]
    fn count_specials_test(){
        let line:&str = "Hello! my name is FlocFlocFloc!1! And this is a f*|*|g test 100%";
        let mut result:HashMap<String, i32> = HashMap::new(); 
        count_specials(line, &mut result);
        assert_eq!(result, HashMap::from([("!".to_string(), 3),("*".to_string(), 2), ("|".to_string(),2),("%".to_string(), 1)]));
    }
}
