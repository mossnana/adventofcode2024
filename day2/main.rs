use std::fs;

fn main() {
    let reports = read_reports(String::from("input.txt"));
    let mut safe_count: i16 = 0;
    for report in reports {
        let is_safe = is_safe_report(report.clone());
        safe_count += if is_safe { 1 } else { 0 };
        if is_safe {
            continue;
        }

        let mut found_more_safe: i8 = 0;
        for i in 0..report.len() {
            let mut temp = report.clone();
            temp.remove(i);
            if is_safe_report(temp.clone()) {
                found_more_safe += 1;
            }
        }
        safe_count += if found_more_safe > 1 { 1 } else { 0 };
    }
    println!("safe {safe_count:?} reports")
}

fn read_reports(path: String) -> Vec<Vec<i32>> {
    let result = fs::read_to_string(path);
    match result {
        Ok(input) => {
            let report = input.split("\n");
            let mut reports = vec![];
            for level in report {
                let level: Vec<i32> = level
                    .split(" ")
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();
                reports.push(level);
            }
            reports
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

fn is_safe_report(report: Vec<i32>) -> bool {
    (report.windows(2).all(|w| w[0] > w[1]) || report.windows(2).all(|w| w[0] < w[1]))
        && report.windows(2).all(|w| (w[0] - w[1]).abs() < 4)
}

// Generated by chatgpt
// fn count_safe_reports_with_dampener(reports: Vec<Vec<i32>>) -> usize {
//     let mut safe_count = 0;

//     fn is_descending(report: &[i32]) -> bool {
//         report.windows(2).all(|w| w[0] >= w[1])
//     }

//     for report in reports {
//         if is_descending(&report) {
//             safe_count += 1;
//         } else {
//             // Check if removing one level makes it safe
//             let mut found_safe = false;
//             for i in 0..report.len() {
//                 let mut temp = report.clone();
//                 temp.remove(i);
//                 if is_descending(&temp) {
//                     found_safe = true;
//                     break;
//                 }
//             }
//             if found_safe {
//                 safe_count += 1;
//             }
//         }
//     }

//     safe_count
// }

