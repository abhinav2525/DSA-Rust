use std::{collections::HashMap, vec};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

    let mut freq: HashMap<&i32, usize> = HashMap::new();

    for num in &nums{
        *freq.entry(num).or_insert(0)+=1;
    }
    
    let mut bucket: Vec<Vec<&i32>> = vec![vec![];nums.len()];

    for (key, count) in freq {
        bucket[count].push(key);
    }

    let mut result: Vec<i32> = vec![];

    for i in bucket.iter().rev(){
        for &j in i {
            result.push(*j);
        }
        if result.len() == k as usize{
            return result;
        }
    }

    result
}