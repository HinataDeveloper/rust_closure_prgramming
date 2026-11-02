// lib.rs
// Date: Wed Oct 28 2026
// Educational and Practice Rust Programming Language Code

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

use std::fmt::Display;

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[width: {}, height:{}, area: {}, perimeter:{}]",
            self.width(),
            self.height(),
            self.area(),
            self.perimeter()
        )
    }
}

pub fn separator() {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}
