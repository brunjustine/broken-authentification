#[macro_use]

use clap::{Arg, App, SubCommand, arg_enum};

fn to_int(v: String) -> Result<(), String> {
    match v.parse::<u32>() {
        Ok(t) => Ok(()),
        _  => Err(String::from("The value is not an integer"))
    }
}

/*
fn evaluate_entropy(n:u32,symbol_sets:String) -> f32 {
}
*/

fn main() {
    let matches = App::new("CLI_broken_authentification")
                          .version("1.0")
                          .author("Justine Brun <brunjustin@eisti.eu>")
                          .about("Evaluates the entropy of a password")
                          .arg(Arg::with_name("debug")
                            .short("d"))
                          .subcommand(SubCommand::with_name("entropy")
                                      .about("Evaluates the entropy of a password")
                                      .version("1.0")
                                      .author("brunjustin@eisti.eu")
                                      .arg(Arg::with_name("evaluate")
                                           .short("e")
                                           .long("eval")
                                           .help("Sets symbol sets to use to evaluate the entropy of a password")
                                           .required(true)
                                           .takes_value(true)
                                           .possible_values(&["NUMBERS", "HEXADECIMAL","ASCII"]))
                                        .arg(Arg::with_name("length")
                                           .short("l")
                                           .long("length")
                                           .help("Sets the length of the password")
                                           .takes_value(true)
                                           .required(true)
                                           .validator(to_int)))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("brunjustin@eisti.eu")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                          .get_matches();
 
    if let Some(matches) = matches.subcommand_matches("entropy") {
        // Use the struct like normal
        match matches.value_of("evaluate") {
        Some("NUMBERS")=>println!("Some numbers"),
        Some("HEXADECIMAL")=>println!("Some hexadecimal"),
        Some("ASCII")=>println!("Some ascii"),
        _ => println!("Don't be crazy"),
        }
    }
}