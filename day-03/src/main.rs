use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn load_data() -> Vec<String>{
  let file = File::open("data/part-01.txt").expect("Cannot open File");
  //let file = File::open("data/test-01.txt").expect("Cannot open File");

  let reader = BufReader::new(file);
  let data = reader.lines();
  let data: Vec<String> = data
    .filter(|result| result.is_ok())
    .map(|result| result.unwrap())
    .filter(|line| !line.is_empty())
    .collect();
  return data
}

fn find_numbers(input: &Vec<String>) -> Vec<&str>{
  let number_re = Regex::new(r"(\d+)").unwrap();
  let symbol_re = Regex::new(r"[^\.\d]").unwrap();
  let mut parts : Vec<&str> = [].to_vec();
  for (ix,line) in input.iter().enumerate() {
    let matches = number_re.find_iter(&line);
    
    for m in matches {
      let left = m.start().checked_sub(1).unwrap_or(0);
      let right = Ord::min(line.len() -1, m.end().checked_add(1).unwrap_or(m.end()));
      let above = ix.checked_sub(1).unwrap_or(0);
      let below = Ord::min(input.len() - 1, ix.checked_add(1).unwrap_or(ix));
      
      if (above != ix && symbol_re.is_match(&input[above][left..right]))
      || (below != ix && symbol_re.is_match(&input[below][left..right]))
      || symbol_re.is_match(&input[ix][left..right])
      {
        parts.push(m.as_str());
      }
    }
  }
  return parts;
}

fn main() {
  let data = load_data();
  let parts = find_numbers(&data);
  let result = parts
    .iter()
    .map(|num| num.parse::<i32>().unwrap())
    .reduce(|acc, num| acc + num)
    .unwrap();
  println!("{:?}", result);

  
}
