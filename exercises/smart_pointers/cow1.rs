// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

//Cow是Clone-On-Write的缩写，属于智能指针的一种。它的设计目的是在需要修改数据时才进行克隆，
//避免不必要的复制。这在处理大块数据或者共享数据时特别有用，可以提高性能。
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    输入的是由切片创建的Cow，即Borrowed状态。
    因为在这个函数中进行修改，所以必须进行克隆，变成Owned状态。所以期望结果是Cow::Owned，这部分应该是对的。
    */
    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
    /*
    输入的切片都是非负数，所以不需要修改。
    这时候abs_all函数不会进行任何克隆，返回的仍然是Borrowed状态的Cow。因此这里应该匹配Cow::Borrowed。
    原来的代码里用的是match的结果是否等于Cow::Owned，如果不匹配就报错，所以需要把错误的分支改成Cow::Borrowed才对。
    */
    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()), // ✅ 修改点：预期未克隆
            _ => Err("Expected borrowed value"),
        }
    }
    /*
    输入的是直接由向量创建的Cow，也就是Owned状态。
    此时即使没有修改，函数返回的还是Owned，因为Cow在创建时已经是拥有的状态，
    不会因为不修改而变成Borrowed。所以这里应该检查是否是Cow::Owned。
    */
    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // ✅ 修改点：Cow始终持有所有权
            _ => Err("Expected owned value"),
        }
    }
    /*
    输入的向量包含负数，需要进行修改。
    这时调用to_mut()会检查是否已经拥有数据，如果是的话就可以直接修改，
    无需克隆。因此结果仍然是Cow::Owned。所以这里也应该匹配Cow::Owned。
    */
    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // ✅ 修改点：修改操作不会改变所有权状态
            _ => Err("Expected owned value"),
        }
    }
}
