enum Shape{
    Triangle,
    Square,
}

impl Shape{
    fn corner(self) -> i32 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4
        }
    }
}

fn main() {
    let triangle:Shape = Shape::Triangle;
    let square:Shape = Shape::Square;

   

    println!("{}" , triangle.corner());
    println!("{}" , square.corner());

}
