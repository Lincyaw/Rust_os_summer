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

#[macro_use]
extern crate lazy_static;


use process::*;

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
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    memory::init();
    interrupt::init();

    // // 新建一个带有内核映射的进程。需要执行的代码就在内核中
    // let process = Process::new_kernel().unwrap();

    // for message in 0..8 {
    //     let thread = Thread::new(
    //         process.clone(),            // 使用同一个进程
    //         sample_process as usize,    // 入口函数
    //         Some(&[message]),           // 参数
    //     ).unwrap();
    //     PROCESSOR.get().add_thread(thread);
    // }

    // 把多余的 process 引用丢弃掉
    drop(process);

    PROCESSOR.get().run();



    {
        let mut processor = PROCESSOR.get();
        let kernel_process = Process::new_kernel().unwrap();
        for i in 1..33usize {
            processor.add_thread(create_kernel_thread(
                kernel_process.clone(),
                test_kernel_thread as usize,
                Some(&[i]),
            ));
        }
    }

    unsafe { PROCESSOR.unsafe_get().run() }

}

fn sample_process(message: usize) {
    for i in 0..500000 {
        if i % 200000 == 0 {
            println!("thread {} {}", message,i);
        }
    }
}

/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.get().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}

/// 创建一个内核进程
pub fn create_kernel_thread(
    process: Arc<RwLock<Process>>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    // 创建线程
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    // 设置线程的返回地址为 kernel_thread_exit
    thread
        .as_ref()
        .inner()
        .context
        .as_mut()
        .unwrap()
        .set_ra(kernel_thread_exit as usize);

    thread
}