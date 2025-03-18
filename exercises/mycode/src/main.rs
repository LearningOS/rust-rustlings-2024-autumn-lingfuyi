//cargo test -- --nocapture  执行所有的测试用例，并打印出所有输出

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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;

fn main() {
    println!("这是一个测试程序，可以使用cargo test -- --nocapture 命令来执行所有的测试用例，并打印出所有输出");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] //测试可变引用
    fn test_mutable_reference() {
        let mut book = Book {
            title: "The Rust Programming Language".to_string(),
            weight: 1000,
        };
        print_book_info(&mut book);
    }
    #[test] //测试过程宏
    fn test_macro() {
        println!("answer: {}", answer());
    }
    #[test] //测试arc无锁编程,自旋锁
    fn test_arc_spinlock() {
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
    fn test_barrier() {
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
    fn test_condvar() {
        let condvar = Arc::new((Mutex::new(false), Condvar::new()));
        let mut handles = Vec::with_capacity(10);
        for _ in 0..10 {
            let c = condvar.clone();
            handles.push(thread::spawn(move || {
                let mut lock = c.0.lock().unwrap();
                while !*lock {
                    lock = c.1.wait(lock).unwrap();
                }
                // critical section
                *lock = false;
                c.1.notify_all();
            }));
        }
        for handle in handles {
            if let Err(panic) = handle.join() {
                println!("Thread panicked: {:?}", panic);
            }
        }
    }
}
