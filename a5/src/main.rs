fn main() {
    let mut num = 2;

    if is_even(num) {
        println!("Even");
    }
    else{
        println!("Odd");
    }

    num = 5;


    
}


fn is_even(num:u8) -> bool{
    let digit: u8 = num %2;
    digit == 0
}