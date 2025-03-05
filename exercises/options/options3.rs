// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    /*
    问题原因：Rust 所有权机制要求明确变量的生命周期。当通过值匹配（如 match y）时，变量会被“消耗”（所有权转移），后续不可再使用。
    解决方案：若需保留原变量的有效性，可通过引用（&y）或克隆（y.clone()）来避免所有权转移
        */
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
