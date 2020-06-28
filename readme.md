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







## 2020年6月28日16:20:53

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