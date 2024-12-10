fn main() {

    let calibrations: Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b'\n')
        .filter_map(|v| String::from_utf8(v.to_owned()).ok())
        .map(|s| {
            let (val, parts) = s.split_once(": ").unwrap();
            let ival: usize = val.parse().unwrap();
            let nums: Vec<usize> = parts.split(" ").filter_map(|s| s.parse().ok()).collect();
            (ival, nums)
        })
        .collect();

    // This problem could be solved using some tree-search algorithm.
    // But since the inputs are relatively small some bit twiddling
    // can be used.

    let count2:usize = calibrations.iter()
        .filter(|(target, nums)| calibrate2(*target, nums))
        .map(|(target, _)| target)
        .sum();

    println!("Sum = {}", count2);

    let count3:usize = calibrations.iter()
        .filter(|(target, nums)| calibrate3(*target, nums))
        .map(|(target, _)| target)
        .sum();

    println!("Sum = {}", count3);
}

fn calibrate2(target: usize, nums: &[usize]) -> bool {
    
    let op_count = nums.len() - 1;
    let max = 1 << op_count;

    // println!("\n\nTARGET = {}", target);
    // println!("VALUES = {:?}", nums);
    // println!("MAX = {}", max);
    // println!("OPS = {}", op_count);

    for i in 0..max {

        let mut parts: Vec<Part> = vec![Part::Value(nums[0])];
        let mut value = nums[0];
        let mut ops = i;

        for j in 1..=op_count {
            match ops & 1 {
                0 => {
                    parts.push(Part::Op("*"));
                    parts.push(Part::Value(nums[j]));
                    value *= nums[j];
                },
                1 => {
                    parts.push(Part::Op("+"));
                    parts.push(Part::Value(nums[j]));
                    value += nums[j];
                },
                _ => { continue; },
            }

            if value == target && j == op_count {
                display_debug(i, target, value, &parts, "OK");
                return true;
            } else if value > target {
                break;
            }
                
            ops = ops >> 1;            
        }
        display_debug(i, target, value, &parts, "Failed");
    }
    false
}


fn calibrate3(target: usize, nums: &[usize]) -> bool {
    
    let op_count = nums.len() - 1;
    let max = 1 << (op_count * 2);

    // println!("\n\nTARGET = {}", target);
    // println!("VALUES = {:?}", nums);
    // println!("MAX = {}", max);
    // println!("OPS = {}", op_count);

    for i in 0..max {

        let mut parts: Vec<Part> = vec![Part::Value(nums[0])];
        let mut value = nums[0];
        let mut ops = i;

        for j in 1..=op_count {
            match ops & 3 {
                0 => {
                    parts.push(Part::Op("*"));
                    parts.push(Part::Value(nums[j]));
                    value *= nums[j];
                },
                1 => {
                    parts.push(Part::Op("+"));
                    parts.push(Part::Value(nums[j]));
                    value += nums[j];
                },
                2 => {
                    parts.push(Part::Op("||"));
                    parts.push(Part::Value(nums[j]));

                    // let base = 10.0f64;
                    // let digits = (nums[j] as f64).log10().floor() + 1.0;
                    // let factor = base.powf(digits) as usize;

                    value = format!("{}{}", value, nums[j]).parse().unwrap();
                }                
                _ => {
                    continue;
                },
            }

            if value == target && j == op_count {
                display_debug(i, target, value, &parts, "OK");
                return true;
            } else if value > target {
                break;
            }
                
            ops = ops >> 2;            
        }
        display_debug(i, target, value, &parts, "Failed");
    }
    false
}



#[derive(Debug, Clone, Copy)]
enum Part {
    Value(usize),
    Op(&'static str)
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Value(n) => write!(f, "{}", n),
            Self::Op(op) => write!(f, "{}", op),
        }
    }    
}

#[allow(unused)]
fn display_debug(index: usize, target:usize, value:usize, parts: &[Part], msg:&'static str) {
    // println!("{}: {} -> {} = {}: {}", index, target, parts.iter().map(|part| format!("{}", part)).collect::<Vec<_>>().join(" "), value, msg);
}