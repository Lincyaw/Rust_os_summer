//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//!
//! 
//! 
//! 
//! - `#![deny(missing_docs)]`  
//!   任何没有注释的地方都会产生警告：这个属性用来压榨写实验指导的学长，同学可以删掉了
#![warn(missing_docs)]
#![feature(llvm_asm)]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(asm)]`  
//!   内嵌汇编
#![feature(asm)]
//!
//! - `#![feature(global_asm)]`  
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(slice_fill)]
#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;
mod process;
mod drivers;
mod fs;
mod kernel;
#[macro_use]
extern crate lazy_static;

use crate::memory::*;
use process::*;
use fs::*;
extern crate alloc;


// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("asm/entry.asm"));

/// Rust 的入口函数
/// 
/// 
///
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();

    let process = Process::new_kernel().unwrap();

    PROCESSOR
        .get()
        .add_thread(Thread::new(process.clone(), simple as usize, Some(&[0])).unwrap());

    // 把多余的 process 引用丢弃掉
    drop(process);

    PROCESSOR.get().run()
}

/// 测试任何内核线程都可以操作文件系统和驱动
fn simple(id: usize) {
    println!("hello from thread id {}", id);
    // 新建一个目录
    fs::ROOT_INODE
        .create("tmp", rcore_fs::vfs::FileType::Dir, 0o666)
        .expect("failed to mkdir /tmp");
    // 输出根文件目录内容
    fs::ls("/");

    loop {}
}