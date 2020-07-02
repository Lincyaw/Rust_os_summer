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

![image-20200630163414065](../../source/images/readme/image-20200630163414065.png)

回到rust上，感觉rust未来可期，现在缺的只是时间积累，再过几年有了积累之后比其他语言会有很大的优势。

![image-20200630163707029](../../source/images/readme/image-20200630163707029.png)





## 2020年7月1日10:57:54

```
cd /mnt/c/users/fay/documents/github/rust_os_summer/labproject/os
```





今天看了看实验的内容, 顺便配置了一下wsl环境, 为以后实验做准备



### 2020年7月1日11:27:32

在`lab0`执行`make run`时出现如下问题, 不知道为啥

![image-20200701112600657](../../source/images/readme/image-20200701112600657.png)







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