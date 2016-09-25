extern crate clap;
extern crate time;

use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{ Write, BufRead, BufReader };
use std::path::{ Path, PathBuf };
use std::process::exit;
use std::str::FromStr;
use clap::{ Arg, App, SubCommand, ArgMatches };

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
           let success = run_add(&data_path.unwrap(), &matches);
           if !success { exit_code = 1 };
        },
        Some("show") => {
            let success = run_show(&data_path.unwrap(), &matches);
            if !success { exit_code = 2 };
        }
        Some(other) => { println!("Other subcommand {}", other); }
        None => { println!("No subcommand") }
    }

    exit(exit_code);
}

fn filepath(matches: &ArgMatches, default_path: Option<PathBuf>) -> Option<PathBuf> {
    match matches.value_of("file") {
        Some(path) => Some(PathBuf::from(path)),
        None => {
            match default_path {
                Some(path) => Some(path.join(".budget")),
                None => None
            }
        }
    }
}

fn run_add(data_path: &Path, matches: &ArgMatches) -> bool {
    match matches.subcommand_matches("add") {
        Some(submatches) => {
            let entry = Entry::new(submatches.value_of("date").unwrap(),
                                   submatches.value_of("amount").unwrap()
                                  );
            match entry.validate() {
                Validation::Valid => write_to_file(&entry, data_path),
                Validation::DateParseError => {
                    println!("Invalid Date {}; must format as yyyy-mm-dd", entry.date_string);
                    false
                },
                Validation::AmountParseError => {
                    println!("Invalid Amount {}; must be a float", entry.amount_string);
                    false
                }
            }
        }
        None => false
    }
}

fn run_show(data_path: &Path, matches: &ArgMatches) -> bool {
    match matches.subcommand_matches("show") {
        Some(submatches) => {
            match read_file(data_path) {
                Some(entries) => {
                    for delta in delta_by_line(filter_entries(&entries, &submatches)) {
                        println!("{}", delta);
                    }
                    true
                }
                None => false
            }
        }
        None => false
    }
}

fn write_to_file(entry: &Entry, file_path: &Path) -> bool {
    match OpenOptions::new()
                      .append(true)
                      .create(true)
                      .open(file_path) {
        Ok(mut f) => {
            match f.write_all(format!("{}", entry).as_bytes()) {
                Ok(_) => true,
                Err(_) => false
            }
        }
        Err(_) => {
            false
        }
    }
}

fn read_file(file_path: &Path) -> Option<Vec<Entry>> {
    match OpenOptions::new()
                      .read(true)
                      .open(file_path) {
        Ok(f) => {
            let reader = BufReader::new(f);
            Some(reader.lines()
                       .filter_map(|l|
                            match l {
                                Ok(line) => Some(Entry::from_line(line)),
                                Err(_) => None
                            }
                        )
                       .collect::<Vec<Entry>>())
        }
        Err(msg) => None
    }
}

fn filter_entries<'a>(entries: &'a [Entry], submatches: &ArgMatches) -> &'a [Entry] {
    if let Some(interval) = submatches.value_of("num") {
        match usize::from_str(interval) {
            Ok(n) => {
                entries.split_at(entries.len().checked_sub(n).unwrap()).1
            }
            Err(_) => {
                entries
            }
        }
    } else {
        entries
    }
}

fn delta_by_line<'a>(entries: &'a [Entry]) -> Vec<Delta<'a>> {
    entries.windows(2)
           .map(|es| Delta::new(&es[0], &es[1]))
           .collect::<Vec<Delta<'a>>>()

}

#[derive(PartialEq, Eq, Debug)]
enum Validation {
    Valid,
    DateParseError,
    AmountParseError
}

#[derive(PartialEq, Eq, Debug)]
struct Entry {
    date_string: String,
    amount_string: String
}

impl Entry {
    fn new<S>(date_string: S, amount_string: S) -> Self
        where S: Into<String> {
        Entry { date_string: date_string.into(),
                amount_string: amount_string.into()
              }
    }

    fn from_line<S>(line: S) -> Self
        where S: Into<String> {
        let line_str = line.into();
        let strs = line_str.split("|")
                           .into_iter()
                           .collect::<Vec<&str>>();
        Entry::new(strs[0], strs[1])
    }

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
        match time::strptime(&self.date_string, "%Y-%m-%d") {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn valid_amount(&self) -> bool {
        match f64::from_str(&self.amount_string) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn date(&self) -> time::Tm {
        time::strptime(&self.date_string, "%Y-%m-%d").unwrap()
    }

    fn amount(&self) -> f64 {
        f64::from_str(&self.amount_string).unwrap()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}\n", self.date_string, self.amount_string)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Delta<'a> {
    start: &'a Entry,
    end: &'a Entry
}

impl<'a> Delta<'a> {
    fn new(start: &'a Entry, end: &'a Entry) -> Self {
        Delta { start: start, end: end }
    }

    fn delta(&self) -> f64 {
        self.end.amount() - self.start.amount()
    }
}

impl<'a> fmt::Display for Delta<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}: {} -> {} | {}", self.start.date_string,
                                             self.end.date_string,
                                             self.start.amount_string,
                                             self.end.amount_string,
                                             self.delta())
    }
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;
    use std::io::{ Write, BufRead, BufReader };
    use std::path::{ Path, PathBuf };
    use clap::{ Arg, App };
    use super::{ Entry,
                 Delta,
                 Validation,
                 write_to_file,
                 read_file,
                 filepath,
                 delta_by_line,
                 filter_entries
               };

    #[test]
    fn validate_date_string() {
        let valid_date = "2016-09-01";
        let invalid_date = "9/1/16";
        let valid_amount = "1000";

        let valid_entry = Entry::new(valid_date, valid_amount);

        let invalid_entry = Entry::new(invalid_date, valid_amount);

        assert_eq!(valid_entry.validate(), Validation::Valid);
        assert_eq!(invalid_entry.validate(), Validation::DateParseError);
    }

    #[test]
    fn validate_amount_string() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";
        let invalid_amount = "hello";

        let valid_entry = Entry::new(valid_date, valid_amount);

        let invalid_entry = Entry::new(valid_date, invalid_amount);

        assert_eq!(valid_entry.validate(), Validation::Valid);
        assert_eq!(invalid_entry.validate(), Validation::AmountParseError);
    }

    #[test]
    fn entry_formats_with_newline() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";

        let valid_entry = Entry::new(valid_date, valid_amount);

        assert_eq!(format!("{}", valid_entry), "2016-09-01|1000\n");
    }

    #[test]
    fn entry_from_line() {
        let line = "2016-01-01|1000.00".to_string();
        let entry = Entry::from_line(line);

        assert_eq!(entry.validate(), Validation::Valid);
        assert_eq!(entry.date_string, "2016-01-01");
        assert_eq!(entry.amount_string, "1000.00");
    }

    #[test]
    fn builds_entries_from_file() {
        // 2016-01-01|1000.00
        // 2016-02-01|2000.00
        let test_file = Path::new("./test_data/existing_data");
        let entries = read_file(&test_file).unwrap();

        let expected = vec![Entry::new("2016-01-01", "1000.00"),
                            Entry::new("2016-02-01", "2000.00")
                           ];

        assert_eq!(entries, expected);
    }

    #[test]
    fn returns_the_last_n_entries() {
        let entries = vec![Entry::new("2016-09-01", "1000"),
                           Entry::new("2016-10-01", "1200"),
                           Entry::new("2016-11-01", "1100"),
                           Entry::new("2016-12-01", "1300")
                          ];

        let matches = App::new("test")
                          .arg(Arg::with_name("num")
                                   .short("n")
                                   .long("number")
                                   .takes_value(true))
                          .get_matches_from(vec!["test", "-n", "2"]);

        let filtered = filter_entries(&entries, &matches);

        assert_eq!(filtered, entries.split_at(2).1);
    }

    #[test]
    fn delta_calculates_difference_between_entries() {
        let entry_1 = Entry::new("2016-10-01", "1200");
        let entry_2 = Entry::new("2016-11-01", "1100");

        let delta = Delta::new(&entry_1, &entry_2);

        assert_eq!(delta.delta(), -100.0)
    }

    #[test]
    fn delta_formats_for_display() {
        let e1 = Entry::new("2016-01-01", "1000.00");
        let e2 = Entry::new("2016-02-01", "2000.00");
        let delta = Delta::new(&e1, &e2);

        assert_eq!(format!("{}", delta),
                   "2016-01-01 -> 2016-02-01: 1000.00 -> 2000.00 | 1000");
    }

    #[test]
    fn returns_differences_for_each_entry() {
        let entries = vec![Entry::new("2016-09-01", "1000"),
                           Entry::new("2016-10-01", "1200"),
                           Entry::new("2016-11-01", "1100"),
                           Entry::new("2016-12-01", "1300")
                          ];

        let differences = delta_by_line(&entries).iter()
                                                 .map(|d| d.delta())
                                                 .collect::<Vec<f64>>();

        assert_eq!(differences, vec![200.0, -100.0, 200.0]);
    }

    #[test]
    fn returns_default_filepath() {
        let matches = App::new("test")
                          .arg(Arg::with_name("file")
                                   .short("f")
                                   .long("file")
                                   .takes_value(true))
                          .get_matches_from(vec!["test"]);

        let default_path = PathBuf::from("/home");
        let path = filepath(&matches, Some(default_path)).unwrap();

        assert_eq!(path, PathBuf::from("/home/.budget"));
    }

    #[test]
    fn returns_provided_filepath() {
        let matches = App::new("test")
                          .arg(Arg::with_name("file")
                                   .short("f")
                                   .long("file")
                                   .takes_value(true))
                          .get_matches_from(vec!["test", "-f", "/var/budget"]);

        let default_path = PathBuf::from("/home");
        let path = filepath(&matches, Some(default_path)).unwrap();

        assert_eq!(path, PathBuf::from("/var/budget"));
    }

    #[test]
    fn write_entry_to_file() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";

        let valid_entry = Entry::new(valid_date, valid_amount);
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

        let cleanup = f.write_all(existing_lines.iter()
                                                .fold("".to_string(),
                                                       |s, l| s.to_string() + l + "\n",
                                                     ).as_bytes());
        if cleanup.is_err() {
            println!("cleanup failed");
        }
    }
}
