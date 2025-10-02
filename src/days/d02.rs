// https://adventofcode.com/2024/day/2

pub fn solve(input: String) -> (String, String) {
    let lines = input.lines();
    let reports = lines.map(|line| {
        let arr: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>().expect("Failed to parse integers");
        return arr;
    });

    let mut safe_reports = 0;
    let mut safe_reports_removal = 0;
    for report in reports {
        if check_report(&report) {
            safe_reports += 1;
        }
        if check_report_dampened(report) {
            safe_reports_removal += 1;
        }
    }






    (safe_reports.to_string(), safe_reports_removal.to_string())
}

fn check_report(report: &Vec<i32> ) -> bool {
    let mut increasing = false;
    let mut decreasing = false;

    for i in 0..report.len()-1 {
        let difference = report[i+1] - report[i];

        if difference.abs() < 1 || difference.abs() > 3 {
            return false;
        }

        increasing = difference > 0 || increasing;
        decreasing = difference < 0 || decreasing;
    }
    if decreasing && increasing {
        return false;
    }

    return true;
}

fn check_report_dampened(report: Vec<i32>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;

    for i in 0..report.len()-1 {
        let difference = report[i+1] - report[i];

        if difference.abs() < 1 || difference.abs() > 3 || difference > 0 && decreasing || difference < 0 && increasing {
            let mut report1 = report.clone();
            report1.remove(i);
            let mut report2 = report.clone();
            report2.remove(i+1);

            return check_report(&report1) || check_report(&report2);
        }

        increasing = difference > 0 || increasing;
        decreasing = difference < 0 || decreasing;
    }
    if decreasing && increasing {
        return false;
    }

    return true;
}


