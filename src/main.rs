extern crate clap;

use std::env;
use std::process::exit;
use clap::{ Arg, App, SubCommand };

mod base;
mod add;
mod show;

use base::{ filepath };

fn main() {
    let matches = App::new("Budget")
                      .version("001.0")
                      .author("Stuart <shterrett@gmail.com>")
                      .about("tracks value of single account")
                      .arg(Arg::with_name("file")
                           .help("alternate file")
                           .short("f")
                           .long("file")
                           .takes_value(true))
                      .subcommand(SubCommand::with_name("add")
                                  .about("add an entry")
                                  .arg(Arg::with_name("date")
                                       .help("yyyy-mm-dd")
                                       .index(1)
                                       .required(true))
                                  .arg(Arg::with_name("amount")
                                       .help("float")
                                       .index(2)
                                       .required(true)))
                      .subcommand(SubCommand::with_name("show")
                                  .about("show differences")
                                  .arg(Arg::with_name("num")
                                       .help("number of recent entries")
                                       .short("n")
                                       .long("number")
                                       .takes_value(true)))
                      .get_matches();

    let mut exit_code = 0;

    let data_path = filepath(&matches, env::home_dir());
    if !data_path.is_some() {
        println!("Could not find file path");
        exit(1);
    }

    match matches.subcommand_name() {
        Some("add") => {
           let success = add::run_add(&data_path.unwrap(), &matches);
           if !success { exit_code = 1 };
        },
        Some("show") => {
            let success = show::run_show(&data_path.unwrap(), &matches);
            if !success.is_ok() { exit_code = 2 };
        }
        Some(other) => { println!("Other subcommand {}", other); }
        None => { println!("No subcommand") }
    }

    exit(exit_code);
}
