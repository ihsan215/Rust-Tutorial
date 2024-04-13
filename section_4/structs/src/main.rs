// struct User {
//     active: bool,
//     user_name: String,
//     sign_in_count: u32,
// }

// struct Coordinate(i32,i32,i32);

// struct UnitStruct;

// struct Square {
//     width: u32,
//     height: u32,
// }

// impl Square {
//     fn calculate_area(&self)->u32{
//         self.width * self.height
//     }

//     fn get_my_witdh(&self) -> u32 {
//         self.width
//     }

//     fn change_widht(&mut self,new_widt:u32){
//         self.width = new_widt;
//     }
// }

struct MyString<'a>{
    text: &'a str,
}

fn main() {


    let str1 = String::from("This is my string");
    let s : MyString = MyString{text: str1.as_str()};
    // let user1:User = User{active:true,user_name:String::from("ihsan"),sign_in_count:0};
    // println!("{:?}" , user1.user_name);

    // let user2: User = build_user("ali".to_string());
    // println!("{:?}" , user2.user_name);

    // let cords = Coordinate(1,2,3);

    // let mut sq = Square{width:10,height:5};
    // println!{"{}" , sq.calculate_area()};
    // println!{"{}" , sq.get_my_witdh()};
    // sq.change_widht(5);
    // println!{"{}" , sq.calculate_area()};
    // println!{"{}" , sq.get_my_witdh()};

    // let mut r::<'a>: &String;

    // {
    //     let x::<'a>:String = "hello".to_string();
    //     r = &x;
    //     println!("{:?}" , r);
        
    // }
    

    
}



// fn build_user(user_name: String) -> User{
//     User {
//         user_name,
//         active:true,
//         sign_in_count:1
//     }
// }