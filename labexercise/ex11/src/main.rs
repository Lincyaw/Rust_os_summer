fn main() {
    let people = 30;
    let cars = 40;
    let trucks = 15;

    if cars >people{
        println!("We should take the cars.");
    }
    else if cars<people {
        println!("We should not take the cars.");
    }
    else {
        println!("We can't decide.");
    }

    if trucks>cars {
        println!("That's too many trucks.");
    }
    else if trucks<cars {
        println!("Maybe we should take the trucks.");
    }
    else {
        println!("We still can't decide.");
    }

    if people>trucks {
        println!("Alright, let's just take the trucks.");
    }
    else {
        println!("Fine, let's stay home then.");
    }
}
