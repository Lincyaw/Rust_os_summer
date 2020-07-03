fn main() {
    let the_count = vec![1,2,3,4,5];
    let fruits = vec!["apples", "oranges", "pears", "apricots"];
    for number in the_count{
        println!("This is count {}", number);
    }
    for fruit in fruits{
        println!("A fruits of type: {}", fruit);
    }
    let mut elements = vec![];
    for i in 0..6 {
        println!("Adding {} to the vector",i);
        elements.push(i);
    }
    println!("{:?}",elements)
}
