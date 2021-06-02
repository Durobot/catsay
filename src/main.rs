//extern crate structopt;

use structopt::StructOpt;

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

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("   /\\_/\\");
    if options.dead
    { println!("  ( x x )"); }
    else
    { println!("  ( o o )"); }
    println!("  =( I )=");
}
