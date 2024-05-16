use std::cmp::Reverse;
use std::time::{Instant};

pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
    nums.sort_by_key(|num| Reverse(*num));
    for num in &nums {
        if num < &0 { return -1 }
        if nums.contains(&(num * -1)){ return *num }
    }
    return -1
}

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let count = score.len();
    let m = *score.iter().max().unwrap() as usize;

    let mut index = vec![count; m + 1];

    for i in 0..count {
        let j = score[i] as usize;

        index[j] = i;
    }

    let mut result = vec![String::new(); count];
    let mut j = 0;

    for i in (0..m + 1).rev() {
        let k = index[i];

        if k < count {
            match j {
                0 => result[k].push_str("Gold Medal"),
                1 => result[k].push_str("Silver Medal"),
                2 => result[k].push_str("Bronze Medal"),
                p => result[k].push_str(&(p + 1).to_string())
            };

            j += 1;
        }
    }

    result
}

fn main() {
    let start = Instant::now();
    let symbols: Vec<i32> = Vec::from([-1, 2, -3, 3]);
    let resp: i32 = find_max_k(symbols);
    assert_eq!(3, resp);

    let symbols_2: Vec<i32> = Vec::from([5,4,3,2,1]);
    let resp_2: Vec<String> = find_relative_ranks(symbols_2);
    assert_eq!(Vec::from(["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]), resp_2);
    let duration = start.elapsed();
    println!("{:?}", duration);
}
