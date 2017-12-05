use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut count = 0;
    let mut count2 = 0;
    for row in split_input(get_input()) {
        if part1(row.clone()) {
            count += 1;
        }
        if part2(row) {
            count2 += 1;
        }
    }
    println!("count part1: {:?}", count);
    println!("count part2: {:?}", count2);
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

fn part2(row: Vec<String>) -> bool {
    let mut words: HashMap<Vec<char>, bool> = HashMap::new();
    for word in row {
        let mut tmp_word: Vec<char> = word.chars().collect();
        tmp_word.sort();
        if words.contains_key(&tmp_word) {
            return false;
        }
        words.insert(tmp_word, true);
    }
    true
}