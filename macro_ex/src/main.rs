// use macros::debug_print;


// macro_rules! average{
//     ($(,)* )=> {{
//         0.0
//     }};

//     ($($val:expr), + $(,)*) => {{
//         let count = 0usize $(+{let _ = stringify!($val);1})*;

//         let sum = 0.0 $(+ $val as f64)*;

//         sum / count as f64
//     }}
// }


// #[debug_print]


macro_rules! op {
    ($a: expr, $b:expr,$c:expr) => {
        match $c {
            1 => $a+$b,
            2 => $a-$b,
            3 => $a*$b,
            4 => $a/$b,
            5 => $a%$b,
            _ => -1,
        }
    }
}

fn main() {

    // println!("Hello, world!");
    // println!("{}" , average!());
    // println!("{}" , average!(1.0,2.0,3.0));
    // println!("{}" , average!(1,2,3,4,5,6));
    println!("{}" , op!(5,2,1));
    println!("{}" , op!(5,2,2));
    println!("{}" , op!(5,2,3));
    println!("{}" , op!(5,2,4));
    println!("{}" , op!(5,2,6));
}