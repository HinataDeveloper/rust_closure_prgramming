// main.rs
// Date: Mon Oct 26 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning Closure Programming
// Goal: ...
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

use closure_programming::Inventory;
use closure_programming::ShirtColor::{self, Blue, Red};

fn main() {
    println!("\n");

    let mut inv: Inventory = Inventory::new();

    let shirt_one: ShirtColor = Blue;
    inv.add_shirt(shirt_one);

    let shirt_two: ShirtColor = Red;
    inv.add_shirt(shirt_two);

    let shirt_three: ShirtColor = Blue;
    inv.add_shirt(shirt_three);

    let shirt_four: ShirtColor = Blue;
    inv.add_shirt(shirt_four);

    let shirt_five: ShirtColor = Red;
    inv.add_shirt(shirt_five);

    let shirt_six: ShirtColor = Red;
    inv.add_shirt(shirt_six);

    let user_preference_one = Some(ShirtColor::Blue);
    let up_one = inv.give_away(user_preference_one);

    let user_preference_two = None::<ShirtColor>;
    let up_two = inv.give_away(user_preference_two);

    // Specify blue color t-shirt
    println!("User one: {up_one:#?}");

    // Without specifying color
    println!("User two: {up_two:#?}");

    println!("\nThe End ...\n");
}
