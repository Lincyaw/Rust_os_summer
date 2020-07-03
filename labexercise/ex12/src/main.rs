use std::io::stdin;

fn main() {
    println!("You enter a dark room with two doors. Do you go through door #1 or door #2?");
    let mut door = String::new();
    stdin().read_line(&mut door).unwrap();

    if door == "1\n".to_owned(){
        println!("There is a giant bear here eating a cheese cake.");
        println!("What do you do?");
        println!("1. Take the cake.");
        println!("2. Scream at the bear.");

        let mut bear = String::new();
        stdin().read_line(&mut bear).unwrap();

        if bear == "1\n".to_string() {
            println!("The bear eats your face off. Good job!");
        }
        else if bear == "2\n".to_string() {
            println!("The bear eats your legs off. Good job!");
        }
        else {
            println!("Well, doing {} is probably better.", bear);
            println!("Bear runs away.");
        }
    }
    else if door == "2\n".to_string() {
        println!("You stare into the endless abyss at Cthulhu's retina.");
        println!("1. Blueberries");
        println!("2. Yellow jacket clothespins");
        println!("3. Understanding revolvers yelling melodies");

        let mut insanity = String::new();
        stdin().read_line(&mut insanity).unwrap();

        if insanity == "1\n".to_string() || insanity == "2".to_string() {
            println!("Your body survives powered by a mind of jello.");
            println!("Good job!");
        }
        else {
            println!("The insanity rots your eyes into a pol of muck.");
            println!("Good job!");
        }

    }
    else {
        println!("You stumble around and fall on a knife and die. Good job!");
    }



}
