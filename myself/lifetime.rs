#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

/*总结下：&mut self 借用的生命周期和 loan 的生命周期相同，将持续到 println 结束。
而在此期间 foo.share() 又进行了一次不可变 &foo 借用，违背了可变借用与不可变借用不能同时存在的规则，最终导致了编译错误*/
fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    //foo.share(); // 违背了不可变借用不能同时存在的规则
    println!("{:?}", loan);
}
