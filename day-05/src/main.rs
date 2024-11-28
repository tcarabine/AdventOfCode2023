use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = load_data();
    let seeds: Vec<u64> = lines.first().unwrap().trim_start_matches("seeds: ").split(" ").map(|part| part.parse::<u64>().unwrap()).collect();
    
    lines.remove(0);
    let almanac = extract_maps(lines);

    println!("{:#?}", almanac);

    let locations: Vec<u64> = seeds.into_iter().map(|s| walk_to_location(almanac.clone(), s)).collect();

    println!("Closest : {}", locations.iter().min().unwrap())
}

fn load_data() -> Vec<String> {
    //let file = File::open("data/test.txt").expect("Cannot open File");
    let file = File::open("data/input.txt").expect("Cannot open File");

    let reader = BufReader::new(file);
    let data = reader.lines();
    let data: Vec<String> = data
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .filter(|line| !line.is_empty())
        .collect();
    return data;
}

// fn show_progress(almanac: &HashMap<String,HashMap<u64,u64>>) {
//     for k in almanac.keys() {
//         let count = almanac.get(k).unwrap().capacity();
//         println!("{k}\t{count}");

//     }
//     println!("=========");
// }

// fn update_map(mut almanac: HashMap<String, HashMap<u64, u64>>, map_name: &String, line: String) -> HashMap<String, HashMap<u64, u64>> {
fn update_map(almanac: &mut HashMap<String, HashMap<u64, u64>>, map_name: &String, line: String) {
    let parts: Vec<u64> = line
        .split(" ")
        .map(|part| part.parse::<u64>().unwrap())
        .collect();
    let right = parts[0];
    let left = parts[1];
    let count = parts[2];

    let map = almanac.get_mut(map_name).unwrap();


    let left_vec:Vec<u64> = (left..(left+count)).collect();
    let right_vec:Vec<u64> = (right..(right+count)).collect();
    let ranges: Vec<(&u64,&u64)> = left_vec.iter().zip(right_vec.iter()).collect();
    
    map.extend(ranges.into_iter());

    // show_progress(&almanac);
    //return almanac;
}



fn extract_maps(lines: Vec<String>) -> HashMap<String, HashMap<u64, u64>> {
    let sections: Vec<&str> = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let mut almanac = HashMap::new();

    for header in &sections {
        let lookup: HashMap<u64, u64> = HashMap::new();

        almanac.insert(header.trim_end_matches(" map:").to_string(), lookup);
    }

    let mut current: String = String::new();

    for line in lines {
        match line {
            l if l.is_empty() => break,
            l if sections.iter().any(|s| s.to_string() == line) => current = l.trim_end_matches(" map:").to_string(),
            _ => update_map(&mut almanac, &current, line),
        }
    }

    return almanac;
}

fn get_next_step(lookup: &HashMap<u64,u64>, start: u64) -> u64 {

    return *lookup.get(&start).unwrap_or_else(|| &start);
}

fn walk_to_location(almanac: HashMap<String,HashMap<u64,u64>>, seed: u64) -> u64 {
    let soil = get_next_step(almanac.get("seed-to-soil").unwrap(), seed);
    let fert = get_next_step(almanac.get("soil-to-fertilizer").unwrap(), soil);
    let water = get_next_step(almanac.get("fertilizer-to-water").unwrap(), fert);
    let light = get_next_step(almanac.get("water-to-light").unwrap(), water);
    let temp = get_next_step(almanac.get("light-to-temperature").unwrap(), light);
    let hum = get_next_step(almanac.get("temperature-to-humidity").unwrap(), temp);
    let loc = get_next_step(almanac.get("humidity-to-location").unwrap(), hum);

    println!("Seed {}, Soil {}, Fertilizer {}, Water {}, Light {}, Temperature {}, Humidity {}, Location {}", seed, soil,fert,water,light,temp,hum,loc);
    return loc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_map_adds_values() {
        let mut almanac:  HashMap<String,HashMap<u64,u64>> = HashMap::from([
            ("test".to_string(), HashMap::new())
        ]);

        let current = "test".to_string();
        let line = "50 98 2".to_string();

        update_map(&mut almanac, &current, line);
        assert_eq!(almanac.get("test").unwrap().get(&98).unwrap(), &50);
        assert_eq!(almanac.get("test").unwrap().get(&99).unwrap(), &51);
        assert_eq!(almanac.get("test").unwrap().get(&49).unwrap_or_else(|| &1), &1);
        assert_eq!(almanac.get("test").unwrap().get(&52).unwrap_or_else(|| &1), &1);
    }
}
