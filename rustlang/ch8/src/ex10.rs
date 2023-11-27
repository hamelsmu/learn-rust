//Given a list of integers, use a vector and return the median 
//(when sorted, the value in the middle position) and mode 
//(the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;


pub fn run(){
    let mut ints = vec![99, 1, 2, 3, 4, 5, 5, 6, 6, 6, 9, 10, 1];
    ints.sort();

    let mode_answer = mode(&ints);
    let median_answer = median(&ints);
  
    println!("mode: {:?}", mode_answer);
    println!("median: {:?}", median_answer);
}

fn median(ints: &Vec<i32>) -> i32 {
    ints[ints.len() / 2]
}

fn mode(ints: &Vec<i32>) -> i32 {
    let mut map = HashMap::with_capacity(ints.len());

    for i in ints {
        let mut count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (k, v) in &map {
        if *v > max {
            max = *v;
            mode = **k;
        }
    }
    mode
}