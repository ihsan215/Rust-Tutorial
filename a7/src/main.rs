fn main() {
    let i : u8 = 2;

    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ => println!("defualt")
    }
}
