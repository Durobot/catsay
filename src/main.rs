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
//
// Use NO_COLOR=1 cargo run to suppress color output.
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
    dead: bool,

    // • Short / long CL argument names don't have to be the same as
    //   the struct field name.
    // • parse(from_os_str) defines a custom parser scheme. By default
    //   StructOpt will use the from_str scheme, which uses the function signature fn(&str) -> T.
    //   But in this case, we are passing a string of the path name, which might be
    //   represented differently in different operating systems.
    //   (see https://doc.rust-lang.org/std/ffi/struct.OsString.html for why this is necessary)
    //   Therefore, we need to parse from an &OsStr instead.
    // • The type defined for catfile is wrapped in an Option<T>. This is how you indicate
    //   that this field is optional. If the field is not provided, it will simply be Option::None.
    //   There are other options, like Vec<T>, that represent a list of arguments, and u64 indicates
    //   that you want to count the occurrences of a parameter. For example , -v, -vv, and -vvv are
    //   commonly used to set the verbosity level.
    // • Inside the Option, we use a std::path::PathBuf instead of a raw string.
    //   PathBuf can help you handle paths to files more robustly because it hides many
    //   differences in how the operating systems represent paths.
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

// Trying to returning Result<(), std::error::Error> produces the following error:
// the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time 
// dyn std::error::Error is a trait object, meaning "any type that implements std::error::Error trait
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let options = Options::from_args();
    let message = options.message;
    if message.to_lowercase() == "woof"
    { eprintln!("A cat should not bark like a dog!"); } // print to stderr

    println!("{}", message.bright_yellow().underline().on_purple());
    let eye = if options.dead { "x".red() } else { "o".bright_green() };
    match &options.catfile
    {
        Some(path) =>
        {
            let cat_template = std::fs::read_to_string(path)?;
            //    .expect(&format!("could not read file {:?}", path));
            // Can’t use format!() to replace the eyes with o or x.
            // format!() needs to know the formatting string at compile time,
            // but the catfile string is loaded at runtime.
            let cat_picture = cat_template.replace("{eye}", &eye);
            // Or use a library like strfmt to have more format!-flavored code.
            println!("{}", &cat_picture);
        },
        None =>
        {
            println!(" \\");
            println!("  \\");
            println!("   /\\_/\\");
            println!("  ( {e} {e} )", e = eye);
            println!("  =( I )=");
        }
    }

    Ok(())
}
