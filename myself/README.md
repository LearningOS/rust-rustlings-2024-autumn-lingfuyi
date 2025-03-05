# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
总的来说，泛型是多种类型的通用api，而traits则是一种抽象的接口，它定义了一些方法，这些方法可以被不同的数据类型实现。traits可以被用于泛型编程，可以让代码更加灵活，更加易于理解和维护。
泛型是宏观标识，而traits是微观实现。traits也可以使用默认方法来提供默认实现，这样可以简化代码，重新实现会重载这个方法

对比与适用场景
​特性​				​​静态多态​					   ​动态多态​
​确定类型时间​			编译时						运行时
​性能​				无额外开销（零成本抽象）			有虚函数表跳转的间接开销
​代码复用​			通过泛型约束共享逻辑	通过 trait 		对象实现多态接口
​适用场景​			已知类型的集合（如函数参数、数组）		类型未知的集合（如容器存储不同类型）
​4. 总结
​静态多态：适用于类型固定且已知的多态场景，追求极致性能（如游戏引擎、高性能计算）。
​动态多态：适用于类型动态变化或多态接口需要灵活扩展的场景（如 GUI 框架、插件系统）。