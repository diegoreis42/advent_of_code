use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut ans: u32 = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        // if is_safe_a(&levels) {
        //     ans += 1;
        // }

        if is_safe_b(&levels) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn is_safe_b(levels: &[i32]) -> bool {
    is_safe_a(levels) || is_safe_with_dampener(levels)
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);

        if is_safe_a(&modified) {
            return true;
        }
    }
    
    false
}

fn is_safe_a(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing
}
