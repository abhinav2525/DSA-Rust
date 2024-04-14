use std::collections::HashMap;

pub fn is_anagram(s:String, t:String) -> bool {
    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    if s.len() != t.len() {
       return false;
    }
    
    let _s_vec:Vec<_> = s.chars().collect();
    let _t_vec:Vec<_> = t.chars().collect();

    for i in s.chars(){
        let x = map_s.get(&i);
        match x {
            Some(v)=> map_s.insert(i, v+1),
            None => map_s.insert(i,1)
        };
    }

    for j in t.chars() {
        let y = map_t.get(&j);
        match y {
            Some(v)=> map_t.insert(j, v+1),
            None => map_t.insert(j, 1)     
        };
    }
println!("{:?}",map_s);
println!("{:?}",map_t);
    return  map_s == map_t;
}