#[derive(Debug)]
struct City{
    city:String,
    population:u64
}


fn sort_pop(city: &mut Vec<City>){
    city.sort_by_key(pop_helper);
} 

fn pop_helper(pop: &City) -> u64{
    pop.population
}

fn sort_pop_closoure(pop: &mut Vec<City>){
    pop.sort_by_key(|p| p.population);
}

fn main() {
    let a = City{city: String::from("A"), population:12};
    let b = City{city: String::from("B"), population:2123};
    let c = City{city: String::from("C"), population:312};
    let d = City{city: String::from("D"), population:124};
    let e = City{city: String::from("E"), population:122};


    let mut vec: Vec<City> = vec![a,b,c,d,e];
    // sort_pop(&mut vec);
    sort_pop_closoure(&mut vec);
    println!("{:?}" , vec);


    let add = |x:i32| -> i32 {x+1};
    // let add2 = |x| x+1;

    // let y = 5;
    // let add_y = |x| x+y;
    // let copy = add_y;
    // println!("{}" , add_y(copy(10)));

    // let vec = vec![1,2,3];

    // for i in vec.iter() {
    //     println!("{:?}" , i)
    // }

    // let vec2 = vec![1,2,3];
    // let mut iter = (&vec2).into_iter();

    // while let Some(v) = iter.next(){
    //     println!("{}" , v)
    // }

    let vec = vec![1,3,5,7,9];
    let res : Vec<i16> = vec.iter().map(|x| x*10).collect();
    println!("{:?}",res)

    
}

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }