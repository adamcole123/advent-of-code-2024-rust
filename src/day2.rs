use std::io::prelude::*;
use std::fs::File;

pub fn run() {
	let mut file = File::open("input/day-two.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
	let mut _numbers: Vec<Vec<i32>> = Vec::new();
    
    let num_strings: Vec<&str> = contents.split_ascii_whitespace().collect();

    let mut i = 0;
	let mut arr: Vec<i32> = [].to_vec();

    for num in num_strings.iter() {
        if i == 5 {
			_numbers.push(arr);
			arr = [].to_vec();
            i = 0;
        }
        
		arr.push(num.parse().unwrap_or_default());
		i = i+1;
    }

    println!("Day 2 Part 2: {:?}", calculate_part_1(_numbers.clone()));
}

fn calculate_part_1(mut _numbers: Vec<Vec<i32>>) -> i32{
	let mut _arr: Vec<bool> = Vec::new();
    for _set in &_numbers {
		let mut ascending: Option<bool> = None;
		let mut prev: Option<i32> = None;
		let mut i = 0;
		for _num in _set.into_iter() {
			if let Some(prev_value) = prev {
				if let Some(ascending_value) = ascending {
					if _num == &prev_value {
						_arr.push(false);
						break;
					}
					if _num > &prev_value {
						if !ascending_value {
							_arr.push(false);
							break;
						}
						if (_num - &prev_value) > 3 {
							_arr.push(false);
							break;
						}
						ascending = Some(true);
					} 
					
					if _num < &prev_value {
						if ascending_value {
							_arr.push(false);
							break;
						}
						if (&prev_value - _num) > 3 {
							_arr.push(false);
							break;
						}
						ascending = Some(false);
					}
				} else {
					ascending = Some(_num > &prev_value);
				}
			} else {
				prev = Some(*_num);
			}
			if i == 4 {
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