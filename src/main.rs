//extern crate structopt; -- Not needed in Rust 2018

// The library called StructOpt combines clap and custom derive. Custom derive is a
// feature in Rust that automatically generates a default implementation of a
// trait by annotating a struct.
use structopt::StructOpt;
// The colored crate defines a Colorize trait, which is implemented on a
// &str and String. This trait provides various chainable coloring functions:
// • Coloring the text: red(), green(), blue(), etc.
// • Coloring the background: on_red() (i.e., text on red
//   background), on_green(), on_blue(), etc.
// • Brighter version: bright_red(), on_bright_green(),
//   etc.
// • Styling: bold(), underline(), italic(), etc.
use colored::*;

// This custom derive attribute tells structopt to use a macro defined by
// the StructOpt automatically implements the StructOpt trait for the struct.
// This implementation will contain the necessary clap (a command line argument parsing library
// used by structopt "under the hood") code for parsing the arguments.
#[derive(StructOpt)]
struct Options
{
    // The following line is optional. It gives a default value
    // to the 'message' positional argument.
    // The rustdoc comment below it serves as a help string
    // for StructOpt.
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    // Add both short ('-d') and long versions ('--dead') of
    // the following flag (named boolean argument, the value of which
    // is determined by whether it is present or absent).
    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool
}

fn main()
{
    let options = Options::from_args();
    let message = options.message;
    if message.to_lowercase() == "woof"
    { eprintln!("A cat should not bark like a dog!"); } // print to stderr

    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("   /\\_/\\");
    let eye = if options.dead { "x".red() } else { "o".bright_green() };
    println!("  ( {e} {e} )", e = eye);
    println!("  =( I )=");
}
