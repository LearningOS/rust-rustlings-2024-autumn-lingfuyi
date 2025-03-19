# cell的核心特性
1.内部可变性：通过编译时保证安全，而非运行时锁
2.零成本抽象：无锁设计，性能接近直接操作原生数据
3.单线程限制：仅适用于单线程环境，跨线程需要使用mutex或者rwlock等同步机制
## 基本用法
1、创建cell:
```rust
use std::cell::Cell;
let share_state = Cell:new(100); //初始值为100的cell
```
2、常用方法：
| 方法           | 作用                             | 示例                                   |
| -------------- | -------------------------------- | -------------------------------------- |
| get()          | 获取不可变引用                   | let value = shared_state.get();        |
| set(value)     | 替换整个值                       | shared_state.set(100);                 |
| replace(value) | 替换值并返回旧值                 | let old = shared_state.replace(200);   |
| take()         | 移除并返回值，原位置留空（None） | let val = shared_state.take();         |
| swap(value)    | 用新值替换旧值并返回旧值         | let old = shared_state.swap(300);      |
| as_ptr()       | 获取指向内部数据的原始指针       | let ptr = shared_state.as_ptr();       |
| from_mut(ptr)  | 将 *mut T 转换为 &mut Cell<T>    | unsafe { Cell::from_mut(ptr) }         |
| into_inner()   | 移除内部值并返回它               | let value = shared_state.into_inner(); |
## 典型使用场景
1、场景1：全局状态管理
```rust
use std::cell::Cell;
ust std::rc:Rc;
struct Counter {
	count:Cell<u32>,
}
impl Counter {
	fn new()->Rc<self> {
		Rc::new(Cell::new(0))
	}
	fn increment(&self) {
		self.count.set(self.count.get() + 1);
	}
	fn get_count(&self) -> u32 {
		self.count.get()
	}
}
fn main() {
	let counter = Counter::new();
	counter.increment();
	assert_eq!(counter.get_count(), 1); //输出count的值为1
}
```
2、场景2：闭包中可变状态
```rust
use std::cell::Cell;
fn main() {
	let x = Cell::new(10);
	let closure = ||{
		x.set(x.get() + 1);
		x.get()
	};
	println!("closure result: {}", closure()); //输出closure result: 11
	println!("closure result: {}", closure()); //输出closure result: 12
}
```
3、异步任务状态共享
```rust
use std::cell::Cell;
use tokie::task;
async fn async_task(state:Cell<i32>){
	state.set(state.get() + 1);
}
#[tokio::main]
async fn main() {
	let state = Cell::new(0);
	task::spawn(async move {
		async_task(state).await;
	}).await.unwrap();
	println!("state: {}", state.get()); //输出state: 1
	}
```
## 与refcell区别
| 特性       | Cell                 | RefCell                    |
| ---------- | -------------------- | -------------------------- |
| 值类型要求 | 无                   | 无                         |
| 运行时检查 | 无（编译时保证安全） | 运行时 borrow 检查         |
| 性能       | 高（零成本）         | 低（动态检查）             |
| 适用场景   | 单线程、高性能需求   | 需要动态借用检查的复杂逻辑 |
## 总结
Cell和RefCell都是Rust提供的零成本抽象，用于在单线程环境下管理不可变状态。Cell和RefCell的区别在于运行时检查，Cell无运行时检查，RefCell有运行时检查。Cell适用于简单的状态管理，RefCell适用于复杂的状态管理。
Cell<T> 是 Rust 中最轻量级的内部可变性工具，适用于以下场景：

1、​单线程环境下的共享可变状态。
​2、性能敏感的场景（如高频计数器、缓存）。
​3、闭包或异步任务中需要修改外部变量。
​4、全局状态或单例模式的管理。
如果需要在多线程间共享数据，或需要更复杂的借用规则，应改用 Mutex、RwLock 或 RefCell