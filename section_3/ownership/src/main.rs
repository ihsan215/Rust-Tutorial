fn main() {
    // let var = 1;
    // let mut s = "hello".to_string();
    // s.push_str("world");
    // println!("{:?}",s);

    // let x = vec!["ihsan".to_string()];
    // let y = x.clone();
    // println!("{:?}" , x);
    // println!("{:?}" , y);




    // let  s = String::from("takes");
    // take_ownership(s);


    //  let x : i8 = 5;
    //  make_copy(x);
    //  println!{"{}" , x};

    //  let s3 = give_ownership();
    //  println!("{:?}",s3);

    //  let str2 = String::from("Hi");   
    //  let s3:String = takes_and_gives_back(str2);  
    //  println!("{:?}" , s3);

    let mut str1 = "Hello".to_string();
    change_str(&mut str1);
    println!("{:?}" , str1);




}


fn change_str(s: &mut String){
    s.push_str(", world!");
}

// fn take_ownership(s:String) {
//     let str1 = s;
//     println!{"{:?}" , str1};
// }

// fn make_copy(s: i8){
//     let i1 = s;
//     println!{"{}" , i1};

// }

// fn give_ownership() -> String {
//        let s = String::from("give");
//        s 
// }

// fn takes_and_gives_back(s:String) -> String{
//     s
// }