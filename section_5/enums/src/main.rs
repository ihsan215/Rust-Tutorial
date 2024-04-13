// enum Pet {Dog,Cat,Fish}

// impl Pet {
//     fn who_am_i(self) -> &'static str{
//         match self{
//             Pet::Dog => "I am a dog",
//             Pet::Cat => "I am a cat",
//             Pet::Fish => "I am a fish" 
//         }
//     }
// }

// #[derive(Debug)]
// enum IpAddrKind{
//     V4(String),
//     V6,
// }
 

//  struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
//  }

fn main() {
    // let dog = Pet::Dog;
    // println!("{}" , dog.who_am_i());

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // println!("{:?}" , home);
    // let lockpack = IpAddr{kind:IpAddrKind::V6,address:"::1".to_string()};

    let four: Option<i32> = Some(4);
    let five = plus_one(four);
    println!("{:?}",five) 
}


fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}