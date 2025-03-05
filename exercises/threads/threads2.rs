// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 定义一个结构体用于跟踪完成的工作数量
struct JobStatus {
    jobs_completed: u32,
}

// 主函数，创建并管理多个线程来更新共享的工作状态
fn main() {
    // 创建一个Arc智能指针包裹Mutex，用于在线程间安全地共享和更新JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    // 创建10个线程，每个线程将完成一项工作
    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // 克隆Arc（共享所有权）
                                                 // 每个线程在创建时都会移动一个status_shared的副本到闭包中
        let handle = thread::spawn(move || {
            // 移动闭包捕获status_shared
            thread::sleep(Duration::from_millis(250)); // 模拟耗时操作
                                                       // TODO: You must take an action before you update a shared value
                                                       // 锁定Mutex以安全地更新共享的JobStatus
            let mut counter = status_shared.lock().unwrap();
            counter.jobs_completed += 1;
        });
        handles.push(handle);
    }
    // 等待所有线程完成工作
    for handle in handles {
        // 等待所有线程完成
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // 打印当前已完成的工作数量
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
