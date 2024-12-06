use std::{fs::File, io::Read};

use regex::Regex;
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    // ans_a(input);
    ans_b(input);
}

fn ans_b(input: String) {
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mul_enabled = true;
    let mut ans = 0;

    for cap in Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(&input)
    {
        if let Some(_) = do_regex.find(&cap[0]) {
            mul_enabled = true;
        } else if let Some(_) = dont_regex.find(&cap[0]) {
            mul_enabled = false;
        } else if let Some(mul_match) = mul_regex.captures(&cap[0]) {
            if mul_enabled {
                let a: i32 = mul_match[1].parse().unwrap();
                let b: i32 = mul_match[2].parse().unwrap();

                ans += a * b;
            }
        }
    }

    println!("{}", ans);
}

fn ans_a(input: String) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut ans: i32 = 0;

    for mat in re.captures_iter(&input) {
        let a: i32 = mat[1].parse().unwrap();
        let b: i32 = mat[2].parse().unwrap();

        ans += a * b;
    }

    println!("{}", ans);
}
