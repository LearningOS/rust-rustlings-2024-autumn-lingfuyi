// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
//'a 至少要和 name 及 title 变量的生命周期一样长，以确保在 book 实例存在期间，引用的字符串不会被销毁
fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
