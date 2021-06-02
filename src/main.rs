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
    message: String
}

fn main()
{
    let options = Options::from_args();
    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("   /\\_/\\");
    println!("  ( o o )");
    println!("  =( I )=");
}
