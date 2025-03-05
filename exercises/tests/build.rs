//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::SystemTime;
fn main() {
    // 设置环境变量 TEST_FOO
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // Move `as_secs()` here to fix the issue
                    // let your_command = format!(
                    //     "Your command here with {}, please checkout exercises/tests/build.rs",
                    //     timestamp
                    // );
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
