fn main() {
    let val1: i8 = 5;
    let val2: i8 = 2;
    let ans:i8 = val1+val2;
    println!("{}" , ans);

    let mut vec1 = vec![2,4,6,8,10];
    let final_length = vec1.len();
    vec1.pop();
    vec1.push(12);
    println!("{:?}" , vec1);


    concat_string("world");

    control_flow(21);
}


fn concat_string(phrase:&str){
    println!("Hello {}",phrase);
}

fn control_flow(x:i64){
    if x > 50
    {
        println!("Greater than 50");
    }
    else if x > 25 {
        println!("Greater than 25 but smaller than 50");
    }
    else{
        println!("Smaller than 25");
    }
}