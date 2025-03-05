trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("🐶汪汪！");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("🐱喵喵！");
    }
}
/*
动态分发:
运行时决定调用逻辑：通过虚函数表（VTable）在运行时查找具体方法。
​语法关键字：通过 Box<dyn Trait> 或 trait 对象实现。
​灵活性：类型在运行时才能确定（例如存储在集合或动态分配的容器中）。

​​ trait 对象：Box<dyn Animal> 包含一个指向虚函数表的指针，每个具体类型（如 Dog、Cat）的实例都有一个对应的 VTable。
​动态开销：每次调用 animal.speak() 需要通过虚函数表跳转，存在间接访问的开销。
​灵活性：可以在运行时动态添加新类型（例如从文件加载插件），无需修改原有代码。
*/
fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)]; // animals 被包装为 trait 对象

    for animal in animals {
        animal.speak(); // 运行时通过虚函数表调用具体方法
    }
}
