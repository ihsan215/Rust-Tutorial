
// use rand::seq::SliceRandom;
// use rand::thread_rng;

// use std::collections::BinaryHeap;

use std::collections::HashMap;

fn main() {

    // let mut nums : Vec<i32> = Vec::new();

    // nums.push(1);
    // nums.push(5);
    // nums.push(10);

    // println!("{:?}" , nums);


    // let poped = nums.pop();


    // println!("{:?}" , poped);
    // println!("{:?}" , nums);

    // let first = nums.first();
    // println!("{:?}" , first);


    // nums.insert(0,12);
    // println!("{:?}" , nums);
    // nums.remove(2);
    // println!("{:?}" , nums);


    // nums.shuffle(&mut thread_rng());
    // println!("{:?}" , nums);


//     let mut heap = BinaryHeap::new();
//     // Let's add some scores...
//     heap.push(1);

//     heap.push(6);
//     heap.push(5);

//     for x in &heap {
//         println!("{}",*x);
//     }

//     // We can clear the heap of any remaining items.
// heap.clear();

    let mut hm = HashMap::new();

    hm.insert("test1".to_string() , 2);
    hm.insert("test2".to_string() , 4);
    hm.insert("test3".to_string() , 5);

    println!("{:?}" , hm);

    if hm.contains_key("test1") {
        println!("We've got {} reviews.",
        hm.len());
    }
}
