use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "0.0.1",
    author = "mkusaka",
    about = "super awoesome RPN calc"
)]
struct Ops {
    // sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Ops::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}
