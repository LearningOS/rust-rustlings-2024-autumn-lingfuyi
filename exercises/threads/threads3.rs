// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    // 创建一个新的 Queue 实例，包含长度为10，分为两半的向量
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 启动两个线程，分别从 Queue 的两个半部分发送数据到通道的发送端
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = qc.clone();
    let qc2 = qc.clone();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // 创建一个通道，用于线程间通信
    let (tx, rx) = mpsc::channel();
    // 创建一个新的 Queue 实例
    let queue = Queue::new();
    // 获取 Queue 的长度
    let queue_length = queue.length;

    // 调用 send_tx 函数，启动两个线程发送数据
    send_tx(queue, tx);

    // 接收来自通道的数据，并计算接收到的数据总数
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 打印接收到的总数，并断言总数等于 Queue 的长度
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}