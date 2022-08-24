// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Introductory example to test whether the library is set up / working properly and compiles.
//!
//! cargo run --example getting_started

use identity_iota::client::Result;

#[tokio::main]
async fn main() -> Result<()> {
  // Print IOTA Identity header
  println!();
  println!(r#"  _____ ____ _______          _____    _            _   _ _         "#);
  println!(r#" |_   _/ __ \__   __|/\      |_   _|  | |          | | (_) |        "#);
  println!(r#"   | || |  | | | |  /  \       | |  __| | ___ _ __ | |_ _| |_ _   _ "#);
  println!(r#"   | || |  | | | | / /\ \      | | / _` |/ _ \ '_ \| __| | __| | | |"#);
  println!(r#"  _| || |__| | | |/ ____ \    _| || (_| |  __/ | | | |_| | |_| |_| |"#);
  println!(r#" |_____\____/  |_/_/    \_\  |_____\__,_|\___|_| |_|\__|_|\__|\__, |"#);
  println!(r#"                                                               __/ |"#);
  println!(r#"                                                              |___/ "#);

  // Print welcome text
  println!();
  println!("Welcome to IOTA Identity! The library was set up correctly and everything works!");
  println!("You can try out the other examples using: cargo run --example <example_name>");
  println!("Please visit https://github.com/iotaledger/identity.rs/tree/main/examples for further info!");
  println!();

  Ok(())
}
