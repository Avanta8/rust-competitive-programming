#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain
)]

use std::cmp::*;
use std::collections::*;

#[derive(Clone, PartialEq, Eq, Copy)]
enum Choice {
    Unknown,
    Impostor,
    Crewmate,
}

use Choice::*;

fn query(a: usize, b: usize, c: usize) -> i64 {
    println!("? {} {} {}", a, b, c);
    read_int()
}

pub fn solve_one(n: usize) {
    let wrap = |i| (i - 1) % n + 1;

    let mut arr = vec![Unknown; n + 1];
    let mut prev = query(1, 2, 3);
    for i in 2..=n {
        let q = query(wrap(i), wrap(i + 1), wrap(i + 2));
        if prev == 1 && q == 0 {
            arr[wrap(i - 1)] = Crewmate;
            arr[wrap(i + 2)] = Impostor;
        } else if prev == 0 && q == 1 {
            arr[wrap(i - 1)] = Impostor;
            arr[wrap(i + 2)] = Crewmate;
        }
        prev = q;
    }

    let crewmate = arr.iter().position(|&c| c == Crewmate).unwrap();
    let impostor = arr.iter().position(|&c| c == Impostor).unwrap();
    let unknown = (1..=n).filter(|&i| arr[i] == Unknown).collect::<Vec<_>>();

    for i in unknown {
        let q = query(crewmate, impostor, i);
        arr[i] = if q == 1 { Crewmate } else { Impostor }
    }

    let idxs = (1..=n)
        .filter_map(|i| (arr[i] == Impostor).then(|| i.to_string()))
        .collect::<Vec<_>>();
    println!("{}", format!("! {} {}", idxs.len(), idxs.join(" ")));
}

fn read_int() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub fn main() {
    for _ in 0..read_int() {
        solve_one(read_int() as usize);
    }
}
