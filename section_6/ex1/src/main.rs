#[derive(Debug)] 
struct Car{
    mpg: i32,
    color: String,
    top_speed: i32
}

#[derive(Debug)] 
struct Motorcycle{
    mpg: i32,
    color: String,
    top_speed: i32
}

trait Proporties {
    fn set_mpg(&mut self,new_mpg:i32);

    fn set_color(&mut self,new_color: String);

    fn set_top_speed(&mut self,new_speed:i32);
}

impl Proporties for Car {
    fn set_mpg(&mut self,new_mpg:i32){
        self.mpg = new_mpg;
    }

    fn set_color(&mut self,new_color: String){
        self.color = new_color
    }

    fn set_top_speed(&mut self,new_speed:i32){
        self.top_speed = new_speed;
    }
}

impl Proporties for Motorcycle {
    fn set_mpg(&mut self,new_mpg:i32){
        self.mpg = new_mpg;
    }

    fn set_color(&mut self,new_color: String){
        self.color = new_color
    }

    fn set_top_speed(&mut self,new_speed:i32){
        self.top_speed = new_speed;
    }
}

fn print<T: std::fmt::Debug>(value:T){
    println!("{:?}" , value);
}

fn main() {
    let mut car1: Car = Car {
        mpg: 4,
        color:"blue".to_string(),
        top_speed: 100
    };

    let mut motor: Motorcycle = Motorcycle {
        mpg: 4,
        color:"black".to_string(),
        top_speed: 150
    };

    car1.set_mpg(21);
    car1.set_color("red".to_string());
    car1.set_top_speed(200);


    motor.set_mpg(21);
    motor.set_color("green".to_string());
    motor.set_top_speed(150);

    println!{"{:?}" , car1};
    println!{"{:?}" , motor};


    print(10);
    print(vec![1,2,3]);
    print("Tesst");

}
