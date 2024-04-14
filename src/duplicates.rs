use std::collections::HashSet;
pub fn find_duplicates(num: &Vec<i32>) -> bool {

    let mut map = HashSet::new();
    
    for i in num{
        if map.contains(&i){
          return  true;
        } else {
            map.insert(i);
        }
        
    }
    false
}