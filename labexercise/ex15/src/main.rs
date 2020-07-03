
fn main() {
    let ten_things="Apples Oranges Crows Telephones Light Sugar";

    println!("Wait there are not 10 things in that list. Let's fix that.");

    let mut stuff:Vec<&str> = ten_things.split(" ").collect::<Vec<&str>>();
    println!("{:?}",stuff);

    let mut more_stuff = vec!["Day", "Night", "Song", "Frisbee", "Corn", "Banana", "Girl", "Boy"];

    while stuff.len() != 10 {
        let next_one  = more_stuff.pop();
        println!("Adding: {:?}", next_one);
        stuff.push(next_one.unwrap());
        println!("There are {} items now", stuff.len());
    }
    println!("{:?}", stuff);
}
