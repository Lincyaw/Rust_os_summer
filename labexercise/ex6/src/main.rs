use std::io::stdin;

fn main() {
    let mut age = String::new();
    println!("How old are you?");
    stdin().read_line(&mut age).unwrap();
    print2!("{}", age);


    let mut height = String::new();
    println!("How tall are you?");
    stdin().read_line(&mut height).unwrap();
    print!("{}", height);


    let mut weight = String::new();
    println!("How much do you weight?");
    stdin().read_line(&mut weight).unwrap();
    print!("{}", weight);
	
	println!("So, you're {} old, {} tall and {} heavy.", age, height, weight);
}
