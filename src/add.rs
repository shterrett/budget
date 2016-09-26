extern crate clap;

use std::fs::OpenOptions;
use std::io::{ Write, Error as ioError };
use std::path::Path;
use clap::ArgMatches;

use base::{ Entry, Validation, Error };

pub fn run_add(data_path: &Path, matches: &ArgMatches) -> Result<bool, Error> {
    matches.subcommand_matches("add")
           .ok_or(Error::InputError)
           .and_then(|submatches| build_entry(submatches))
           .and_then(|entry| {
            match entry.validate() {
                Validation::Valid => write_to_file(&entry, data_path).map_err(|_| Error::InputError),
                Validation::DateParseError => {
                    println!("Invalid Date {}; must format as yyyy-mm-dd", entry.date_string);
                    Err(Error::InputError)
                },
                Validation::AmountParseError => {
                    println!("Invalid Amount {}; must be a float", entry.amount_string);
                    Err(Error::InputError)
                }
            }
        })
}

fn build_entry(submatches: &ArgMatches) -> Result<Entry, Error> {
    submatches.value_of("date")
              .and_then(|date| {
                  submatches.value_of("amount")
                            .map(|amount| Entry::new(date, amount))
              })
              .ok_or(Error::InputError)
}

fn write_to_file(entry: &Entry, file_path: &Path) -> Result<bool, ioError> {
    OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)
                .and_then(|mut f| f.write_all(format!("{}", entry).as_bytes()))
                .map(|_| true)
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;
    use std::io::{ Write, BufRead, BufReader };
    use std::path::Path;
    use base::Entry;
    use super::{ write_to_file };

    #[test]
    fn write_entry_to_file() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";

        let valid_entry = Entry::new(valid_date, valid_amount);
        let test_file = Path::new("./test_data/write_entry_to_file");
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

        let res = write_to_file(&valid_entry, &test_file);
        assert!(res.is_ok());
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

        let cleanup = f.write_all(existing_lines.iter()
                                                .fold("".to_string(),
                                                       |s, l| s.to_string() + l + "\n",
                                                     ).as_bytes());
        let sync = f.sync_all();
        assert!(cleanup.is_ok());
        assert!(sync.is_ok());
    }
}
