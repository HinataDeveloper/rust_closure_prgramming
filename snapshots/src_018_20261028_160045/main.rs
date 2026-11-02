// main.rs
// Date: Wed Oct 28 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning Closure Programming
// Goal: ...
// Dependency: Without any dependency

// rustc 1.101.0-nightly (16d1ffac1 2026-10-26)
// binary: rustc
// commit-hash: 16d1ffac17ad8b739e32483ac2e65fe7e1804977
// commit-date: 2026-10-26
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.4

// cargo 1.101.0-nightly (cad07cf8e 2026-10-23)
// release: 1.101.0-nightly
// commit-hash: cad07cf8e597397fff5d0f6cb79c9f870c0b1dd2
// commit-date: 2026-10-23
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.12-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use closure_programming::Rectangle;
use closure_programming::separator;

fn main() {
    println!("\n");

    let mut rectangle_list: [Rectangle; 3] = [
        Rectangle::new(120, 41),
        Rectangle::new(41, 63),
        Rectangle::new(52, 39),
    ];

    let mut sort_operation: Vec<String> = Vec::new();
    let msg: String = String::from("Closure Called");

    separator();

    // Encounter with compile time error.
    // Because sort_operation moves into the closure
    rectangle_list.sort_by_key(|r| {
        sort_operation.push_within_capacity(msg);
        r.width()
    });

    println!("\nThe End ...\n");
}
