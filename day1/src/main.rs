use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("reading file");
    let input = read_lines("input.txt");
    let mut result: Vec<u32> = vec![]; 
    for input_line in input{
        result.push(get_first_int(&input_line)*10+get_last_int(&input_line));
    }
    let result_combined:u32 = result.iter().sum();
    println!("result {}", result_combined);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_first_int(input_line: &str)-> u32{
    let re = Regex::new(r"(\[0-9])*(?<number>[0-9])(.)*").unwrap();
    re.captures(input_line).unwrap()["number"].parse().unwrap()
}
fn get_last_int(input_line: &str)-> u32{
    let re = Regex::new(r"(.)*(?<number>[0-9])(\[0-9])*").unwrap();
    re.captures(input_line).unwrap()["number"].parse().unwrap()
}
