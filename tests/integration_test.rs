use std::process::Command; // Run programs
// https://docs.rs/assert_cmd/1.0.5/assert_cmd/index.html
// Assert Command - Easy command initialization and assertions.
// assert_cmd aims to simplify the process for doing integration testing of CLIs, including:
//    Finding your crate’s binary to test
//    Assert on the result of your program’s run.
use assert_cmd::prelude::*; // Add methods on commands
// Crate predicates
// This library implements an interface to "predicates" - boolean-valued functions of one argument.
// This allows combinatorial logic to be created and assembled at runtime and then used one or more times for evaluating values.
// Module predicates::prelude
// Module that contains the essentials for working with predicates.
use predicates::prelude::*; // For predicate::str::contains("Meow!")

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>>
{
    Command::cargo_bin("catsay") // Create a Command to run a specific binary of the current crate
        .expect("binary doesn't exist") // as cargo_bin() returns Result<Self, CargoError>, we need to unwrap to get that Command
        .assert() // pub fn assert(&mut self) -> Assert : run a Command and make assertions on the struct std::process::Output
        .success() // Struct assert_cmd::assert::Assert : pub fn success(self) -> Self : Ensure the command succeeded
        .stdout(predicate::str::contains("Meow!")); // Function predicates::prelude::predicate::str::contains
        // pub fn contains<P>(pattern: P) -> ContainsPredicate
        //     where P: Into<String>,
        // Creates a new Predicate that ensures a str contains pattern
        // pub struct predicates::str::ContainsPredicate - Predicate that checks for patterns.
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>>
{
    Command::cargo_bin("catsay")
        .expect("binary doesn't exist")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure(); // Struct assert_cmd::assert::Assert : pub fn failure(self) -> Self : Ensure the command failed
    Ok(())
}
