use std::env;
fn main() {
    let mut args = env::args();          //每使用一次args.next()就会移到下一次, 可以看做C中的++1, 这次使用完之后再加1.

    println!("{}",args.next().unwrap() ); //这里打印出路径地址, 然后指向第二个参数
    println!("{}",args.next().unwrap() ); //这里打印出第二个参数, 然后指向第三个参数
    println!("{}",args.next().unwrap() );


    // let script = args.next().unwrap();
    //
    // let user_name = match args.next() {
    //     Some(s) => s,
    //     None => String::from(""),
    // };
    //
    // println!("Hello, {}, I am the {} script.", user_name, script);
}

