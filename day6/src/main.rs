use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = get_input();
    let mut count = 0;
    let mut seen: HashMap<Vec<i32>, bool> =  HashMap::new();
    loop {
        count += 1;
        let max_i = find_max_index(input.clone());
        let val = input[max_i as usize];
        input[max_i as usize] = 0;
        redistribute(&mut input, max_i as usize, val);
        if seen.contains_key(&input) {
            break;
        }
        seen.insert(input.clone(), true);
    }
    println!("{:?}", count);
}


fn get_input() -> Vec<i32> {
    let mut input = String::new();
    let mut row: Vec<i32> = Vec::new();

    io::stdin().read_line(&mut input).unwrap();

    let tmp = input.split("\t").collect::<Vec<&str>>();
    for item in tmp {
        row.push(item.parse::<i32>().unwrap());
    }

    row
}

fn find_max_index(input: Vec<i32>) -> i32 {
    let mut max_value = 0;
    let mut max_index = 0;
    for (i, v) in input.iter().enumerate() {
        if v > &max_value {
            max_value = *v;
            max_index = i;
        }
    }
    max_index as i32
}

fn redistribute(input: &mut Vec<i32>, start: usize, value: i32) {
    let mut red_val = value;
    let mut i = start+1;
    loop {
        if red_val < 1 {
            return;
        }
        if i >= input.len() {
            i = 0;
        }
        input[i] += 1;
        red_val -= 1;
        i += 1;
    }
}