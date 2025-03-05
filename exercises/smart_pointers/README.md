# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

类型	所有权语义			克隆策略		典型场景
&T	不可变借用			无克隆			临时不可变访问
Cow<T>	延迟克隆的混合所有权		按需克隆		共享数据的高效修改
Rc<T>	多个不可变所有者		克隆指针而非数据	共享不可变数据
Arc<T>	线程安全的多个不可变所有者	原子克隆指针		跨线程共享数据