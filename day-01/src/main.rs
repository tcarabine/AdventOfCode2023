use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn load_data() -> Vec<String>{
  let file = File::open("data/part-01.txt").expect("Cannot open File");
  // let file = File::open("data/test-01.txt").expect("Cannot open File");
  // let file = File::open("data/test-02.txt").expect("Cannot open File");

  let reader = BufReader::new(file);
  let data = reader.lines();
  let data: Vec<String> = data
    .filter(|result| result.is_ok())
    .map(|result| result.unwrap())
    .filter(|line| !line.is_empty())
    .collect();
  return data
}

fn get_calibration(input: &String) -> String{
  let re = Regex::new(r"(\d)").unwrap();

  let matches:Vec<_> = re
    .find_iter(&input)
    .map(|m| m.as_str())
    .collect();

  return format!(
    "{0}{1}",
    matches.first().unwrap(),
    matches.last().unwrap()
  )
}

fn replace_numbers(input: &String) -> String{
  let haystack = input.clone();
  let dictionary= HashMap::from([
    (r"ten","10"),
    (r"nine","9"),
    (r"eight","8"),
    (r"seven","7"),
    (r"six","6"),
    (r"five","5"),
    (r"four","4"),
    (r"three","3"),
    (r"two","2"),
    (r"one","1")
  ]);

  let replacers: Vec<(Regex,&str)> = (&dictionary)
    .into_iter()
    .map(|(re, replace)| (Regex::new(re).unwrap(), replace.to_owned()))
    .collect();

  let mut first_matches:  Vec<(usize,Regex)> = vec![];

  for re in (&replacers).into_iter().map(|(re,_)| re) {
    for f in re
      .find_iter(&haystack)
      .map(|m| m.start()) {
        let regex = re.clone();
      first_matches.push((f,regex));
    }
  }

  let mut last_matches:  Vec<(usize,Regex)> = vec![];

  for re in (&replacers).into_iter().map(|(re,_)| re) {
    for f in re
      .find_iter(&haystack)
      .map(|m| m.end()) {
        let regex = re.clone();
      last_matches.push((f,regex));
    }
  }

  if first_matches.is_empty() {
    return input.to_string();
  }

  let first = first_matches
    .into_iter()
    .min_by(|a,b| a.0.partial_cmp(&b.0).unwrap())
    .unwrap().1;

  let last = last_matches
    .into_iter()
    .max_by(|a,b| a.0.partial_cmp(&b.0).unwrap())
    .unwrap().1;

  let first_rep = dictionary.clone().get(&first.as_str()).unwrap().to_owned().to_owned();
  let last_rep = dictionary.clone().get(&last.as_str()).unwrap().to_owned().to_owned();
  let haystack = first.replace(&haystack, first_rep);
  let haystack = last.replace(&haystack, last_rep);
  let result = haystack.to_string();

  return result
}

fn main() {
  let data = load_data();
  
  let part1: i32 = (&data)
    .into_iter()
    .map(|line| get_calibration(&line))
    .filter_map(|s| s.parse::<i32>().ok())
    .fold(0, |acc, n| acc + n);
 
  let part2: i32 = (&data)
    .into_iter()
    .map(|line| replace_numbers(&line))
    .map(|line| get_calibration(&line))
    .filter_map(|s| s.parse::<i32>().ok())
    .fold(0, |acc, n| acc + n);
  println!("part 1 {0}", part1);
  println!("part 2 {0}", part2);
}