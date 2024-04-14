mod test;
mod duplicates;
mod anagram;
mod twosum;
#[derive(Debug)]
struct Items {
    count: i32,
}
fn main() {

    let mut item = Items {count:0};
    add_one(&mut item);
    println!("{:?}", item);
    test::tesify();

    let vc = vec![1,2,3,1];
    let result = duplicates::find_duplicates(&vc);
    println!("{}", result);

    let a = String::from("tan");
    let b=String::from("ana");

    let result = anagram::is_anagram(a, b);
    println!("{}", result);

    let result = twosum::twosum([0,7,11,15,2].to_vec(), 9);
    println!("{:?}",result)
   
}

fn add_one(item:&mut Items){
    item.count += 1;
}