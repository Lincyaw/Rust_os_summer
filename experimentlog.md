# Lab0

这一章的实验指导中，我将学到：

- 使用 Rust 包管理器 cargo 创建一个 Rust 项目
- 移除 Rust 程序对操作系统的依赖，构建一个独立化可执行的程序
- 我们将程序的目标平台设置为 RISC-V，这样我们的代码将可以在 RISC-V 指令集的裸机（Bare Metal）上执行 Rust 代码
- 生成内核镜像、调整代码的内存布局并在 QEMU 模拟器中启动
- 封装如输出、关机等一些 SBI 的接口，方便后续开发

## Q1

自定义panic(这些操作都没有在rust的教程中看到过.

```
error: `#[panic_handler]` function required, but not found
```

解决方法

```rust
use core::panic::PanicInfo
#[panic_handler]  //把下面定义的panic函数作为panic_handler, 负责在程序发生panic时调用
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

## Q2

错误相关语义项缺失 (用于标记某函数用来实现**堆栈展开**处理功能的语义项

```
error: language item required, but not found: `eh_personality`
```

将 dev 配置和 release 配置的 panic 的处理策略设为直接终止，也就是直接调用我们的 `panic_handler` 而不是先进行堆栈展开等处理再调用。

解决方法: 在cargo.toml中设置panic的处理策略为直接终止

```rust
# panic 时直接终止，因为我们没有实现堆栈展开的功能
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```



## Q3

RISC-V 中，内存的物理地址是从 0x80000000 开始的, 为了保证用户进程不能直接操作内核,保证内核的安全.

所以需要调整内存布局.



## Q4

### 重写程序入口点`_start`

CPU加电后运行在`M mode`(Machine mode), 我们要实现的OS内核运行在`S mode` .

OpenSBI固件做的事是把CPU从`M mode`切换成`S mode` 





## Q5

`console_putchar`的实现

[内联汇编模板]([https://kaisery.gitbooks.io/rust-book-chinese/content/content/Inline%20Assembly%20%E5%86%85%E8%81%94%E6%B1%87%E7%BC%96.html](https://kaisery.gitbooks.io/rust-book-chinese/content/content/Inline Assembly 内联汇编.html))

我们通过 ecall 发起系统调用。OpenSBI 会检查发起的系统调用的编号，如果编号在 0-8 之间，则进行处理，否则交由我们自己的中断处理程序处理.

执行 `ecall` 前需要指定系统调用的编号，传递参数。一般而言，`a7` 为系统调用编号，`a0`、`a1` 和 `a2` 为参数：

```rust
//! 调用 Machine 层的操作
// 目前还不会用到全部的 SBI 调用，暂时允许未使用的变量或函数
#![allow(unused)]

/// SBI 调用
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret) //输出部分，我们将结果保存到变量ret中，限制条件 {x10} 告诉编译器使用寄存器 x10（即 a0 寄存器），前面的 = 表明汇编代码会修改该寄存器并作为最后的返回值。
            
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            //输入部分，我们分别通过寄存器 x10、x11、x12 和 x17（这四个寄存器又名 a0、a1、a2 和 a7） 传入参数 arg0、arg1、arg2 和 which ，其中前三个参数分别代表接口可能所需的三个输入参数，最后一个 which 用来区分我们调用的是哪个接口（SBI Extension ID）。这里之所以提供三个输入参数是为了将所有接口囊括进去，对于某些接口有的输入参数是冗余的，比如 sbi_console_putchar 由于只需一个输入参数，它就只关心寄存器 a0 的值。
            
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile");  // 为了防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}
```

由于`llvm_asm`不稳定, 在编译的时候会报错。因此需要在`main.rs`中添加`#![feature(llvm_asm)]`.(不知道为什么在`sbi.rs`中添加没有效果)





## Q6

[简单的正则表达式介绍](https://www.runoob.com/regexp/regexp-syntax.html)

| 特别字符 | 描述                                                         |
| -------- | ------------------------------------------------------------ |
| $        | 匹配输入字符串的结尾位置。如果设置了 RegExp 对象的 Multiline 属性，则 \$ 也匹配 '\n' 或 '\r'。 |
| ( )      | 标记一个子表达式的开始和结束位置。子表达式可以获取供以后使用。 |
| *        | 匹配前面的子表达式零次或多次。                               |
| +        | 匹配前面的子表达式一次或多次。                               |
| .        | 匹配除换行符 \n 之外的任何单字符。                           |
| [        | 标记一个中括号表达式的开始。                                 |
| ?        | 匹配前面的子表达式零次或一次，或指明一个非贪婪限定符。       |
| ^        | 匹配输入字符串的开始位置，除非在方括号表达式中使用，当该符号在方括号表达式中使用时，表示不接受该方括号表达式中的字符集合。 |
| {        | 标记限定符表达式的开始。                                     |
| \|       | 指明两项之间的一个选择。                                     |
|          |                                                              |





# Lab1

## Q1

```asm
# Context, scause 和 stval 作为参数传入
    mv a0, sp
    csrr a1, scause
    csrr a2, stval
    jal  handle_interrupt

    .globl __restore
# 离开中断
# 从 Context 中恢复所有寄存器，并跳转至 Context 中 sepc 的位置
__restore:
    # 恢复 CSR
    LOAD    s1, 32
    LOAD    s2, 33
    # 思考：为什么不恢复 scause 和 stval？如果不恢复，为什么之前要保存
    csrw    sstatus, s1
    csrw    sepc, s2
```

关于这个问题, scause和stval传给a1和a2只是为了传递参数, 并没有保存到栈中。

再者，中断处理完成之后也没有必要恢复这个中断产生的原因是什么了。





## Q2

因为学习过汇编\计组\电子工艺实习, 所以对中断这块相对于其他几个实验是更加了解的。

代码配上注释基本能够知道为什么要这么做。但是如果要自己实现一下的话，可能还有一定的差距。









## Q3

sp减去了Context所需的空间(34*8), 因为要把Context中的寄存器保存到栈中.

再把sp写入到第二个位置。 在中断过程中可能会使用到栈，所以sp会变化， 但是在中断结束之后， sp就恢复到了中断之前的值



去掉panic之后会顺序执行下面的指令， 但是因为后面没有指令了， 所以会执行内存中的任意指令。



在match中添加一个处理case



实验一内容完成， 保存在wsl中的lab1位置。



整个流程是进入到rustmain函数之后， 初始化中断，开启中断， 然后进入中断，进入interrupt.asm文件,保存现场, 在进入handle_interrupt,  判断出中断类型是断点， 进入断点处理函数， 处理完之后恢复现场, 最后程序跳回sepc 的位置.

