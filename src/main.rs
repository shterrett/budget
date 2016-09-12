extern crate clap;
extern crate time;

use clap::{ Arg, App };
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let matches = App::new("Budget")
                      .version("001.0")
                      .author("Stuart <shterrett@gmail.com>")
                      .about("tracks value of single account")
                      .arg(Arg::with_name("add")
                               .short("a")
                               .long("add")
                               .help("add a new entry"))
                      .get_matches();

    match env::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }

    if matches.occurrences_of("add") > 0 {
        println!("Now adding ... ");
    }

    println!("Done!");
}

fn write_to_file(entry: &Entry, file_path: &Path) -> bool {
    match OpenOptions::new()
                      .append(true)
                      .create(true)
                      .open(file_path) {
        Ok(mut f) => {
            match f.write_all(entry.format_for_write().as_bytes()) {
                Ok(_) => true,
                Err(_) => false
            }
        }
        Err(_) => {
            false
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Validation {
    Valid,
    DateParseError,
    AmountParseError
}

struct Entry<'a> {
    date_string: &'a str,
    amount_string: &'a str
}

impl<'a> Entry<'a> {
    fn validate(&self) -> Validation {
        if !self.valid_date() {
            return Validation::DateParseError
        }
        if !self.valid_amount() {
            return Validation::AmountParseError
        }
        return Validation::Valid
    }

    fn valid_date(&self) -> bool {
        match time::strptime(self.date_string, "%Y-%m-%d") {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn valid_amount(&self) -> bool {
        match f64::from_str(self.amount_string) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn format_for_write(&self) -> String {
        self.date_string.to_string() + "|" + self.amount_string
    }
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::io::Write;
    use std::path::Path;
    use super::{ Entry,
                 Validation,
                 write_to_file
               };

    #[test]
    fn validate_date_string() {
        let valid_date = "2016-09-01";
        let invalid_date = "9/1/16";
        let valid_amount = "1000";

        let valid_entry = Entry { date_string: valid_date,
                                  amount_string: valid_amount
                                };

        let invalid_entry = Entry { date_string: invalid_date,
                                    amount_string: valid_amount
                                  };

        assert_eq!(valid_entry.validate(), Validation::Valid);
        assert_eq!(invalid_entry.validate(), Validation::DateParseError);
    }

    #[test]
    fn validate_amount_string() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";
        let invalid_amount = "hello";

        let valid_entry = Entry { date_string: valid_date,
                                  amount_string: valid_amount
                                };

        let invalid_entry = Entry { date_string: valid_date,
                                    amount_string: invalid_amount
                                  };

        assert_eq!(valid_entry.validate(), Validation::Valid);
        assert_eq!(invalid_entry.validate(), Validation::AmountParseError);
    }

    #[test]
    fn write_entry_to_file() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";

        let valid_entry = Entry { date_string: valid_date,
                                  amount_string: valid_amount
                                };
        let test_file = Path::new("./test_data/existing_data");
        let existing_lines = vec!["2016-01-01|1000.00",
                                  "2016-02-01|2000.00"];

        let mut new_lines = existing_lines.clone();
        new_lines.push("2016-09-01|1000");
        {
            let f = OpenOptions::new()
                                .read(true)
                                .open(test_file)
                                .unwrap();
            let reader = BufReader::new(f);
            assert_eq!(reader.lines()
                             .map(|l| l.unwrap_or("".to_string()))
                             .collect::<Vec<String>>(),
                       existing_lines);
        }

        write_to_file(&valid_entry, &test_file);
        {
            let f = OpenOptions::new()
                                .read(true)
                                .open(test_file)
                                .unwrap();
            let reader = BufReader::new(f);
            assert_eq!(reader.lines()
                             .map(|l| l.unwrap_or("".to_string()))
                             .collect::<Vec<String>>(),
                       new_lines);
        }


        let mut f = OpenOptions::new()
                                .write(true)
                                .truncate(true)
                                .open(test_file)
                                .unwrap();

        f.write_all(existing_lines.iter()
                                  .fold("".to_string(),
                                        |s, l| s.to_string() + l + "\n",
                                       ).as_bytes());
    }
}
