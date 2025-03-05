trait Animal {
    fn speak(&self);
}

fn make_animal_speak<T: Animal>(animal: &T) {
    animal.speak();
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
静态分发
​编译期决定调用逻辑：编译器根据泛型类型的具体信息生成对应的代码。
​零运行时开销：没有虚函数表或类型判断的开销。
​语法关键字：通过泛型约束 T: Trait 实现。

泛型函数：make_animal_speak<T: Animal> 在编译时会为每个具体的 T（如 Dog、Cat）生成不同的函数实例。
​性能：调用 animal.speak() 直接跳转到具体类型的实现，无任何间接开销。
​限制：类型必须在编译时已知（例如函数参数的类型必须明确）。
*/

fn main() {
    let dog = Dog;
    let cat = Cat;

    make_animal_speak(&dog); // 编译时确定调用 Dog::speak()
    make_animal_speak(&cat); // 编译时确定调用 Cat::speak()
}
