// extern crate proc_macro; // 2018 版本之后不需要这行代码
use proc_macro::TokenStream;

/// 函数式：`make_answer!()`
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
