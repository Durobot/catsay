use std::process::Command; // Run programs
// https://docs.rs/assert_cmd/1.0.5/assert_cmd/index.html
// Assert Command - Easy command initialization and assertions.
// assert_cmd aims to simplify the process for doing integration testing of CLIs, including:
//    Finding your crate’s binary to test
//    Assert on the result of your program’s run.
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>>
{
    Command::cargo_bin("catsay") // Create a Command to run a specific binary of the current crate
        .expect("binary doesn't exist") // as cargo_bin() returns Result<Self, CargoError>, we need to unwrap to get that Command
        .assert() // pub fn assert(&mut self) -> Assert : run a Command and make assertions on the struct std::process::Output
        .success(); // Struct assert_cmd::assert::Assert : pub fn success(self) -> Self : Ensure the command succeeded
    Ok(())
}
