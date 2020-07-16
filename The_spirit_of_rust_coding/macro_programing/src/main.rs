#![feature(let_chains)]
#[macro_export]
macro_rules! unless {
    ($arg: expr, $branch: expr) => (if !$arg { $branch};);
}
fn main() {

}
fn cmp(a: i32, b: i32){
    unless!(a>b, {
    println!("{} < {}", a, b);
    })
}
#[test]
fn test_cmp() {
    let (a,b) = (3,2);
    cmp(a,b);
}

// 声明宏重复匹配的格式是  $(...)sep rep
// sep是分隔符, 可以是逗号, => , ; 什么的

macro_rules! hashmap0 {
    ($($key: expr => $value: expr), *) => {  // 该模式在处理最后一行键值对的时候, 只能匹配没有逗号结尾的情况
    {
        let mut _map = ::std::collections::HashMap::new();  // 在生成代码中定义一个空的HashMap实例_map, 这里使用了绝对路径::std::collections::HashMap避免冲突
        $(
            _map.insert($key, $value);   // 这里与匹配模式中的重复格式对应, 也用&(...)*的格式, 不同的是这里不需要分隔符了
        )*
        _map
    }
    }
}
//但是上面那个宏有点问题, 在最后一个键值对加上逗号时, 编译就会出错
macro_rules! hashmap1 {
    ($($key: expr => $value: expr,)*) => {  // 这里是新加的一条匹配规则, 注意左边的匹配规则为 $($key: expr => $value: expr,)* , 逗号在匹配模式里面
                                            // 右边递归调用了hashmap!宏, 这行规则经过匹配之后就会把 hashmap!("a"=>1, "b"=>2, ) 替换为 hashmap!("a"=>1, "b"=>2)
        hashmap1! ($($key => $value), *)
    };

    ($($key: expr => $value: expr),*) => {
{        let mut _map = ::std::collections::HashMap::new();
        $(
            _map.insert($key, $value);
        )*
        _map}
    };
}
// 或者用重复匹配技巧

macro_rules! hashmap2 {
    ($($key: expr => $value: expr),* $(,)*) => {  // 增加了$(,)*之后, 这样就可以同时匹配最后键值对结尾是否带逗号的情况
{        let mut _map = ::std::collections::HashMap::new();
        $(
            _map.insert($key, $value);
        )*
        _map}
    };
}

#[test]
fn test_hashmap(){
    let a = hashmap2!("2"=>"你是傻逼","3"=>"我不是", "5"=>"信息检索才是傻逼",);
    println!("{:?}",a);
}

// 在创建hashmap的的时候根据给定键值对的个数来预分配容量



#[test]
fn owner(){
    let mut i = 5_i32;
    let mut x = & mut i;
    {
        *x = 6;
    }
    println!("{:?}",x);
}