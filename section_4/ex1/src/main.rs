#[derive(Debug)] 
struct Car{
    mpg: i32,
    color: String,
    top_speed: i32
}

impl Car {
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

fn main() {
    let mut car1: Car = Car {
        mpg: 4,
        color:"blue".to_string(),
        top_speed: 100
    };

    car1.set_mpg(21);
    car1.set_color("red".to_string());
    car1.set_top_speed(200);

    println!{"{:?}" , car1.mpg};
    println!{"{:?}" , car1.color};
    println!{"{:?}" , car1.top_speed};
}
