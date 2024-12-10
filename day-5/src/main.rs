use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet}};
use core::str;

fn main() {

    let mut page_pred: BTreeMap<i32,BTreeSet<i32>> = BTreeMap::default();
    let mut reports: Vec<Vec<i32>> = Vec::default();

    for line in include_bytes!("input-1.txt").split(|c| *c == b'\n') {
        let mut nums = line.split(|c| *c == b'|');
        let ia: i32 = nums.next().map(|v| str::from_utf8(v).unwrap().parse().unwrap()).unwrap();
        let ib: i32 = nums.next().map(|v| str::from_utf8(v).unwrap().parse().unwrap()).unwrap();
        page_pred.entry(ia).or_default().insert(ib);
    }

    // Add 2nd level transitive dependencies
    page_pred.entry(96).or_default().insert(54);

    for line in include_bytes!("input-2.txt").split(|c| *c == b'\n') {
        let report = line.split(|c| *c == b',')
            .map(|v| str::from_utf8(v).unwrap().parse().unwrap())
            .collect();
        reports.push(report);
    }

    // Closure to see if two pages are correctly ordered
    let page_before = |a, b| page_pred.get(&a).map(|set| set.contains(&b)).unwrap_or_default();
    
    // Closure to check if a report is ordered
    let is_ordered = |report:&[i32]| report.windows(2).map(|w| page_before(w[0], w[1])).fold(true, |acc, ok| acc && ok);


    let mut sum_correct = 0;
    let mut sum_incorrect = 0;

    for report in reports.iter_mut() {
        
        if is_ordered(&report) {
            sum_correct += report[report.len()/2];
        } else {

            report.sort_by(|a, b| { if page_before(*a, *b) { Ordering::Less } else { Ordering::Greater } });

            if !is_ordered(report) {
                println!("PROBLEM AFTER SORT!: {:?}", report);
                report.as_slice()
                    .windows(2)
                    .for_each(|w| {
                        let ordered = page_before(w[0], w[1]);
                        println!("{}, {}: {}", w[0], w[1], ordered);
                    });
            }

            sum_incorrect += report[report.len()/2];
        }
    }

    println!("Sum of correct: {}", sum_correct);
    println!("Sum of incorrect: {}", sum_incorrect);
}
