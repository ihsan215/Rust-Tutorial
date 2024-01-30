fn main() {
    // print_phase("print my argument");
    println!("{}",gcd(20,5));
    println!("{}",multiple_reutrn_values(true));
    println!("{}",multiple_reutrn_values(false));
}

// fn print_phase(phrase:&str){
//     println!("{:?}" , phrase);
// }


fn gcd(mut a:u64,mut b:u64) ->u64{

    while a!=0{
        if a<b{
            let c = a;
            a = b;
            b = c;
        }
        a = a %b;
    }
    b
}

fn multiple_reutrn_values(flag:bool) -> bool {
    if flag{
        return true;
    }
    false
}