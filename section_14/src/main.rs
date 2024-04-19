// use std::time::Duration;
// use std::thread;
// use std::sync::mpsc;
// use std::rc::Rc;
// use std::sync::{Arc,Mutex};

use rayon::prelude::*;
use num::{BigUint,One};
use std::time::Instant;

// fn factorial(num:u32)->BigUint{
//     if num == 0 || num == 1{
//         return BigUint::one()
//     }else{
//         (1..=num).map(BigUint::from).reduce(|acc,x| acc*x).unwrap()
//     }
// }


// fn multi_fact(num:u32) -> BigUint {
//     if num == 0 || num == 1{
//         return BigUint::one()
//     }else{
//         (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one() , |acc,x| acc*x)
//     }
// }


fn fib_recursive(n: u32) -> u32 {
    if n<2{
        return n;
    }

    fib_recursive(n-1) + fib_recursive(n-2)
}


fn fibonacci_join(n:u32) -> u32{

    if n<2{
        return n;
    }

    let (a,b) = rayon::join(|| fib_recursive(n-1), || fib_recursive(n-2));
    a+b

}


fn main() {

let now = Instant::now();
fib_recursive(25);
println!("{:.2?}", now.elapsed());

let now = Instant::now();
fibonacci_join(25);
println!("{:.2?}", now.elapsed());


// println!("{}", factorial(4));

// println!("{}", multi_fact(4));

// let now = Instant::now();
// factorial(5000);
// println!("{:.2?}", now.elapsed());

// let now = Instant::now();
// multi_fact(5000);
// println!("{:.2?}", now.elapsed());

//    let handle =  thread::spawn(move ||{
//         println!("Hello, world!");
//     } );

//     // thread::sleep(Duration::from_secs(1));

//     handle.join().unwrap();
//     println!("Hello, world!-2");

    // let v = vec![1,2,3];
    // let a:i32 = 12;

    // let handle = thread::spawn(move || {
    //     println!("{:?}" , v);
    // });
    // handle.join().unwrap();

    // let mut threads_handle = Vec::new();

    // for e in v {
    //     threads_handle.push(thread::spawn(move || {println!("{:?}",e)}));
    // }

    // for handle in threads_handle{
    //     handle.join().unwrap();
    // }

    // let (transmitter,receiver) = mpsc::channel();
    // let (transmitter,receiver) = mpsc::sync_channel(1000); // hold values
    // let tx = transmitter.clone();
    
    // let val = String::from("Transmitting...");
    // thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = receiver.recv().unwrap();
    // println!("{}",msg);

    // thread::spawn(move || {
    //     let vec = vec![String::from("Transmitting"),String::from("From"),String::from("Original")];
    //     for val in vec {
    //         transmitter.send(val).unwrap();
    //     }
    // });

    // thread::spawn(move || {
    //     let vec = vec![String::from("Transmitting"),String::from("From"),String::from("Clone")];
    //     for val in vec {
    //         tx.send(val).unwrap();
    //     }
    // });

    // for rec in receiver{
    //     println!("{}" , rec);
    // }


    // let rc1 = Rc::new(String::from("Test")); // not work
    // let rc1 = Arc::new(String::from("Test")); //  work
    // let rc2 = rc1.clone();

    // std::thread::spawn(move || {
    //     rc2;
    // });


    // let mutex = Arc::new(Mutex::new(0));

    // let mut handles = vec![];

    // for _ in 0..10{
    //     let mutex = Arc::clone(&mutex);
    //     let handle = thread::spawn(move || {
    //         let mut num = mutex.lock().unwrap();
         
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles{
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *mutex.lock().unwrap()); 



}
