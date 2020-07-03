fn main() {
    println!("We can just give the function numbers directly:");
    cheese_and_crackers(20,30);

    println!("Or we can use variables from our scripts.");
    let amount_of_cheese = 10;
    let amount_of_crackers = 50;
    cheese_and_crackers(amount_of_cheese, amount_of_crackers);

    println!("We can even do math inside too:");
    cheese_and_crackers(10+20, 5+6);

    println!("And we can combine the two, variables and math:");
    cheese_and_crackers(amount_of_crackers+100, amount_of_crackers+1000);
}

fn cheese_and_crackers(cheese_count:i32 , boxes_of_crackers:i32){
    println!("You have {} cheeses!", cheese_count);
    println!("You have {} boxes of crackers!", boxes_of_crackers);
    println!("Man that's enough for a party!");
    println!("Get a blanket.\n");
}