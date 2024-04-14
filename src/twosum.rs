use std::collections::HashMap;

pub fn twosum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        let diff = target - n;

        // if let Some(&j) = map.get(&diff) {
        //     return vec![i as i32, j as i32];
        // } else {
        //     map.insert(n, i);
        // }

        let find_num = map.get(&diff);

        match find_num {
            Some(val) => return vec![i as i32, *val],
            None => map.insert(*n, i as i32)
        };

    }
    vec![]
}
