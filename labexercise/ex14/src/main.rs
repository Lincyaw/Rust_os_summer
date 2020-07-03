fn main() {
    let mut i = 0;
    let mut number = vec![];


    while i<6 {
        println!("At the top i is {}", i);
        number.push(i);
        i+=1;
        println!("number now: {:?}", number);
        println!("At the botton i is {}", i);
    }
    println!("The numbers:");
    for num in number{
        println!("{}", num);
    }
}
