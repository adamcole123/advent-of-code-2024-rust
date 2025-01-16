use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = File::open("input/day-two.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let mut _numbers: Vec<Vec<i32>> = Vec::new();

    let _numbers: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|num| num.parse().unwrap_or_default())
                .collect::<Vec<i32>>()
        })
        .collect();

    println!("Day 2 Part 1: {:?}", calculate_part_1(_numbers.clone()));
}

fn calculate_part_1(mut _numbers: Vec<Vec<i32>>) -> i32 {
    let mut _arr: Vec<bool> = Vec::new();
    for _set in &_numbers {
        let mut prev_diff: Option<i32> = None;
        let mut prev: Option<i32> = None;
        let mut i = 0;
        for _num in _set.into_iter() {
            if let Some(prev_value) = prev {
                let cur_diff = _num - &prev_value;
                if i32::abs(cur_diff) > 3 || i32::abs(cur_diff) < 1 {
                    _arr.push(false);
                    break;
                }
                if let Some(prev_diff_val) = prev_diff {
                    if prev_diff_val.is_positive() != cur_diff.is_positive() {
                        _arr.push(false);
                        break;
                    }
                }
                prev_diff = Some(cur_diff);
                prev = Some(*_num);
            } else {
                prev = Some(*_num);
            }
            if i == _set.len() - 1 {
                _arr.push(true);
                break;
            }
            i = i + 1;
        }
    }

    return _arr.into_iter().filter(|safe| *safe).count() as i32;
}

// fn calculate_part_2(mut _numbers1: Vec<i32>, mut _numbers2: Vec<i32>) -> i32{
//     let occurences: Vec<i32> = _numbers1
//     .iter()
//     .map(|num| _numbers2.iter().filter(|num2| &num == num2).count() as i32 * num)
//     .collect::<Vec<i32>>();

//     return occurences.into_iter().sum();
// }
