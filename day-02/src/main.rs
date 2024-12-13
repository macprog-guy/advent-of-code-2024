fn is_safe_report(report: &[i32]) -> bool {

    let mut n = 0;
    for w in report.windows(2) {
        let d = w[1] - w[0];
        if d.abs() >= 1 && d.abs() <= 3 {
            n += d.signum();
        }
    }
    n.abs() == (report.len() - 1) as i32
}

fn main() {

    let input = include_str!("input.txt");

    let mut reports: Vec<Vec<i32>> = Vec::default();

    for line in input.lines() {
        let report = line.split(" ")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        reports.push(report);
    }

    let mut safe_report_count = 0;

    for report in &reports {
        
        if is_safe_report(&report) {
            safe_report_count += 1;
            continue;
        } 

        for i in 0..report.len() {
            let mut dampened_report = report.clone();
            dampened_report.remove(i);
            if is_safe_report(&dampened_report) {
                safe_report_count += 1;
                break;
            }
        }
    }

    println!("Safe reports = {}", safe_report_count);
}
