use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Mutex,Arc};
fn main() {

}
#[test]
fn move_before_borrow(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //println!("{:?}",v); v的所有权已经被handle线程抢走
    handle.join().unwrap();
}

#[test]
fn thread_mpsc(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { //move 可以转移变量的所有权, 在该线程里中用的外部变量的所有权都转移到该线程中
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);


}

#[test]
fn one_tx_one_rx(){
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
}

#[test]
fn multi_tx_one_rx(){
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx); //这里克隆了发送端, 可以使两个线程都发送
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
// 在某个会议的一次小组座谈会中，只有一个麦克风。如果一位成员要发言，他必须请求或表示希望使用麦克风。
// 一旦得到了麦克风，他可以畅所欲言，然后将麦克风交给下一位希望讲话的成员。
// 如果一位成员结束发言后忘记将麦克风交还，其他人将无法发言。
// 如果对共享麦克风的管理出现了问题，座谈会将无法如期进行！
#[test]
fn single_mutex(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

//会报错
//错误信息表明 counter 值在上一次循环中被移动了。所以 Rust 告诉我们不能将 counter 锁的所有权移动到多个线程中。
#[test]
fn mutex_between_threads_fail(){
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());
}


#[test]
fn mutex_between_threads_success() {
    let counter = Arc::new(Mutex::new(0));  //将mutex这个互斥器封装进Arc(引用计数智能指针  Rc智能指针不能用于多线程
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
        // for h in handles.iter()  {
        //     println!("{:?}",h);
        // }
        println!("{:?}",handles);
    }

    // for h in handles.iter(){
    //     println!("{:?}",h);
    // }
    for handle in handles{ //for in 默认是into
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}