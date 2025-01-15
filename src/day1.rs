use std::io::prelude::*;
use std::fs::File;

pub fn run() {
	let mut file = File::open("input/day-one.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
	let mut _numbers1: Vec<i32> = Vec::new();
	let mut _numbers2: Vec<i32> = Vec::new();
    
    let num_strings: Vec<&str> = contents.split_ascii_whitespace().collect();

    let mut i = 0;

    for num in num_strings.iter() {
        if i == 0 {
            _numbers1.push(num.parse().unwrap_or_default());
            i = 1;
            continue;
        }
        
        if i == 1 {
            _numbers2.push(num.parse().unwrap_or_default());
            i = 0;    
            continue;
        }
    }

    println!("{:?}", calculate_part_1(_numbers1.clone(), _numbers2.clone()));
    println!("{:?}", calculate_part_2(_numbers1.clone(), _numbers2.clone()));
}

fn calculate_part_1(mut _numbers1: Vec<i32>, mut _numbers2: Vec<i32>) -> i32{
    _numbers1.sort();
    _numbers2.sort();

    let mut arr: Vec<i32> = Vec::new();

    let mut i = 0;
    for _num in &_numbers1 {
        arr.push((_numbers1[i] - _numbers2[i]).abs());
        i = i+1;
    }

    return arr.into_iter().sum();
}

fn calculate_part_2(mut _numbers1: Vec<i32>, mut _numbers2: Vec<i32>) -> i32{
    let occurences: Vec<i32> = _numbers1
    .iter()
    .map(|num| _numbers2.iter().filter(|num2| &num == num2).count() as i32 * num)
    .collect::<Vec<i32>>();

    return occurences.into_iter().sum();
}