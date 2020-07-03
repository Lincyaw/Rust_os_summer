fn main() {
    print_two("zerd", "Shaw");
    print_two_again("zed", "Shaw");
    print_one("First!");
    print_none()
}

fn print_two(a: &str, b: &str){
    println!("{}, {}", a.to_owned(), b.to_owned());
}

fn print_two_again(a: &str, b: &str){
    println!("a: {}, b: {}", a.to_owned(), b.to_owned());
}

fn print_one(a: &str){
    println!("a: {}", a.to_owned());
}

fn print_none(){
    println!("I got nothing.");
}