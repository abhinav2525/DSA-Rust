use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

    let a: Vec<i32> = vec![0; 26];

    for str in strs {
        let mut copy: Vec<i32> = a.clone();
        let str_arr: Vec<u8> = str.chars().map(|x| x as u8).map(|x| x - 97).collect();

        for i in str_arr {
            copy[i as usize] += 1;
        }

        let map_item = map.get_mut(&copy);

        match map_item {
            None => {
                map.insert(copy, vec![str]);
            }
            Some(v) => {
                v.push(str);
            }
        }
    }

    map.into_iter().map(|(_, val)| val).collect()
}
