mod list{
    
    pub struct Tasks{
        pub item:String
    }

    // pub mod things_todo{
    //    pub fn add_activity(){}
    //     fn update_activity(){}
    //     fn marked_completed(){}
    // }

    mod items_completed{
        fn remove_task(){}
        fn move_back_todo(){}
    }
}


mod things_todo;
use things_todo::add_activity;

fn lets_add_task(){
    let tast = list::Tasks{item:String::from("Tasks")};
    // list::things_todo::add_activity(); // relative path
    // create::list::things_todo::add_activity(); // absolute path

    // things_todo::add_activity(); 
    add_activity();

}