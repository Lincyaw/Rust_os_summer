fn main() {
    println!("Hello, world!");
}
#[test]
fn offset_method(){
    let s: &str = "Rust";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{:?}", *ptr.offset(1) as char);
        println!("{:?}", *ptr.offset(3) as char);
        println!("{:?}", *ptr.offset(25) as char);
    }
}

#[test]
fn read_write0(){
    let x = "hello".to_string();
    let y: *const u8 = x.as_ptr();
    //调用x的as_ptr方法得到一个指向堆内存的原生指针,
    // 因为String类型本质是一个字节序列数组, 所以该指针类型是*const u8, 指向第一个字节序列
    unsafe {
        println!("{:?}",y.read() as char);
        //.read()方法是unsafe方法, read方法通过指针来读取当前指针指向的内存,
        // 但不会转移所有权(也就是说在读完内存后,该内存有可能会被其他内容覆盖
        assert_eq!(y.read() as char, 'h');
    }

}
#[test]
fn read_write1(){
    let x = [0,1,2,3];
    let y = x[0..].as_ptr() as *const [u32;4]; //这里的原生指针类型是带长度的, 如果将类型改为*const [u32;3]则只能读到前3个值
    unsafe {
        assert_eq!(y.read(), [0,1,2,3]);
        println!("{:?}",y.read());
    }
}
#[test]
fn read_write2(){
    let x = vec![0,1,2,3,4];
    let y = &x as *const Vec<i32>; //这里直接将x的引用通过as操作符转换为原生指针
    let z = &x;
    unsafe {
        println!("{:?}", z);
        println!("{:?}", y);
        println!("{:?}", y.read()); //这次调用read方法读出来的是全部元素
        assert_eq!(y.read(), [0,1,2,3,4]);
        //注意; 通过as_ptr得到的指针是字符串或数组内部的指向存放数据堆或栈内存的指针,而引用则是对字符串或数组本身的引用
    }
}

#[test]
fn read_write3(){
    let mut x = "";
    let y = &mut x as *mut &str;
    let z = "hello";
    unsafe {
        println!("{:?}", x);
        println!("{:?}", y.read());
        y.write(z);
        println!("{:?}", y.read());
        assert_eq!(y.read(), "hello");
    }
}
#[test]
fn replace0(){
    let mut v: Vec<i32> = vec![1,2,3];
    let v_ptr: *mut i32 = v.as_mut_ptr();
    unsafe {
        let old_v = v_ptr.replace(5);
        println!("{:?}",v);
        assert_eq!(1,old_v);
        assert_eq!([5,2,3], &v[..]);//v是vec, 所以在比较的时候要对其中一个转换成和另一个一样的类型
        assert_eq!([5,2,3].to_vec(), v); //v是vec, 所以在比较的时候要对其中一个转换成和另一个一样的类型
    }
}
#[test]
fn replace1(){
    let mut v: Vec<i32> = vec![1,2,3];
    let v_ptr = &mut v as *mut Vec<i32>;  //这里只是将v的可变引用转换成了可变原生指针, 该指针指向数组的全部元素
    unsafe {
        let old_v = v_ptr.replace(vec![4,5,6,7]);
        println!("{:?}",v);
        assert_eq!([1,2,3],&old_v[..]);
        assert_eq!([4,5,6,7], &v[..]);//v是vec, 所以在比较的时候要对其中一个转换成和另一个一样的类型
        assert_eq!([4,5,6,7].to_vec(), v); //v是vec, 所以在比较的时候要对其中一个转换成和另一个一样的类型
    }
}

#[test]
fn swap0(){
    let mut array = [0,1,2,3];
    let x = array[0..].as_mut_ptr() as *mut [u32;2];
    let y = array[1..].as_mut_ptr() as *mut [u32;2];
    unsafe {
        assert_eq!([0,1], x.read());
        assert_eq!([1,2],y.read());
        x.swap(y);
        assert_eq!([1,0,1,3],array);
    }
}

fn foo<'a>(input: *const u32)->&'a u32{  //input为原生指针*const u32类型, 然后通过解引用原生指针和引用符号将其转为引用
    unsafe {
        println!("{:?}",*input);
        return &*input;  //&*input相当于&(*input)
    }
}
#[test]
fn unbound_lifetime(){
    let x;
    {
        let y = 42;
        x = foo(&y);  //如果在safe rust中, 这里返回的是对y的引用
    }
    println!("Hello: {}",x);  //离开作用域后y已经被丢弃, 所以x是悬垂指针, 但是在这里可以正常编译 因为经过foo函数产生了一个未绑定生命周期的借用,所以跳过了rust的借用检查
}
// fn longer_wrong(s1: &str, s2: &str) -> &str {
//     if s2.len() > s1.len() {
//         s2
//     } else {
//         s1
//     }
// }
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

#[test]
fn lifetime(){
    let s1 = "ecmascript";
    let r;
    {
        let s2 = "rust";
        r = longer(s1, s2);
    }
    println!("{} is longer", r);
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn lifetime2(){ //&str能够运行, string::from不行
    let string1 = "long string is long";
    let result;
    {
        let string2 = "xyz";
        result = longest(string1, string2);
    }
    println!("The longest string is {}", result);
}



#[test]
fn test_t(){ //&str能够运行, string::from不行
    for current_start in 0..100{
        let lowbit = if current_start > 0 {
            current_start & (!current_start + 1)
        } else {
            32
        };
        println!("{:b}  {:b}",current_start,lowbit);
    }

}
