fn main() {
    let arr:[u8;4] =[0,1,2,3];
    let slice = &arr[0..2];
    borrowing_slice(arr,slice);


    for i in 0..6{
        println!("{}" , i)
    }
}


fn borrowing_slice(arr:[u8;4] , slice:&[u8]){
    println!("{:?}" , arr);
    println!("{:?}" , slice);
    println!("length {}" , slice.len());
    println!("{} {}" , slice[0] , slice[1]);
}
