use std::num::ParseIntError;
use std::collections::HashMap;

fn main() -> Result<(), ParseIntError> {
    
    let input = include_str!("input.txt");

    let mut vec_a:Vec<i32> = Vec::default();
    let mut vec_b:Vec<i32> = Vec::default();

    for line in input.lines() {
        if let Some((a, b)) = line.split_once("   ") {
            vec_a.push(a.parse()?);
            vec_b.push(b.parse()?);            
        }
    }

    vec_a.sort();
    vec_b.sort();

    let sum_dist: i32 = vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a,b)| (a - b).abs())
        .sum();

    println!("Distance = {}", sum_dist);

    let mut count_bs: HashMap<i32,i32> = HashMap::default();

    for b in vec_b {
        *count_bs.entry(b).or_default() += 1;
    }

    let mut similarity = 0;
    for a in vec_a {
        let count = count_bs.get(&a).copied().unwrap_or_default();
        similarity +=  a * count;
    }

    println!("Similarity = {}", similarity);

    Ok(())
}
