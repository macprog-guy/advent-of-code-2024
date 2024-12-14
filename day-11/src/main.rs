use std::collections::BTreeMap;
use core::str;

fn main() {

    const BLINK_COUNT1:usize = 25;
    
    let original_stones: Vec<_> = include_bytes!("input.txt")
        .split(|c| *c == b' ')
        .map(|v| str::from_utf8(v).unwrap().parse::<u64>().unwrap())
        .collect();

    let mut stones = original_stones.clone();
    for _ in 0..BLINK_COUNT1 {
        stones = blink(&stones);
    }

    println!("Stones: {}", stones.len());

    // ----------------------------------------------------------------------
    // Part II
    // ----------------------------------------------------------------------

    const BLINK_COUNT2:usize = 75;

    /*
        Since to count the number of stones, the order is actually not important,
        we can group stones with the same number. Then when we apply the "blink"
        we will generate as many of the output as the count.

    */
    let mut stones: BTreeMap<u64,usize> = BTreeMap::default();
    for n in &original_stones {
        *stones.entry(*n).or_default() += 1;
    }
    
    for _ in 0..BLINK_COUNT2 {
        stones = blink_grouped(&stones);
    }

    println!("Stones: {}", stones.values().sum::<usize>());   
}

fn count_digits(n: u64) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

fn split_number(n: u64, digits: u32) -> Vec<u64> {
    let k = (10 as u64).pow(digits / 2) as u64;
    let a = n / k;
    let b = n % k;
    vec![a,b]
}

fn blink(row: &[u64]) -> Vec<u64> {
    row.iter()
       .copied()
       .flat_map(|n| {
            if n == 0 {
                vec![1]
            } else {
                let digits = count_digits(n);
                if digits % 2 == 0 {
                    split_number(n, digits)
                } else {
                    vec![n * 2024]
                }
            }
       })
       .collect()
}

fn blink_grouped(row: &BTreeMap<u64,usize>) -> BTreeMap<u64,usize> {
    let mut out = BTreeMap::default();
    for (n, count) in row.iter() {
        if *n == 0 {
            *out.entry(1).or_default() += count;
        } else {
            let digits = count_digits(*n);
            if digits % 2 == 0 {
                let xy = split_number(*n, digits);
                *out.entry(xy[0]).or_default() += count;
                *out.entry(xy[1]).or_default() += count;
            } else {
                *out.entry(n * 2024).or_default() += count;
            }
        }
    }
    out
}
