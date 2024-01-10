fn sum(a:i32 , b:i32) -> i32{
    a+b
}

fn display(res:i32) {
    println!("{:?}" , res);
}

fn main() {
    
    let res = sum(5,5);
    display(res);
}
