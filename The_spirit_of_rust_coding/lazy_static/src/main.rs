#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
//ref可以理解为在左边的引用, &是在右边的
//let ref a = 1 <=> let a = &1
lazy_static! {
    #[derive(Debug)]
    static ref VEC:Vec<u8> = vec![0x18u8, 0x11u8];
    #[derive(Debug)]
    static ref MAP: HashMap<u32, String> = {
        let mut map = HashMap::new();
        map.insert(18, "henry".to_owned());
        map
    };
    #[derive(Debug)]
    static ref PAGE:u32 = multi(18);
}

fn multi(i: u32) -> u32 {
    i * 2
}

fn main() {
    println!("{:?}", *PAGE);
    println!("{:?}", *VEC);
    println!("{:?}", *MAP);

    let ref a:Vec<u32> = vec![1,2,3];
    println!("{:?}",a);
}
