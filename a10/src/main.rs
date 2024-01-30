use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, &str>  = HashMap::new();

    map.insert(0,"hi");
    map.insert(1,"hi2");
    println!("{:?}" , map);
    

    match map.get(&0){
        Some(str) => println!("{}" , str),
        _ => println!("Doesn't exist in map")
    }
    
    match map.get(&1){
        Some(str) => println!("{}" , str),
        _ => println!("Doesn't exist in map")
    }

    map.remove(&0);
    println!("{:?}" , map);

}
