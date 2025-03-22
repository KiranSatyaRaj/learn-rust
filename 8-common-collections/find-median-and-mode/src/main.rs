use std::collections::HashMap;
use std::ops::Deref;

fn main() {
    let v = vec![1, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 11];
    let median_index = v.len() / 2 - 1;
    let median = &v[median_index];

    println!("The median for vector {v:?} is {median}");

    let mut num_freq = HashMap::new();

    for num in &v {
        let count = num_freq.entry(num.clone()).or_insert(0);
        *count += 1;
    }

    let mut max: i32 = 0;

    for key in num_freq.keys().clone() {
        if num_freq.get(key).unwrap().clone() >= max {
            max = num_freq.get(key).unwrap().clone();
        }
    }

    let mut mode: i32 = 0;

    for (key, value) in &num_freq {
        if num_freq.get(&*key).unwrap().clone() == max {
            mode = *key;
        }
    }

    println!("The mode for the vector {v:?} is {mode} with frequency {max}");

    for (key, value) in &num_freq {
        println!("{key}: {}", num_freq.get(key).unwrap());
    }
}