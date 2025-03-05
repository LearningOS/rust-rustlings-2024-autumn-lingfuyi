// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.
/*
生命周期标注的目的
生命周期标注的主要目的是帮助编译器理解引用的有效范围，并确保在任何时候都不会有悬垂引用（指向已经释放的内存的引用）
当您编写一个函数，该函数接受引用作为参数并返回一个引用时，编译器需要知道返回的引用与输入参数中的哪些引用具有相同的生命周期。

为什么longest需要生命周期标注
在longest函数中，您返回了一个字符串切片引用，这个引用必须基于输入参数中的某个引用。如果不标注生命周期，
编译器无法知道返回的引用应该与哪个输入参数的生命周期相关联。通过标注生命周期'a，您告诉编译器：

总结
即使string1和string2是在main函数中定义的，并且它们的生命周期会维持到main结束，longest函数仍然需要生命周期标注。
这是因为编译器需要明确的指示来理解返回的引用与输入参数中的哪些引用具有相同的生命周期，从而确保内存安全。
这是Rust语言设计的一部分，它通过强制开发者显式处理引用的生命周期来避免悬垂引用和其他内存安全问题
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;

    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());

    println!("The longest string is '{}'", result);
}
