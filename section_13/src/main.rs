// use std::rc::Rc;
// use std::cell::RefCell;


// #[derive(Debug)]
// struct Flagger{
//     is_true: Rc<RefCell<bool>>,
// }

// fn main() {
//     let t = (12,"eggs"); // created on the stack
//     let b = Box::new(t); // created on the heap, but b stored on the stack
//     println!("{:?}", b);

//     let s1 = Rc::new(String::from("Pointer"));
//     let s2 = s1.clone();
//     let s3 = s2.clone();
//     println!{"{:?} {:?} {:?}" , s1,s2,s3};

//     let flag = Flagger {is_true: Rc::new(RefCell::new(true))};
//     // borrow returns Ref<T>
//     // borrow_mut return RefMut;

//     let reference = Rc::new(flag.is_true.clone());
//     println!("{:?}" , reference);

//     let mut mut_ref = reference.is_true.borrow_mut();
//     *mut_ref = false;
//     println!("{}" , mut_ref);
//     println!("{:?}" , flag.is_true);
//     println!("{:?}" , flag);


// }

use std::rc::Rc;

fn main(){
    let val = 5;
    let val2 = Box::new(2);

    println!("{}" , val * *val2);

    let rc_value = String::from("RC value");


    {
        let rc: Rc<String> = Rc::new(rc_value);
        println!("{}", Rc::strong_count(&rc));

        {
            
            let rc2: Rc<String> = Rc::clone(&rc);
            println!("{}", Rc::strong_count(&rc));
            println!("{}", Rc::strong_count(&rc2));
        }
        println!("{}", Rc::strong_count(&rc));
   
    }

  
}