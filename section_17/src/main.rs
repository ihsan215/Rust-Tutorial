use async_std::{fs::File, io, prelude::*, task};

async fn read_file(path:&str) -> io::Result<String>{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}



fn main() {

    let task = task::spawn(async {
        let result = read_file("read.txt").await;
        match result {
            Ok(k) => println!("{}",k),
            Err(e) => println!("Error {}",e),
        }
    });

    println!("Task Started");
    task::block_on(task);
    println!("Task stopped");
}




// Future
// trait Future {
//     type Output;
//     fn poll(self: Pin<&mut self>, cx: &mut Context) -> Poll<self::Output>;

//     // poll: ready poll: pending 
// }
