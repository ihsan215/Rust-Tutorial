// struct Point<T,U>{
//     x:T,
//     y:U,
// }


// trait Overview {
//     fn overview(&self) -> String{
//         String::from("This is a defult fn")
//     }
// }

// struct Course{
//     headline:String,
//     author:String
// }

// impl Drop for Course{
//     fn drop(&mut self){
//         println!("Dropping : {}" , self.author);
//     }
// }


// struct AnotherCourse{
//     headline:String,
//     author:String
// }


// impl Overview for Course{}
// impl Overview for AnotherCourse {
//     fn overview(&self) -> String{
//         format!("{} {} " , self.author, self.headline)
//     }
// }

use std::ops::Add;

#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}

impl<T> Add for Point<T>
    where
    T: Add<Output = T>{
        type Output = Self;
        fn add(self, rhs: Self) -> Self{
            Point{
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }


fn main() {

    let point1 = Point{x:1.0, y:5.5};
    let point2 = Point{x:3.0, y:7.5};

    let point3 = point1 + point2;
    println!("{:?}" , point3);

    // let coord1 = Point{x:1 , y:2};
    // let coord2 = Point{x: "x" , y: 1};

    // let course1 = Course{headline:"course-1".to_string() , author:"ihsan".to_string()};
    // let course2 = AnotherCourse{headline:"course-2".to_string() , author:"ihsan-2".to_string()};


    // println!("{}" , course1.overview());
    // println!("{}" , course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);

}

// fn call_overview(item: &impl Overview){
//     println!("{}" , item.overview());
// }

// fn call_overview<T:Overview>(item: &T){
//     println!("{}" , item.overview());
// }