use std::fs;

fn main() {
    let content = fs::read_to_string("Data.txt").expect("Cannot read the file");
    let num_pairs = prepare_data(&content);
    let minimums:Vec<String> = num_pairs.into_iter().map(compare_numbers).collect();
    let result_parsed = minimums.join(" ");
    println!("{:?}", result_parsed);
}

fn prepare_data(raw_data: &String) -> Vec<&str>{
    let mut data: Vec<&str> = raw_data.split("\n").collect();
    data.remove(0);
    return data;
}

fn compare_numbers(arr: &str) -> String {
    let numbers: Vec<&str> = arr.split(" ").collect();
    let num_one = numbers[0].parse::<i32>().unwrap();
    let num_two = numbers[1].parse::<i32>().unwrap();
    let minimum:i32 = if num_one > num_two { num_two } else { num_one };
    return minimum.to_string();
}