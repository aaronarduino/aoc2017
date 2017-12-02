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

    let c = input.chars().nth(0).unwrap();
    if c == input.pop().unwrap() {
        numbers.push(c.to_digit(10).unwrap());
    }

    println!("{:?}", numbers.iter().sum::<u32>());
}
