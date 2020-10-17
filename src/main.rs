use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN program")
        .version("0.0.1")
        .author("your name")
        .about("Super awesome sample RPN calc")
        .arg(
            Arg::new("formula_file")
                .about("formulars written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("no file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
