use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut last = input.chars().nth(0).unwrap();
    let mut numbers: Vec<u32> = Vec::new();
    for c in input.chars().skip(1) {
        if c == last {
            numbers.push(c.to_digit(10).unwrap());
        } else {
            last = c;
        }
    }
    let numbers2 = part2(input.clone());

    let c = input.chars().nth(0).unwrap();
    if c == input.pop().unwrap() {
        numbers.push(c.to_digit(10).unwrap());
    }

    println!("{:?}", numbers.iter().sum::<u32>());
    println!("{:?}", numbers2.iter().sum::<u32>());
}

fn part2(input: String) -> Vec<u32> {
    let len = input.len();
    let in_string: Vec<char> = input.chars().collect();
    println!("input: {:?}", input);
    let mut numbers: Vec<u32> = Vec::new();
    for (i, c) in in_string.iter().enumerate() {
        let digit = c.to_digit(10).unwrap();
        let test = in_string[skip_index(len, i)].to_digit(10).unwrap();
        if digit == test {
            numbers.push(digit);
        }
    }
    numbers
}

fn skip_index(len: usize, i: usize) -> usize {
    let spacing = len / 2;
    let index = i + spacing;
    if index <= len - 1 {
        return index;
    }
    index - len
}