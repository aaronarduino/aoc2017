use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut count = 0;
    for row in split_input(get_input()) {
        if part1(row) {
            count += 1;
        }
    }
    println!("count part1: {:?}", count);
}

fn part1(row: Vec<String>) -> bool {
    let mut words: HashMap<String, bool> = HashMap::new();
    for word in row {
        if words.contains_key(&word) {
            return false;
        }
        words.insert(word, true);
    }
    true
}


fn get_input() -> Vec<String>{
    let mut input: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
    input
}

fn split_input(input: Vec<String>) -> Vec<Vec<String>>{
    let mut output: Vec<Vec<String>> = Vec::new();
    for l in input {
        let tmp = l.split(" ").collect::<Vec<&str>>();
        let mut row: Vec<String> = Vec::new();
        for item in tmp {
            row.push(item.to_string());
        }
        output.push(row);
    }
    output
}

fn is_anagram() {
    unimplemented!();
}