# Rust_os_summer
鹏城实验室和清华大学合办的项目



## 2020年6月26日08:58:36

昨天在这里写了一些准备的话, 但是今天早上为了新建项目, 一不小心把readme也删了, 然后没了...

crates要换成国内的镜像源, 不然下载太慢了。

### 2020年6月26日11:33:45

struct3中的创建结构体失败的那个test不知道要做啥。

上午做到了这里



## 2020年6月28日08:16:49

昨天在完成学校的实验报告, 导致没怎么看.

今天准备把语法全部过一遍, 然后题目能做多少做多少, 明天全部结束. 再做编程题.





### 2020年6月28日09:06:17

- `Fn`：表示捕获方式为通过引用（`&T`）的闭包
- `FnMut`：表示捕获方式为通过可变引用（`&mut T`）的闭包
- `FnOnce`：表示捕获方式为通过值（`T`）的闭包



### 2020年6月28日11:57:54

看到了泛型, 泛型tcl  我C++的泛型都没搞懂







### 2020年6月28日16:20:53

定义宏中的指示符类型:

这里列出全部指示符：

- `block`
- `expr` 用于表达式
- `ident` 用于变量名或函数名
- `item`
- `pat` (**模式** *pattern*)
- `path`
- `stmt` (**语句** *statement*)
- `tt` (**标记树** *token tree*)
- `ty` (**类型** *type*)

[宏的重复](https://rustwiki.org/zh-CN/rust-by-example/macros/repeat.html)





[enum类型的解构操作](https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum.html)

```rust
// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}
```





### 2020年6月28日20:59:01

今天看到错误处理, 题目也写到错误处理。 收工。









## 2020年6月29日11:30:44

题目err1还有点问题, 关于match, err处理这块还是不熟悉

错误处理这里要多看





### 2020年6月29日16:51:36

errorsn这个文件要多看看, 裂开来



## 2020年6月30日10:37:25

给函数传数组的时候, 使用固定大小的数组, `mut arr: [u32,5]`

或者使用胖指针, `arr: &mut [u32]`







### 2020年6月30日16:30:04

刚开始学rust的时候没想那么多，在搜一些rust用法的时候发现网上没有什么关于rust的信息。然后才开始搜rust这门语言，竟然这么年轻。然后在知乎上面看到了很多关于rust的讨论，其中rust编程之道的作者张汉东老师也在里面hhhh，而且发现还是一个这么年轻的作者， 知乎上回答了很多稀奇古怪的问题hhh

![image-20200630163414065](images/readme/image-20200630163414065.png)

回到rust上，感觉rust未来可期，现在缺的只是时间积累，再过几年有了积累之后比其他语言会有很大的优势。

![image-20200630163707029](images/readme/image-20200630163707029.png)





## 2020年7月1日10:57:54

```
cd /mnt/c/users/fay/documents/github/rust_os_summer/labproject/os
```





今天看了看实验的内容, 顺便配置了一下wsl环境, 为以后实验做准备



### 2020年7月1日11:27:32

在`lab0`执行`make run`时出现如下问题, 不知道为啥

![image-20200701112600657](images/readme/image-20200701112600657.png)







### 2020年7月1日22:28:36

目前发现是wsl版本的问题导致的这个安装不完整. 

现在升级了windows系统, 重新安装了wsl, 用了version2, 不知道行不行. 

搞了一天了, 裂开了.

` rustup target add riscv64imac-unknown-none-elf`







## 2020年7月2日10:44:45

配了一整天的环境, 呜呜呜呜呜。 学校的网真垃圾md。 垃圾

用流量下好了那些包







### 2020年7月2日16:41:00

磕磕绊绊总算完成了习题,

其中迭代器\多线程\错误处理\类型转换\标准库的一些知识点还是很不熟悉, 需要进一步加强







### 2020年7月2日21:57:08

今天收工, 用rust实现了`learn python the hard way`中的7个习题

其中,第七个是读取文件, python版本中要求用户输入文件路径后再读取, 但是不知道为什么我输入之后就无法找到该文件, 即打开失败; 如果在代码内固定好要读取的文件就成功了. 

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::stdin;

fn main() {
    // 创建指向所需的文件的路径
    let mut file = String::new();
    println!("What's your path?");
    stdin().read_line(&mut file).unwrap();

    let path = Path::new(&file);
    let display = path.display();


    // // 创建指向所需的文件的路径
    // let path = Path::new("hello.txt");
    // let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.to_string()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.to_string()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
}
```



明天把剩下的几个练习实现一下,  自学rust编程这个阶段就可以告一段落了， 然后到第二个阶段， 自学risc-v系统结构。这个我估计四天左右应该能结束。加油！





## 2020年7月3日11:55:31

发现了是因为readline这个函数读出来的字符串包含了换行符, 目前不知道怎么解决.





### 2020年7月3日14:05:14

`unwrap()` 和`collect()`真是好东西, 要再看看



### 2020年7月3日15:29:07

现在开始看RISC-V架构





计算机体系结构的传统方法是**增量ISA**，新处理器不仅必须实现新的ISA扩展，还必须

实现过去的所有扩展。目的是为了保持向后的二进制兼容性，这样几十年前程序的二进制

版本仍然可以在最新的处理器上正确运行。



机器模式（缩写为 M 模式，M-mode）是 RISC-V 中 *hart*（hardware thread，硬件线

程）可以执行的最高权限模式。在 M 模式下运行的 hart 对内存，I/O 和一些对于启动和配

置系统来说必要的底层功能有着完全的使用权。



机器模式最重要的特性是拦截和处理异常（不寻常的运行时事件）的能力。RISC-V 将

异常分为两类。一类是同步异常，这类异常在指令执行期间产生，如访问了无效的存储器

地址或执行了具有无效操作码的指令时。另一类是中断，它是与指令流异步的外部事件，

比如鼠标的单击。





### 2020年7月3日19:49:31

接下来的一个星期要继续巩固rust, 看一看编程之道里面的章节

学习一下risc-v体系结构, 指令集, 特权指令集规范.



以后在完成实验的时候记得要写学习报告.

现在学一下操作系统的内容,  b站上的王道考研.







进程的五种状态:  运行态, 就绪态(万事俱备,只欠cpu), 阻塞态, 创建态, 终止态



2.1.4





## 2020年7月4日10:48:12

上午看了视频, 感觉还是挺虚的。决定调整一下策略， 直接开始实验， 在实验时有不懂的再去看慕课补知识点比较好。



### 2020年7月4日12:26:34

回顾了一下lab0, 虽然指导书中有一些操作都没有学过。

接下要要做的是定义print和println宏。（现在又可以回去看rust语法了 逃



下午把lab0做完, 然后学习一下怎么在linux环境下用git等东西。砍柴不误磨刀工。



### 2020年7月4日20:01:15

打算明天休息一天, 做一下学校课程里的作业, 以及上一节外教课.

从26号到现在已经9天了。这9天过了一遍rust语法， 现在再看代码大概能知道代码的意思， 但是如果自己写的话， 估计就不是很熟练。 对于一些语法还不是很熟悉。 

然后今天再过了一遍lab0， 感觉自己需要看的还有很多很多。 

自己就感觉要在这几天里看完操作系统和体系结构，就好累。就算是大概的了解一下，但是就搞得自己很焦虑。



## 2020年7月6日08:37:10

休息了一天, 今天准备再回顾一下rust语言, 晚上做一下lab1。



目标：

:ballot_box_with_check:rust编程之道第9,10,13章



## 2020年7月7日09:21:35

昨天效率一点都不高, 学校有一整天的课, 我没听; 但是os感觉自己也没有好好学. :cry:







关于这个问题:

![image-20200707094603400](../images/readme/image-20200707094603400.png)

scause和stval传给a1和a2只是为了传递参数, 并没有保存到栈中。

再者，中断处理完成之后也没有必要恢复这个中断产生的原因是什么了。





## 2020年7月7日10:18:30

lab1结束. 现在自己实现一下中断部分。

可以注意一下`timer.rs`中全局变量的声明和使用

```rust
pub static mut TICKS: usize = 0;
```



2020年7月7日11:49:43

自己实现完成, 其中有一些报错还是自己没有熟练掌握模块管理的原因.

现在趁热打铁, 再去看看编程之道.



### 2020年7月7日17:28:03

今天效率还可以, 下午学习了清华学堂在线慕课中一些os的基础知识.

[os笔记](https://www.notion.so/lincyawer/0dd3231f26a9420d89e642339a5abaf4)





## 2020年7月8日11:33:35

## 总结

单纯Rust语言上考虑。
 我们在不同情况下解释`&`的意思：

1. 在表达式上，表示的是借用。
2. 在变量绑定上，表示解地址操作与*类似。
3. 在类型声明上，表示引用类型。
4. 在模式匹配上，**无效关键字**

那么`ref`的通用解释是：

1. 在表达式上，**无效关键字**。
2. 在变量绑定上，表示引用类型。
3. 在类型声明上，**无效关键字**。
4. 在模式匹配上，表示引用类型。

非要给区分`ref`和`&`到底哪个是引用，哪个是借用。我们可以先从词性划分，引用我归类为名词，而借用归类为动词。`&A`在表达式上 表示借用A，这是一个动作，那结果就是产出一个引用类型。所以`let ref B`表示声明了一个引用类型，它只能绑定到某次借用动作上。

**所以`ref` 更适合叫引用， `&`叫借用。**

```rust
#![feature(core_intrinsics)]

fn main() {
    let x = &false;
    print_type_name_of(x);

    let &x = &false;
    print_type_name_of(x);

    let ref x = &false;
    print_type_name_of(x);
}

fn print_type_name_of<T>(_: T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() })
}
```

```
&bool
bool
&&bool
```





### 2020年7月8日16:30:54

由于看到助教们还在修改实验内容, 以及考虑到实验的形式汇编, 再加上自己实现的那个代码有好多报错， 打算过几天再看实验内容。

现在再复习一下多线程的内容。





for in 迭代默认是用的into_iter方法, 是移动(move)

还有`iter()`和`iter_mut()`(允许集合被就地修改)







如下为选择 `Box<T>`，`Rc<T>` 或 `RefCell<T>` 的理由：

- `Rc<T>` 允许相同数据有多个所有者；`Box<T>` 和 `RefCell<T>` 有单一所有者。
- `Box<T>` 允许在编译时执行不可变或可变借用检查；`Rc<T>`仅允许在编译时执行不可变借用检查；`RefCell<T>` 允许在运行时执行不可变或可变借用检查。
- 因为 `RefCell<T>` 允许在运行时执行可变借用检查，所以我们可以在即便 `RefCell<T>` 自身是不可变的情况下修改其内部的值。

在不可变值内部改变值就是 **内部可变性** 模式。让我们看看何时内部可变性是有用的，并讨论这是如何成为可能的。





### 明日规划

[refcell](https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html)

[引用循环与内存l泄露](https://kaisery.github.io/trpl-zh-cn/ch15-06-reference-cycles.html)

[可拓展的并发](https://kaisery.github.io/trpl-zh-cn/ch16-04-extensible-concurrency-sync-and-send.html)

unsafe Rust

清华的os课

看riscv资料







## 2020年7月9日16:04:18

逆变、协变、不变  晕了  这部分等到时候用到了再看看吧



有个问题: 

```
rust中只要定义了传入或者传出变量是引用的, 就会报错吗
```

