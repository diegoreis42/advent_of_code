use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    // ans_a();
    ans_b();
}

fn ans_a() {
    let stdin = io::stdin();
    let mut pairs: Vec<(u32, u32)> = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse::<u32>().unwrap();
        let b = iter.next().unwrap().parse::<u32>().unwrap();
        pairs.push((a, b));
    }

    let (mut v1, mut v2): (Vec<u32>, Vec<u32>) = pairs.into_iter().unzip();

    v1.sort_unstable();
    v2.sort_unstable();

    let ans = v1
        .iter()
        .zip(v2.iter())
        .fold(0, |acc, (&a, &b)| acc + a.abs_diff(b));

    println!("{}", ans);
}

fn ans_b() {
    let stdin = io::stdin();
    let mut pairs: Vec<(u32, u32)> = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse::<u32>().unwrap();
        let b = iter.next().unwrap().parse::<u32>().unwrap();
        pairs.push((a, b));
    }

    let (v1, v2): (Vec<u32>, Vec<u32>) = pairs.into_iter().unzip();

    let v2_freq = v2.iter().fold(HashMap::<u32, u32>::new(), |mut map, &x| {
        map.entry(x).and_modify(|frq| *frq += 1).or_insert(1);

        map
    });

    let ans = v1
        .iter()
        .fold(0, |acc, x| acc + (x * v2_freq.get(x).or(Some(&0)).unwrap()));

    println!("{}", ans);
}
