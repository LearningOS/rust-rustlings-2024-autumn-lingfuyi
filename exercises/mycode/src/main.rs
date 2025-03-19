//cargo test -- --nocapture  执行所有的测试用例，并打印出所有输出

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, sync_channel};
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;
struct Book {
    title: String,
    weight: u32,
}
use mycode::make_answer;
make_answer!();

fn print_book_info(b: &mut Book) {
    let weight = b.weight;
    //let title = b.title; 由于string实现了move语义，所以不能再使用b.title，但是weight可以,因为weight是u32类型，所以可以直接使用
    println!("Title: {}, Weight: {}", &b.title, weight);
}

fn main() {
    println!("这是一个测试程序，可以使用cargo test -- --nocapture 命令来执行所有的测试用例，并打印出所有输出");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] //测试可变引用
    fn test_mutable_reference_1() {
        let mut book = Book {
            title: "The Rust Programming Language".to_string(),
            weight: 1000,
        };
        print_book_info(&mut book);
    }
    #[test] //测试过程宏
    fn test_macro_2() {
        println!("answer: {}", answer());
    }
    #[test] //测试arc无锁编程,自旋锁
    fn test_arc_spinlock_3() {
        let spinlock = Arc::new(AtomicUsize::new(1));
        let spinlock_clone = spinlock.clone();
        let thread = thread::spawn(move || {
            /*
            统计计数器（只增不减）	Relaxed	无需顺序约束，追求高性能
            锁的释放	Release	确保锁内修改对其他线程可见
            锁的获取	Acquire	确保锁内读取看到最新的修改
            原子交换	AcqRel	同时保证读和写的顺序性
            强一致性需求	SeqCst	确保所有线程操作顺序完全一致，性能较差
                */
            //对于锁的属性选择SeqCst，保证所有线程操作顺序完全一致，性能较差，这里使用SeqCst
            spinlock_clone.store(0, Ordering::SeqCst);
            // critical section
        });
        while spinlock.load(Ordering::SeqCst) != 0 {}
        if let Err(panic) = thread.join() {
            println!("Thread panicked: {:?}", panic);
        }
    }
    #[test] //测试内存屏障，barrier
    fn test_barrier_4() {
        let mut handles = Vec::with_capacity(10);
        let barrier = Arc::new(Barrier::new(10));
        for _ in 0..10 {
            let c = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before wait"); //等待所有线程到达barrier，这里会输出10次打印信息
                c.wait(); //直到所有线程都到达barrier后，才继续执行后续代码
                println!("after wait");
                // critical section
            }));
        }
        for handle in handles {
            if let Err(panic) = handle.join() {
                println!("Thread panicked: {:?}", panic);
            }
        }
    }
    #[test] //测试条件变量condvar
            //条件变量代表阻止线程的能力，使其在等待事件发生时不消耗CPU时间，而是被唤醒并继续执行。条件变量是通过锁和通知机制实现的。
    fn test_condvar_5() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();
        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            //通知等待在条件变量上的至少一个线程
            cvar.notify_one();
        });
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        //等待条件变量通知以及started变量被修改为true
        while !*started {
            //释放互斥锁，并将当前线程置于等待状态，直到条件变量被通知。
            started = cvar.wait(started).unwrap();
        }
        // critical section
    }
    #[test] //测试mpsc同步通道
    fn test_mpsc_channels_6() {
        //创建一个普通通道，非阻塞
        let (tx, rx) = channel();
        //创建一个同步通道，最大缓冲区为3，阻塞
        let (sync_tx, sync_rx) = sync_channel(3);
        //启动一个线程作为生产者，发送消息到普通通道
        let sender = thread::spawn(move || {
            let values = vec![
                String::from("普通消息1"),
                String::from("普通消息2"),
                String::from("普通消息3"),
            ];
            for val in values {
                //依次发送消息到普通通道并且睡眠1秒
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        //启动另一个线程作为生产者，发送消息到同步通道
        let sync_sender = thread::spawn(move || {
            let values = vec![
                String::from("同步消息1"),
                String::from("同步消息2"),
                String::from("同步消息3"),
            ];
            for val in values {
                //依次发送消息到同步通道并且睡眠1秒
                sync_tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        //消费者线程，接收消息并打印
        let reciver = thread::spawn(move || {
            let mut recived: Vec<String> = Vec::new();
            //接收普通消息
            while let Ok(msg) = rx.recv() {
                //实现了Display trait，所以可以直接打印
                println!("接收到普通消息：{}", msg);
                recived.push(msg);

                if recived.len() == 3 {
                    break;
                }
            }
            //接收同步消息
            while let Ok(msg) = sync_rx.recv() {
                println!("接收到同步消息：{}", msg);
                recived.push(msg);

                if recived.len() == 6 {
                    break;
                }
            }
            recived
        });
        //等待线程结束
        sender.join().unwrap();
        sync_sender.join().unwrap();
        reciver.join().unwrap();
    }
    #[test] //测试mutex互斥锁
    #[ignore] // 标记为跳过测试，因为测试用例会导致死锁
    fn test_mutex_7() {
        let m = Arc::new(Mutex::new(5));
        let m2 = m.clone();
        let handle = thread::spawn(move || {
            let mut num = m2.lock().unwrap();
            *num += 1;
            println!("thread num: {}", *num);
        });
        let mut num = m.lock().unwrap();
        *num += 1;
        println!("main num: {}", *num);
        handle.join().unwrap();
        assert_eq!(*num, 7);
    }
    #[test] //测试rwlock读写锁
    #[ignore] // 标记为跳过测试，因为测试用例会导致死锁
    fn test_rwlock_8() {
        use std::sync::RwLock;
        let lock = Arc::new(RwLock::new(5));
        let lock2 = lock.clone();
        let handle = thread::spawn(move || {
            //获取写锁
            let mut num = lock2.write().unwrap();
            *num += 1;
            println!("thread num: {}", *num);
        });
        //获取写锁
        let mut num = lock.write().unwrap();
        *num += 1;
        println!("main num: {}", *num);
        handle.join().unwrap();
        assert_eq!(*num, 7);
    }
    #[test] //测试once惰性初始化
    fn test_once_9() {
        use std::sync::Once;
        //声明一个静态变量，类型为u32，初始值为0，生命周期为整个程序
        static mut COUNTER: u32 = 0;
        //定义一个once实例,永续保证后面代码只执行一次
        static INIT: Once = Once::new();
        //初始化函数，将COUNTER设置为10
        INIT.call_once(|| unsafe {
            COUNTER = 10;
        });
        assert_eq!(unsafe { COUNTER }, 10);
    }
}
