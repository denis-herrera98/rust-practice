#![allow(warnings)]
use std::collections::HashMap;
// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    for _ in 0..=k {
        let poped = nums.pop().unwrap();
        nums.insert(0, poped);
    }
}

// You are given a 0-indexed integer array arr and an integer k. The array arr is circular.
// In other words, the first element of the array is the next element of the last element, 
// and the last element of the array is the previous element of the first element.

// Input: arr = [2,5,5,7], k = 3
// Output: 5

// -> 12 ---| 
//          |-- 5
// -> 17 ---|
//          |-- 3
// -> 14 ---|

pub fn sum_sub_arr_equal(arr: &mut Vec<i32>, k: &i32) -> i64 {
    let mut end = 0;
    let mut slice;
    let mut operations: i64 = 0;
    let total_sum: i32 = arr.iter().sum();
    let average: i32 = (total_sum as f32 / arr.len() as f32).round() as i32;

    println!("Avergae {:?}", average);

    arr.iter().for_each(|x| operations += (x - average).abs() as i64);
    println!("HOALAA {}", operations);

    return operations;

    // let average: f32 = total_sum as f32 / arr.len() as f32;
    for i in 0..arr.len() {

        end = i + *k as usize;
        if end >= arr.len() {
            end -= arr.len();
        } 
        
        if end < i {
            let initial_slice = &arr[0..end];
            let end_slice = &arr[i..arr.len()];
            let mut initial = initial_slice.to_vec();
            let mut end = end_slice.to_vec();
            slice = [end, initial].concat();
        } else {
            let new_slice = &arr[i..end].to_vec();
            slice = new_slice.to_vec();
        }

        println!("{:?}", slice);

        
    }

    return 12313213;
}

// Given a string containing digits from 2-9 inclusive,
// return all possible letter combinations that the number could represent.
// Return the answer in any order.
// A mapping of digits to letters (just like on the telephone buttons) is given below.
// Note that 1 does not map to any letters.

// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//
struct Ball {
    letter: String,
    connected: HashMap<String, Ball>,
}

pub fn letter_combinations(letters: String) -> Vec<String> {
    let mut phone: HashMap<String, String> = HashMap::new();
    phone.insert("2".to_string(), String::from("abc"));
    phone.insert("3".to_string(), String::from("def"));
    phone.insert("4".to_string(), String::from("ghi"));
    phone.insert("5".to_string(), String::from("jkl"));
    phone.insert("6".to_string(), String::from("mno"));
    phone.insert("7".to_string(), String::from("pqrs"));
    phone.insert("8".to_string(), String::from("tuv"));
    phone.insert("9".to_string(), String::from("wxyz"));

    for letter in letters.chars() {
        println!("{}", letter);
    }


    return vec![String::from("AS")];
}

