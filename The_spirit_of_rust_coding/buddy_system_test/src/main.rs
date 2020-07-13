fn main() {
    println!("Hello, world!");
}


#[test]
fn test1(){
    for i in (10..=1).rev(){
        println!("{}",i);
    }
    for i in (1..=10).rev(){
        println!("{}",i);
    }

}