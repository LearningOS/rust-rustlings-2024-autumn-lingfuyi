// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
/*使用 impl SomeTrait + OtherTrait 表示参数类型是同时实现了 SomeTrait 和 OtherTrait 的具体类型。
静态分发：
编译时确定具体类型，调用方法通过静态分发（直接跳转到具体类型的实现），效率更高。
类型推断：
当调用 some_func(SomeStruct {}) 或 some_func(OtherStruct {}) 时，Rust 编译器会自动推断出 item 的具体类型（SomeStruct 或 OtherStruct），并验证它们是否满足 SomeTrait + OtherTrait 约束。
*/
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
