fn main() {
    let arr1:[u8;3] = [1,2,3];
    let arr2:[u8;6] = [100;6];


    println!("index : {} , length: {}" , arr1[0] ,arr1.len());
    println!("{:?}" , arr2);


    let tuple:(u8,bool,f32) = (5,true,2.1);
    let tuple2 = (3,5);

    println!("first {} , second {} third {}" , tuple.0,tuple.1,tuple.2);

    println!("{:?}" , tuple2);


    let (a,b,c) = tuple;
    println!("first {} , second {} third {}" , a,b,c);

}
