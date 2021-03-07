#![feature(wrapping_int_impl)]
use clap::{Arg, App, SubCommand, ArgMatches};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::num::Wrapping;


fn is_int(v: String) -> Result<(), String> {
    match v.parse::<u32>() {
        Ok(_) => Ok(()),
        _  => Err(String::from("The value is not an integer"))
    }
}

fn evaluate_entropy(l:u32,symbol_sets:f64) -> u32 {
    ((l as f64)*symbol_sets.log2()) as u32
}

fn entropy_to_combination(number_bits : &u32) -> u64 {
    //u64::pow(2, *number_bits)
    1<< *number_bits
}

fn combination_to_time(combination : u64) -> f64{
    let time = 60f64;
    let guess = 1000f64;
    let day  = 24f64;
    (combination as f64) / guess/time/time/day
}

fn call_entropy(matches : &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("entropy") {
        let password_length = matches.value_of("length").unwrap().parse::<u32>().unwrap();
        let symbol_sets = matches.value_of("evaluate").unwrap();

        // Use the struct like normal
        let password_entropy = match matches.value_of("evaluate") {
            Some("NUMBERS")=> evaluate_entropy(password_length,10.0),
            Some("HEXADECIMAL")=> evaluate_entropy(password_length,62.0),
            Some("ASCII")=> evaluate_entropy(password_length,94.0),
            _ => 0,
        };
        let password_combination = entropy_to_combination(&password_entropy);
        let time_to_find = combination_to_time(password_combination);
        println!("\n\nEvaluate entropy, password length : {}, symbol sets : {}, \n entropy : {} \n number of combination: {}\n time to find with 1000 guess/s : {} day(s)", 
                password_length,
                symbol_sets,
                password_entropy as u32, 
                password_combination,
                time_to_find);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_critical (password : &str, critical_passwords: &Vec<String>) {
    if critical_passwords.contains(&password.to_string()) {
        print!("\n\t /!\\ CRITICAL PASSWORD /!\\ \n\n");
    } else {
        print!("\n\t not critical\n\n");
    }
}

fn populate_criticals() -> Vec<String> {
    let mut critical_passwords = Vec::new();
    if let Ok(lines) = read_lines("./asset/mostCommonCredentials.txt") {
        for line in lines {
            if let Ok(ip) = line {
                critical_passwords.push(ip);
            }
        }
    }
    critical_passwords.to_vec()
}

fn call_critical(matches : &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("critical") {
        let password = matches.value_of("password").unwrap();
        let critical_passwords = populate_criticals();
        is_critical(&password, &critical_passwords);
    }
}

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
                                           .validator(is_int)))
                           .subcommand(SubCommand::with_name("critical")
                                     .about("Check if it's a criticial password")
                                     .version("1.0")
                                     .author("brunjustin@eisti.eu")
                                     .arg(Arg::with_name("password")
                                          .short("p")
                                          .long("password")
                                          .help("Set the password to check")
                                          .required(true)
                                          .takes_value(true)))
                          .get_matches();

    call_entropy(&matches);
    call_critical(&matches);
}