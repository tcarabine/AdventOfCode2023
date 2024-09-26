use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = load_data();

    let scores: Vec<i32> = lines.into_iter().map(|l| calculate_score(&l)).collect();
    
    let total = scores.into_iter().fold(0, |acc, x| acc + x);
    print!("{}", total)
}

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

fn trim_string (line: &str) -> &str {

    let ix_colon = line.find(':').unwrap();

    let split =  line.get(ix_colon ..).unwrap();
    
    return split.strip_prefix(": ").unwrap();
}

fn extract_numbers (line: &str) -> (Vec<i32>, Vec<i32>) {
    let trimmed = trim_string(line);
    let (left, mut right) = trimmed.split_at(trimmed.find('|').unwrap());
    right = right.strip_prefix("|").unwrap();

    let unstring_left = unstring_numbers(left);
    let unstring_right = unstring_numbers(right);
    return (unstring_left, unstring_right);
}

fn unstring_numbers (line: &str) -> Vec<i32> {
    let nums = line.split(" ");
    let unstringed: Vec<i32> = nums.map(|s| s.trim().trim_end()).filter(|s| !s.is_empty() ).map(|s| s.parse::<i32>().unwrap()).collect();

    return unstringed;
}

fn score (winners: Vec<i32>, numbers: Vec<i32>) -> i32 {
    let winning_numbers: Vec<i32> = winners.into_iter().filter(|w| numbers.contains(w)).collect();
    let number_of_winners = winning_numbers.len() as u32;
    if number_of_winners == 0 {
        return 0;
    }
    return 2_i32.pow(number_of_winners - 1);
}

fn calculate_score(line: &str) -> i32 {
    let (winners, numbers) = extract_numbers(line);
    return score(winners, numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_removes_start() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let result = trim_string(input);
    
        assert_eq!(result, expected);
    }
    
    #[test]
    fn unstring_numbers_has_correct_output() {
        
        let input = "41 48 83 86 17";
        let expected = vec![41,48,83,86,17];

        let result = unstring_numbers(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn extact_numbers_has_correct_output() {

        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let expected_left = vec![41,48,83,86,17];
        let expected_right = vec![83,86,6,31,17,9,48,53];

        let (l,r) = extract_numbers(input);

        assert_eq!(l, expected_left);
        assert_eq!(r, expected_right);
    }

    #[test]
    fn score_gives_correct_for_1_winner() {
        let winners = vec![1,2];
        let numbers = vec![1,3,4,5,6];
        let expected = 1;

        let result = score(winners,numbers);

        assert_eq!(result, expected);
    }
    #[test]
    fn score_gives_correct_for_3_winner() {
        let winners = vec![1,2,3];
        let numbers = vec![1,2,3,4,5,6];
        let expected = 4;

        let result = score(winners,numbers);

        assert_eq!(result, expected);
        }
    #[test]
    fn score_gives_correct_for_0_winner() {
        let winners = vec![1,2];
        let numbers = vec![3,4,5,6];
        let expected = 0;

        let result = score(winners,numbers);

        assert_eq!(result, expected);
    }

    #[test]
    fn calculate_scores_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = 8;

        let result = calculate_score(input);

        assert_eq!(result, expected);
    }
}
