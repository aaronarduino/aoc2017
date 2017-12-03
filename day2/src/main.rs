use std::io;
use std::io::prelude::*;

fn main() {
    let input = get_input();
    let spreadsheet = split_input(input);

    part1(spreadsheet.clone());
    part2(spreadsheet);
}

fn part1(spreadsheet: Vec<Vec<i32>>) {
    let mut output: i32 = 0;
    for row in spreadsheet {
        let mut max: i32 = 0;
        let mut min: i32 = std::i32::MAX;
        for v in row {
            if v > max {
                max = v;
            }
            if v < min {
                min = v;
            }
        }
        output += max - min;
    }
    println!("Part 1: {:?}", output);
}

fn part2(spreadsheet: Vec<Vec<i32>>) {
    let mut output: i32 = 0;
    for row in spreadsheet {
        'g: for (i, v) in row.iter().enumerate() {
            for (ii, vv) in row.iter().enumerate() {
                if i == ii {
                    continue;
                }
                if v%vv == 0 {
                    output += v/vv;
                    break 'g; // breaks loop labeled 'g
                }
            }
        }
    }
    println!("Part 2: {:?}", output);
}

fn get_input() -> Vec<String>{
    let mut input: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
    input
}

fn split_input(input: Vec<String>) -> Vec<Vec<i32>>{
    let mut output: Vec<Vec<i32>> = Vec::new();
    for l in input {
        let tmp = l.split("\t").collect::<Vec<&str>>();
        let mut row: Vec<i32> = Vec::new();
        for item in tmp {
            row.push(item.parse::<i32>().unwrap());
        }
        output.push(row);
    }
    output
}