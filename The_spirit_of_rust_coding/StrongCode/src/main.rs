use std::num::ParseIntError;

fn main() {
    // test1: test_shortest
}

/// Option<T>
/// Option<T>类型属于枚举体， 包括两个可边的变体 Some(T None 作为可选值，
/// Option<T>可以被使用在多种场景中。比如可边的结构体、可选的函数参数 可选的结构体
/// 字段、可空的指针、占位（如在 HashMap 中解决 remove 问题）等。

fn get_shortest(name: Vec<&str>) -> Option<&str> {
    if name.len() > 0{
        let mut shortest = name[0];
        for name in name.iter(){
            if name.len() < shortest.len() {
                shortest = *name;
            }
        }
        Some(shortest)
    }else{
        None
    }
}

// map和and_then的区别是and_then返回的不会像map一样包装一层Some
fn get_shortest_length(names: Vec<&str>) -> Option<usize>{
    get_shortest(names).map(|name| (name.len()))
}



fn show_shortest(names: Vec<&str>)->&str{
    match get_shortest(names) {
        Some(shortest) => shortest,
        None => "Not found",
    }
}
/// unwrap方法可以取出包含于Some内部的值, 但是遇到None就会引发panic
fn show_shortest_v1(names: Vec<&str>)->&str{
    get_shortest(names).unwrap()
}
///unwrap_or是对match匹配包装的语法糖, 可以制定处理None返回的值.
fn show_shortest_v2(names: Vec<&str>)->&str{
    get_shortest(names).unwrap_or("Not found")
}
/// 与unwrap_or类似, 区别是他的参数是一个FnOnce->T闭包
fn show_shortest_v3(names: Vec<&str>)->&str{
    get_shortest(names).unwrap_or_else(|| ("Not found"))
}
///在遇到None值时会引发panic, 并且可以传入参数展示指定的异常消息.
fn show_shortest_v4(names: Vec<&str>)->&str{
    get_shortest(names).expect("Not found")
}

fn square(number_str: &str) -> Result<i32, ParseIntError>{
    number_str.parse::<i32>().map(|n| n.pow(2))
}
#[test]
fn square_test(){
    match square("10"){
        Ok(n) => assert_eq!(n,100),
        Err(err) => println!("Error: {:?}",err),
    }
}

#[test]
fn test_shortest(){
    assert_eq!(show_shortest(vec!["you","are","foolish"]), "you");
    assert_eq!(show_shortest_v1(vec!["you","are","foolish"]), "you");
    assert_eq!(show_shortest_v2(vec!["you","are","foolish"]), "you");
    assert_eq!(show_shortest_v3(vec!["you","are","foolish"]), "you");
    assert_eq!(show_shortest_v4(vec!["you","are","foolish"]), "you");

    assert_eq!(show_shortest(Vec::new()), "Not found");
    assert_eq!(show_shortest_v2(Vec::new()), "Not found");
    assert_eq!(show_shortest_v3(Vec::new()), "Not found");

    assert_eq!(get_shortest_length(vec!["you","are","foolish"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new () ) , None) ;
}

#[test]
fn test_shortest_panic(){
    assert_eq!(show_shortest_v4(Vec::new()), "Not found");
    assert_eq!(show_shortest_v1(Vec::new()), "Not found");
}

