# Threads

In most current operating systems, an executed program’s code is run in a process, and the operating system manages multiple processes at once.
Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

## Further information

- [Dining Philosophers example](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html)
- [Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
特性	Mutex实现	原子操作实现
易用性	更直观，无需处理内存顺序	需要理解Ordering语义
性能	较低（系统调用开销）	更高
适用场景	通用型共享状态管理	低开销计数器/标志位等简单操作

Mutex注意事项
死锁风险: 如果某个线程永远不释放锁（如未处理panic），会导致其他线程永久阻塞
性能考量: 在高频更新场景下，原子操作明显优于Mutex
错误处理: lock()返回的Result需要妥善处理（示例中使用unwrap()简化代码）