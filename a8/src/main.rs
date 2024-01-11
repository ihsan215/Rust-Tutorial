
fn main() {
    let name : String = String::from("Bird");
    let bird : Bird = Bird{name,attack:5123};
    bird.print_name();
    println!("{} {}",bird.is_animal(), bird.can_fly());
}


struct Bird {
    name: String,
    attack: u64
}

impl Bird{
    fn print_name(&self){
        println!("{}" , self.name);
        println!("{}" , self.attack);
    }
}

impl Animal for Bird{
    fn can_fly(&self) -> bool {
        true
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        true
    }
}