
use clap::Parser;
// u32 to char upper bounds included
// symbols: 33 - 47 | 58 - 64 | 91 - 96 | 123 - 126 
// numbers: 48 - 57
// letters lower: 97 - 122

type Counts = [u32; 94];

fn get_counts(line: &str) -> Counts {
    let mut out_c: Counts = [0;94];

    for idx in line.to_lowercase()
                .chars()
                .map(|x| x as u32)
                .filter(|x| x >= &&33 && x <= &&126) {
        
        out_c[(idx-33) as usize] +=1;
    }

    return out_c
}

fn sum_counts(counts_v: &Vec<Counts>) -> Counts {
    let mut out:Counts = [0;94];
    let mut counts_idx:usize = 0;
    while counts_v.len() > counts_idx {
        for (i, value) in counts_v[counts_idx].iter().enumerate(){
            out[i] += value;
        } 
        counts_idx +=1
    }

    return out
}


fn counts_to_vec(counts: &Counts) -> Vec<Vec<(char, u32)>> {
    let mut letter_counts: Vec<(char, u32)> = vec![];
    let mut symbol_counts:  Vec<(char, u32)> = vec![];
    let mut number_counts:  Vec<(char, u32)> = vec![];
    

    for (i, count) in counts.iter()
                        .enumerate()
                        .map(|(i,l)| ((i as u32)+33, l)) {
       let chr:char = match char::from_u32(i) {
            Some(chr) => chr,
            None => panic!("character not recognized in ascii!")
       }; 

       if *count  > 0 {

            if i >= 48 && i <= 57 {
                // numbers
                number_counts.push((chr, *count));
            } else if i >= 97 && i <= 122 {
                // letters
                letter_counts.push((chr, *count));
            } else {
                // symbols
                symbol_counts.push((chr, *count));
            }
        }

    }

   
    symbol_counts.sort_by_key(|&(chr,c)|  std::cmp::Reverse(c));
    letter_counts.sort_by_key(|&(chr,c)|  std::cmp::Reverse(c));
    number_counts.sort_by_key(|&(chr,c)|  std::cmp::Reverse(c));
   
     return vec![letter_counts, symbol_counts, number_counts]
}

fn display_symbol_counts(counts_vec: Vec<Vec<(char, u32)>>){
  // depending on option:
  // 0: symbols 
  // 1: numbers
  // 2: letters
  // 3: symbols & numbers
  // 4: all
  

  let rows: Vec<usize> = counts_vec.iter().map(|x| x.len()).collect();
  println!("{:14}", ">SYMBOLS--------");
  println!("{0:<5} | {1:<5}", "char", "count");
  println!("{0:<5} | {1:<5}", "-----", "-----");
  for i in 0..rows[1] {
    println!("{0:^5} | {1:^5}", &counts_vec[1][i].0,&counts_vec[1][i].1);
  }
}

#[derive(Parser)]   
struct Cli {
    path: std::path::PathBuf,
}

fn main(){
    println!("Hello World!");
    let path = std::env::args().nth(1).expect("no path given");

    let args = Cli::parse();

    println!("Reading file {:?}", args.path);
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let counts_v: Vec<Counts> = content.lines()
        .map(|x| get_counts(x))
        .collect();

    let res: Counts = sum_counts(&counts_v);
    let res_display: Vec<Vec<(char, u32)>> = counts_to_vec(&res);

    display_symbol_counts(res_display);
}

