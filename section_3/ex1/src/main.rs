fn main() {
    let mut vec = vec![1,3,5,7];
    let res = takes_vec(&mut vec);
    vec.push(15);
    println!{"{}" , res};
    println!{"{:?}" , vec};

    add_two(&mut vec[0]);
    println!{"{:?}" , vec};

   
}


fn takes_vec(vec: &mut Vec<i8>) -> bool {
    if vec[0] == 1 {
        return true;
    }
   false
}

fn add_two(x: &mut i8){
    *x += 2;
}

