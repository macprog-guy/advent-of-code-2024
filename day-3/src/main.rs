use regex::Regex;

fn main() {
    
    let input = include_str!("input.txt");

    let mul_pat = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    let mut result: i64 = 0;

    for caps in mul_pat.captures_iter(input) {
        let a:i64 = caps.get(1)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or_default();

        let b:i64 = caps.get(2)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or_default();

        result += a * b;
    }

    println!("Result = {}", result);


    let instr_pat = Regex::new("mul\\((\\d+),(\\d+)\\)|don't\\(\\)|do\\(\\)").unwrap();
    let mut enabled = true;
    let mut result: i64 = 0;

    for caps in instr_pat.captures_iter(input) {

        let instr = caps.get(0).map(|c|c.as_str()).unwrap_or_default();
        
        if instr.starts_with("mul") && enabled {
            
            let a:i64 = caps.get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or_default();

            let b:i64 = caps.get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or_default();
            
            result += a * b;

        } else if instr == "don't()" {
            enabled = false;
        } else if instr == "do()" {
            enabled = true;
        }
    }

    println!("Improved result = {}", result);
}
