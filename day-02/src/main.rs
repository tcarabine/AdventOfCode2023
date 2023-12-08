use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use phf::phf_map;

static MAXIMUMS: phf::Map<&'static str, i32> = phf_map!{
  "red" => 12,
  "blue" => 14,
  "green" => 13
};

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

fn extract_id(line: &Vec<&str>) -> String {
  let game_name = line.first().unwrap().split(" ").collect::<Vec<&str>>();
  let game_id = &game_name.last().unwrap();
  return game_id.to_string();
}

fn evaluate_game(count: &i32, colour: &str) -> bool {
  let max = MAXIMUMS.get(colour).unwrap();
  count <= max
}

fn parse_game(game: &Vec<String>) -> (i32, String) {
  let count = game.first().unwrap().parse::<i32>().unwrap();
  let colour = game.last().unwrap();
  return (count.clone(),colour.to_string());
}

fn evaluate_games(games: &Vec<Vec<String>>) -> bool {
  let parsed: Vec<(i32, String)> = games.into_iter().map(|x| parse_game(x)).collect();
  let filtered: Vec<(i32, String)> = parsed.into_iter().filter(|(count, colour)| evaluate_game(count, colour)).collect();
  
  filtered.len() == games.len()
}

fn evaluate_games_part2(games: &Vec<Vec<String>>) -> i32 {
  let parsed: Vec<(i32, String)> = games.into_iter().map(|x| parse_game(x)).collect();

  let mut top_scores: HashMap<String, i32> = HashMap::from([
    ("red".to_string(),0),
    ("blue".to_string(),0),
    ("green".to_string(),0)
  ]);
 
  for (count, colour) in parsed.into_iter() { 
    let top = *top_scores.get(&colour).unwrap();
    if count > top {
      top_scores.insert(colour, count);
    }
  };

  return top_scores.into_iter().fold(1, |acc, (_,v)| acc * v);
  
}

fn extract_games(line: &Vec<&str>) -> Vec<Vec<String>> {
  let games = &line.last().unwrap().clone();
  let games = games.split(";");
  return games
    .map(|game| 
      game
        .split(',')
        .map(|pull|
          pull
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect()
        )
        .collect::<Vec<Vec<String>>>()
    )
    .flatten()
    .collect::<Vec<Vec<String>>>()
    .clone();
}

fn ingest_line(line: &str, hash: &mut HashMap<i32, Vec<Vec<String>>>) {
  let split = &line.split(":").collect::<Vec<&str>>();
  let id:i32 = extract_id(split).parse::<i32>().unwrap();
  let games = extract_games(split);
  hash.insert(id, games);
}

fn main() {
  let data = load_data();
  let mut game_set : HashMap<i32, Vec<Vec<String>>>= HashMap::new();

  for line in data{
    ingest_line(&line, &mut game_set);
  }
  let score = (&game_set)
    .into_iter()
    .filter(|(_,v)| evaluate_games(v))
    .fold(0,| acc, ( k,_)| acc + k);
  
  println!("{}", score);

  let power = (&game_set)
    .into_iter()
    .map(|(_,v)| evaluate_games_part2(&v))
    .fold(0,| acc, power| acc + power);
  println!("{}", power);

}
