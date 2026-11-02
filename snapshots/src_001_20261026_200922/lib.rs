// lib.rs
// Date: Mon Oct 26 2026
// Educational and Practice Rust Programming Language Code

// Dependency: Without any dependency

// rustc 1.101.0-nightly (7189d39d9 2026-10-22)
// binary: rustc
// commit-hash: 7189d39d9ce936881b8132c6addeaf3e295cd455
// commit-date: 2026-10-22
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.4

// cargo 1.101.0-nightly (15be64aa0 2026-10-20)
// release: 1.101.0-nightly
// commit-hash: 15be64aa0859b5c36f7a86d52f7aa0db44ba4d84
// commit-date: 2026-10-20
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.11-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ShirtColor {
    Blue,
    Red,
}

#[derive(Default)]
pub struct Inventory {
    shirt: Vec<ShirtColor>,
}

impl Inventory {
    #[must_use]
    pub const fn new() -> Self {
        Self { shirt: Vec::new() }
    }

    pub fn add_shirt(&mut self, shirt: ShirtColor) {
        self.shirt.push(shirt);
    }

    #[must_use]
    pub fn shirts(&self) -> &[ShirtColor] {
        &self.shirt
    }

    #[must_use]
    pub fn must_stocked(&self) -> ShirtColor {
        let mut most_red: u32 = 0;
        let mut most_blue: u32 = 0;

        for item in self.shirts() {
            match item {
                ShirtColor::Blue => most_blue += 1,
                ShirtColor::Red => most_red += 1,
            }
        }

        if most_blue > most_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }

    #[must_use]
    pub fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.must_stocked())
    }
}
