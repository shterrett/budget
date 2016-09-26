extern crate clap;

use std::env;
use std::process::exit;
use clap::{ Arg, App, SubCommand };

mod base;
mod add;
mod show;

use base::{ filepath, Error };

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

    let data_path = filepath(&matches, env::home_dir());
    if !data_path.is_some() {
        println!("Could not find file path");
        exit(1);
    }

    let result = filepath(&matches, env::home_dir()).ok_or(Error::InputError)
                                                    .and_then(|data_path| {
        match matches.subcommand_name() {
             Some("add") => {
                 add::run_add(&data_path, &matches)
             },
             Some("show") => {
                 show::run_show(&data_path, &matches)
             },
             _ => { Err(Error::InputError) }
         }});

    exit(exit_code(result));
}

fn exit_code(status: Result<bool, Error>) -> i32 {
    match status {
        Ok(true) => 0,
        Ok(false) => 1,
        Err(Error::InputError) => 2,
        Err(Error::ReadError) => 3,
        Err(Error::WriteError) => 4
    }
}
