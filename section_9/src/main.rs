use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("paniced here");
    // let vec = vec![5];
    // vec[10];

    // let file = File::open("error.txt");
    // let res = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("{:?}", error),

    // };

    let file = File::open("error.txt");
    let res = match file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("error.txt"){
                Ok(file_created) => file_created,
                Err(err) => panic!("Cannot create the file"),
            }
            _ => panic!("It was some other error kind"),
        }

    };

}


// enum Result<T,E>{
//     Ok(T),
//     Err(e)
// }